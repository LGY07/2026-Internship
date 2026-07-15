use crate::AppState;
use crate::types::{Book, BookCategory, Category, Record, Renew, RenewStatus, Token, User};
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum::Json;
use axum::extract::{FromRequestParts, Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{Router, delete, get, post, put};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use tracing::error;
use uuid::Uuid;

pub fn route() -> Router<AppState> {
    Router::new()
        // 用户认证
        .route("/register", post(register))
        .route("/login", post(login))
        // 查询书籍
        .route("/books/search", get(search_books))
        .route("/books", get(list_books).post(add_book))
        .route("/books/{id}", delete(delete_book))
        // 用户管理
        .route("/users/me/borrow", get(my_borrow_records))
        .route("/users/me", get(my_info))
        .route("/users", get(list_users))
        .route("/users/{id}/borrow", get(user_borrow_records))
        .route("/users/{id}/password", put(change_user_password))
        .route(
            "/users/{id}",
            get(search_users)
                .patch(update_user_info)
                .delete(delete_user),
        )
        // 续借（特定路由在前）
        .route("/books/borrow/expired", get(expired_borrow_records))
        .route("/books/borrow/renewals/{id}/approve", post(approve_renewal))
        .route("/books/borrow/renewals", get(list_renewals))
        .route("/books/borrow/{id}/renew", post(submit_renewal))
        .route("/books/borrow", get(all_borrow_records))
        // 书籍借阅
        .route(
            "/books/{id}/borrow",
            get(book_borrow_records).post(borrow_book),
        )
        .route("/books/{id}/return", post(return_book))
}

// ==================== 鉴权提取器 ====================

struct Claims {
    user_id: usize,
    permission: String,
}

impl Claims {
    fn is_manager(&self) -> bool {
        self.permission == "manager"
    }
}

impl FromRequestParts<AppState> for Claims {
    type Rejection = (StatusCode, Json<ErrorResponse>);

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok());

        let token_str = auth_header.and_then(|h| h.strip_prefix("Bearer "));
        let token_uuid = token_str.and_then(|s| Uuid::parse_str(s).ok());

        let state = state.clone();

        let token_uuid = token_uuid.ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    success: false,
                    message: "缺少认证令牌",
                }),
            )
        })?;

        let mut db = state.db.db().clone();

        let mut token_record = Token::filter_by_token(&token_uuid)
            .first()
            .exec(&mut db)
            .await
            .map_err(|e| {
                error!("{e}");
                (
                    StatusCode::UNAUTHORIZED,
                    Json(ErrorResponse {
                        success: false,
                        message: "令牌无效",
                    }),
                )
            })?
            .ok_or_else(|| {
                (
                    StatusCode::UNAUTHORIZED,
                    Json(ErrorResponse {
                        success: false,
                        message: "令牌无效",
                    }),
                )
            })?;

        let now = jiff::Timestamp::now();
        if token_record.expired_date < now {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    success: false,
                    message: "令牌已过期",
                }),
            ));
        }

        // 滑动续期：剩余不足 1 小时则续期 12 小时（会话级）
        // 客户端每次启动重新登录，token 只需撑住单次会话
        let total = std::time::Duration::from_secs(12 * 3600);
        let threshold = std::time::Duration::from_secs(3600);
        if token_record.expired_date < now + (total - threshold) {
            let new_expiry = now + total;
            if let Err(e) = token_record
                .update()
                .expired_date(new_expiry)
                .exec(&mut db)
                .await
            {
                error!("令牌续期失败: {e}");
            }
        }

        let user = User::filter_by_id(&token_record.user_id)
            .first()
            .exec(&mut db)
            .await
            .map_err(|e| {
                error!("{e}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        success: false,
                        message: "服务器内部错误",
                    }),
                )
            })?
            .ok_or_else(|| {
                (
                    StatusCode::UNAUTHORIZED,
                    Json(ErrorResponse {
                        success: false,
                        message: "用户不存在",
                    }),
                )
            })?;

        Ok(Claims {
            user_id: user.id,
            permission: user.permission,
        })
    }
}

// ==================== 通用响应类型 ====================

#[derive(Serialize)]
struct ErrorResponse {
    success: bool,
    message: &'static str,
}

#[derive(Serialize)]
struct LoginResponse {
    success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permissions: Option<String>,
    message: &'static str,
}

#[derive(Serialize)]
struct BookInfo {
    id: usize,
    title: String,
    author: String,
    category: Vec<String>,
    description: Option<String>,
    published_date: Option<String>,
    borrowed: bool,
    expire_date: Option<String>,
}

#[derive(Serialize)]
struct BooksResponse {
    success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    books: Option<Vec<BookInfo>>,
    message: &'static str,
}

#[derive(Serialize)]
struct UserInfo {
    id: usize,
    username: String,
    phone: Option<String>,
    email: Option<String>,
    borrowed_books: usize,
}

#[derive(Serialize)]
struct MyInfoResponse {
    success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<UserInfo>,
    message: &'static str,
}

#[derive(Serialize)]
struct UsersResponse {
    success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    users: Option<Vec<UserInfo>>,
    message: &'static str,
}

#[derive(Serialize)]
struct BorrowRecordInfo {
    id: usize,
    user_id: usize,
    book_id: usize,
    borrow_date: String,
    expire_date: String,
    return_date: Option<String>,
}

#[derive(Serialize)]
struct BorrowRecordsResponse {
    success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    borrow_records: Option<Vec<BorrowRecordInfo>>,
    message: &'static str,
}

#[derive(Serialize)]
struct RenewalInfo {
    id: usize,
    user_id: usize,
    book_id: usize,
    request_date: String,
    expired_after: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<String>,
}

#[derive(Serialize)]
struct RenewalsResponse {
    success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    renewals: Option<Vec<RenewalInfo>>,
    message: &'static str,
}

// ==================== 辅助函数 ====================

fn format_date(ts: &jiff::Timestamp) -> String {
    ts.strftime("%Y-%m-%d").to_string()
}

fn parse_date(s: &str) -> Result<jiff::Timestamp, &'static str> {
    jiff::civil::Date::from_str(s)
        .map(|d| d.to_datetime(jiff::civil::Time::midnight()))
        .and_then(|dt| dt.to_zoned(jiff::tz::TimeZone::UTC))
        .map(|z| z.timestamp())
        .map_err(|_| "日期格式错误，应为 YYYY-MM-DD")
}

fn validate_username(username: &str) -> bool {
    let len = username.len();
    if !(3..=32).contains(&len) {
        return false;
    }
    username
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn validate_password(password: &str) -> bool {
    if !(8..=64).contains(&password.len()) {
        return false;
    }
    let has_letter = password.chars().any(|c| c.is_ascii_alphabetic());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    has_letter && has_digit
}

async fn load_book_categories(book_id: usize, db: &mut toasty::Db) -> Result<Vec<String>, ()> {
    let book_cats = BookCategory::filter_by_book_id(&book_id)
        .exec(db)
        .await
        .map_err(|e| error!("{e}"))?;
    let mut categories = Vec::new();
    for bc in &book_cats {
        if let Ok(Some(cat)) = Category::filter_by_id(&bc.category_id)
            .first()
            .exec(db)
            .await
        {
            categories.push(cat.name.clone());
        }
    }
    Ok(categories)
}

async fn book_to_info(book: &Book, db: &mut toasty::Db) -> Result<BookInfo, ()> {
    let categories = load_book_categories(book.id, db).await?;
    let (borrowed, expire_date) = {
        let records = Record::filter_by_book_id(&book.id)
            .exec(db)
            .await
            .map_err(|e| error!("{e}"))?;
        let active = records.iter().find(|r| r.return_date.is_none());
        match active {
            Some(r) => (true, Some(format_date(&r.expire_date))),
            None => (false, None),
        }
    };
    Ok(BookInfo {
        id: book.id,
        title: book.title.clone(),
        author: book.author.clone(),
        category: categories,
        description: book.description.clone(),
        published_date: book.published_date.as_ref().map(format_date),
        borrowed,
        expire_date,
    })
}

fn record_to_info(record: &Record) -> BorrowRecordInfo {
    BorrowRecordInfo {
        id: record.id,
        user_id: record.user_id,
        book_id: record.book_id,
        borrow_date: format_date(&record.borrow_date),
        expire_date: format_date(&record.expire_date),
        return_date: record.return_date.as_ref().map(format_date),
    }
}

async fn count_user_borrows(user_id: usize, db: &mut toasty::Db) -> Result<usize, ()> {
    let records = Record::filter_by_user_id(&user_id)
        .exec(db)
        .await
        .map_err(|e| error!("{e}"))?;
    Ok(records.iter().filter(|r| r.return_date.is_none()).count())
}

fn to_error(status: StatusCode, msg: &'static str) -> (StatusCode, Json<ErrorResponse>) {
    (
        status,
        Json(ErrorResponse {
            success: false,
            message: msg,
        }),
    )
}

// ==================== 用户认证 ====================

#[derive(Deserialize)]
struct RegisterRequest {
    username: String,
    password: String,
}

async fn register(
    State(state): State<AppState>,
    Json(body): Json<RegisterRequest>,
) -> impl IntoResponse {
    if !validate_username(&body.username) {
        return to_error(
            StatusCode::BAD_REQUEST,
            "用户名格式错误： 长度应为 3 至 32 位，且只包含字母、数字和下划线",
        );
    }
    if !validate_password(&body.password) {
        return to_error(
            StatusCode::BAD_REQUEST,
            "密码格式错误： 长度应为 8 至 64 位，且必须包含字母和数字",
        );
    }
    let mut db = state.db.db().clone();
    let count = match User::filter_by_username(&body.username)
        .count()
        .exec(&mut db)
        .await
    {
        Ok(c) => c,
        Err(e) => {
            error!("{e}");
            return to_error(StatusCode::INTERNAL_SERVER_ERROR, "服务器内部错误");
        }
    };
    if count > 0 {
        return to_error(StatusCode::CONFLICT, "用户名已存在");
    }
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = argon2
        .hash_password(body.password.as_bytes(), &salt)
        .expect("Argon2 error")
        .to_string();
    if let Err(e) = User::create()
        .username(body.username)
        .password(hashed_password)
        .permission("guest".to_string())
        .phone(None)
        .email(None)
        .token(None)
        .register_date(jiff::Timestamp::now())
        .exec(&mut db)
        .await
    {
        error!("{e}");
        return to_error(StatusCode::INTERNAL_SERVER_ERROR, "创建用户失败");
    }
    (
        StatusCode::OK,
        Json(ErrorResponse {
            success: true,
            message: "注册成功",
        }),
    )
}

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

async fn login(State(state): State<AppState>, Json(body): Json<LoginRequest>) -> impl IntoResponse {
    let mut db = state.db.db().clone();
    let user = match User::filter_by_username(&body.username)
        .first()
        .exec(&mut db)
        .await
    {
        Ok(Some(u)) => u,
        _ => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(LoginResponse {
                    success: false,
                    access_token: None,
                    permissions: None,
                    message: "用户名或密码错误",
                }),
            );
        }
    };
    let parsed_hash = match PasswordHash::new(&user.password) {
        Ok(h) => h,
        Err(e) => {
            error!("{e}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(LoginResponse {
                    success: false,
                    access_token: None,
                    permissions: None,
                    message: "服务器内部错误",
                }),
            );
        }
    };
    if Argon2::default()
        .verify_password(body.password.as_bytes(), &parsed_hash)
        .is_err()
    {
        return (
            StatusCode::UNAUTHORIZED,
            Json(LoginResponse {
                success: false,
                access_token: None,
                permissions: None,
                message: "用户名或密码错误",
            }),
        );
    }
    let token_uuid = Uuid::now_v7();
    let now = jiff::Timestamp::now();
    let expired_date = now + std::time::Duration::from_secs(12 * 3600);
    if let Err(e) = Token::create()
        .token(token_uuid)
        .expired_date(expired_date)
        .user_id(user.id)
        .exec(&mut db)
        .await
    {
        error!("{e}");
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(LoginResponse {
                success: false,
                access_token: None,
                permissions: None,
                message: "服务器内部错误",
            }),
        );
    }
    (
        StatusCode::OK,
        Json(LoginResponse {
            success: true,
            access_token: Some(token_uuid.to_string()),
            permissions: Some(user.permission.clone()),
            message: "登录成功",
        }),
    )
}

// ==================== 查询书籍 ====================

#[derive(Deserialize)]
struct PageQuery {
    #[serde(default = "default_page")]
    page: usize,
}
fn default_page() -> usize {
    1
}
const PAGE_SIZE: usize = 20;

async fn list_books(
    _claims: Claims,
    State(state): State<AppState>,
    Query(query): Query<PageQuery>,
) -> impl IntoResponse {
    let mut db = state.db.db().clone();
    let books = match Book::all()
        .limit(PAGE_SIZE)
        .offset((query.page.saturating_sub(1)) * PAGE_SIZE)
        .exec(&mut db)
        .await
    {
        Ok(b) => b,
        Err(e) => {
            error!("{e}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(BooksResponse {
                    success: false,
                    books: None,
                    message: "服务器内部错误",
                }),
            );
        }
    };
    let mut book_infos = Vec::new();
    for book in &books {
        match book_to_info(book, &mut db).await {
            Ok(info) => book_infos.push(info),
            Err(()) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(BooksResponse {
                        success: false,
                        books: None,
                        message: "服务器内部错误",
                    }),
                );
            }
        }
    }
    (
        StatusCode::OK,
        Json(BooksResponse {
            success: true,
            books: Some(book_infos),
            message: "查询成功",
        }),
    )
}

#[derive(Deserialize, Default)]
struct SearchBooksQuery {
    title: Option<String>,
    author: Option<String>,
    category: Option<String>,
}

async fn search_books(
    _claims: Claims,
    State(state): State<AppState>,
    Query(query): Query<SearchBooksQuery>,
) -> impl IntoResponse {
    let mut db = state.db.db().clone();
    let all_books = match Book::all().exec(&mut db).await {
        Ok(b) => b,
        Err(e) => {
            error!("{e}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(BooksResponse {
                    success: false,
                    books: None,
                    message: "服务器内部错误",
                }),
            );
        }
    };
    let filtered: Vec<&Book> = if let Some(ref kw) = query.title {
        let kw = kw.to_lowercase();
        all_books
            .iter()
            .filter(|b| b.title.to_lowercase().contains(&kw))
            .collect()
    } else if let Some(ref kw) = query.author {
        let kw = kw.to_lowercase();
        all_books
            .iter()
            .filter(|b| b.author.to_lowercase().contains(&kw))
            .collect()
    } else if let Some(ref kw) = query.category {
        let kw = kw.to_lowercase();
        let mut result = Vec::new();
        for book in &all_books {
            if let Ok(cats) = load_book_categories(book.id, &mut db).await {
                if cats.iter().any(|c| c.to_lowercase().contains(&kw)) {
                    result.push(book);
                }
            }
        }
        result
    } else {
        all_books.iter().collect()
    };
    let mut book_infos = Vec::new();
    for book in filtered {
        match book_to_info(book, &mut db).await {
            Ok(info) => book_infos.push(info),
            Err(()) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(BooksResponse {
                        success: false,
                        books: None,
                        message: "服务器内部错误",
                    }),
                );
            }
        }
    }
    (
        StatusCode::OK,
        Json(BooksResponse {
            success: true,
            books: Some(book_infos),
            message: "查询成功",
        }),
    )
}

// ==================== 书籍管理 ====================

#[derive(Deserialize)]
struct AddBookRequest {
    title: String,
    author: String,
    category: Vec<String>,
    description: Option<String>,
    published_date: Option<String>,
}

async fn add_book(
    claims: Claims,
    State(state): State<AppState>,
    Json(body): Json<AddBookRequest>,
) -> impl IntoResponse {
    if !claims.is_manager() {
        return to_error(StatusCode::FORBIDDEN, "权限不足");
    }
    let mut db = state.db.db().clone();
    let published_date = match &body.published_date {
        Some(d) => match parse_date(d) {
            Ok(ts) => Some(ts),
            Err(msg) => return to_error(StatusCode::BAD_REQUEST, msg),
        },
        None => None,
    };
    let book = match Book::create()
        .title(body.title)
        .author(body.author)
        .description(body.description)
        .published_date(published_date)
        .exec(&mut db)
        .await
    {
        Ok(b) => b,
        Err(e) => {
            error!("{e}");
            return to_error(StatusCode::INTERNAL_SERVER_ERROR, "添加书籍失败");
        }
    };
    for cat_name in &body.category {
        let category = match Category::filter_by_name(cat_name)
            .first()
            .exec(&mut db)
            .await
        {
            Ok(Some(c)) => c,
            _ => match Category::create()
                .name(cat_name.clone())
                .exec(&mut db)
                .await
            {
                Ok(c) => c,
                Err(e) => {
                    error!("{e}");
                    continue;
                }
            },
        };
        if let Err(e) = BookCategory::create()
            .book_id(book.id)
            .category_id(category.id)
            .exec(&mut db)
            .await
        {
            error!("{e}");
        }
    }
    (
        StatusCode::OK,
        Json(ErrorResponse {
            success: true,
            message: "添加书籍成功",
        }),
    )
}

async fn delete_book(
    claims: Claims,
    State(state): State<AppState>,
    Path(id): Path<usize>,
) -> impl IntoResponse {
    if !claims.is_manager() {
        return to_error(StatusCode::FORBIDDEN, "权限不足");
    }
    let mut db = state.db.db().clone();
    if let Ok(book_cats) = BookCategory::filter_by_book_id(&id).exec(&mut db).await {
        for bc in book_cats {
            let _ = bc.delete().exec(&mut db).await;
        }
    }
    let book = match Book::filter_by_id(&id).first().exec(&mut db).await {
        Ok(Some(b)) => b,
        _ => return to_error(StatusCode::NOT_FOUND, "书籍不存在"),
    };
    if let Err(e) = book.delete().exec(&mut db).await {
        error!("{e}");
        return to_error(StatusCode::INTERNAL_SERVER_ERROR, "删除书籍失败");
    }
    (
        StatusCode::OK,
        Json(ErrorResponse {
            success: true,
            message: "删除书籍成功",
        }),
    )
}

// ==================== 用户管理 ====================

async fn my_info(claims: Claims, State(state): State<AppState>) -> impl IntoResponse {
    let mut db = state.db.db().clone();
    let borrowed_books = count_user_borrows(claims.user_id, &mut db)
        .await
        .unwrap_or(0);
    let user = match User::filter_by_id(&claims.user_id)
        .first()
        .exec(&mut db)
        .await
    {
        Ok(Some(u)) => u,
        Err(e) => {
            error!("{e}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(MyInfoResponse {
                    success: false,
                    user: None,
                    message: "服务器内部错误",
                }),
            );
        }
        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                Json(MyInfoResponse {
                    success: false,
                    user: None,
                    message: "用户不存在",
                }),
            );
        }
    };
    (
        StatusCode::OK,
        Json(MyInfoResponse {
            success: true,
            user: Some(UserInfo {
                id: user.id,
                username: user.username,
                phone: user.phone,
                email: user.email,
                borrowed_books,
            }),
            message: "查询成功",
        }),
    )
}

async fn list_users(claims: Claims, State(state): State<AppState>) -> impl IntoResponse {
    if !claims.is_manager() {
        return (
            StatusCode::FORBIDDEN,
            Json(UsersResponse {
                success: false,
                users: None,
                message: "权限不足",
            }),
        );
    }
    let mut db = state.db.db().clone();
    let users = match User::all().exec(&mut db).await {
        Ok(u) => u,
        Err(e) => {
            error!("{e}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(UsersResponse {
                    success: false,
                    users: None,
                    message: "服务器内部错误",
                }),
            );
        }
    };
    let mut user_infos = Vec::new();
    for user in &users {
        let borrowed_books = count_user_borrows(user.id, &mut db).await.unwrap_or(0);
        user_infos.push(UserInfo {
            id: user.id,
            username: user.username.clone(),
            phone: user.phone.clone(),
            email: user.email.clone(),
            borrowed_books,
        });
    }
    (
        StatusCode::OK,
        Json(UsersResponse {
            success: true,
            users: Some(user_infos),
            message: "查询成功",
        }),
    )
}

async fn search_users(
    claims: Claims,
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    if !claims.is_manager() {
        return (
            StatusCode::FORBIDDEN,
            Json(UsersResponse {
                success: false,
                users: None,
                message: "权限不足",
            }),
        );
    }
    let mut db = state.db.db().clone();
    let all_users = match User::all().exec(&mut db).await {
        Ok(u) => u,
        Err(e) => {
            error!("{e}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(UsersResponse {
                    success: false,
                    users: None,
                    message: "服务器内部错误",
                }),
            );
        }
    };
    let kw = id.to_lowercase();
    let filtered: Vec<_> = all_users
        .iter()
        .filter(|u| u.username.to_lowercase().contains(&kw))
        .collect();
    let mut user_infos = Vec::new();
    for user in filtered {
        let borrowed_books = count_user_borrows(user.id, &mut db).await.unwrap_or(0);
        user_infos.push(UserInfo {
            id: user.id,
            username: user.username.clone(),
            phone: user.phone.clone(),
            email: user.email.clone(),
            borrowed_books,
        });
    }
    (
        StatusCode::OK,
        Json(UsersResponse {
            success: true,
            users: Some(user_infos),
            message: "查询成功",
        }),
    )
}

async fn delete_user(
    claims: Claims,
    State(state): State<AppState>,
    Path(id): Path<usize>,
) -> impl IntoResponse {
    if !claims.is_manager() {
        return to_error(StatusCode::FORBIDDEN, "权限不足");
    }
    let mut db = state.db.db().clone();
    let user = match User::filter_by_id(&id).first().exec(&mut db).await {
        Ok(Some(u)) => u,
        _ => return to_error(StatusCode::NOT_FOUND, "用户不存在"),
    };
    if let Ok(tokens) = Token::filter_by_user_id(&id).exec(&mut db).await {
        for t in tokens {
            let _ = t.delete().exec(&mut db).await;
        }
    }
    if let Err(e) = user.delete().exec(&mut db).await {
        error!("{e}");
        return to_error(StatusCode::INTERNAL_SERVER_ERROR, "删除用户失败");
    }
    (
        StatusCode::OK,
        Json(ErrorResponse {
            success: true,
            message: "删除用户成功",
        }),
    )
}

#[derive(Deserialize)]
struct ChangePasswordRequest {
    password: String,
}

async fn change_user_password(
    claims: Claims,
    State(state): State<AppState>,
    Path(id): Path<usize>,
    Json(body): Json<ChangePasswordRequest>,
) -> impl IntoResponse {
    if !claims.is_manager() && claims.user_id != id {
        return to_error(StatusCode::FORBIDDEN, "权限不足");
    }
    if !validate_password(&body.password) {
        return to_error(
            StatusCode::BAD_REQUEST,
            "密码格式错误： 长度应为 8 至 64 位，且必须包含字母和数字",
        );
    }
    let mut db = state.db.db().clone();
    let mut user = match User::filter_by_id(&id).first().exec(&mut db).await {
        Ok(Some(u)) => u,
        _ => return to_error(StatusCode::NOT_FOUND, "用户不存在"),
    };
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = argon2
        .hash_password(body.password.as_bytes(), &salt)
        .expect("Argon2 error")
        .to_string();
    if let Err(e) = user.update().password(hashed_password).exec(&mut db).await {
        error!("{e}");
        return to_error(StatusCode::INTERNAL_SERVER_ERROR, "修改密码失败");
    }
    (
        StatusCode::OK,
        Json(ErrorResponse {
            success: true,
            message: "修改密码成功",
        }),
    )
}

#[derive(Deserialize)]
struct UpdateUserRequest {
    phone: String,
    email: String,
}

async fn update_user_info(
    claims: Claims,
    State(state): State<AppState>,
    Path(id): Path<usize>,
    Json(body): Json<UpdateUserRequest>,
) -> impl IntoResponse {
    if !claims.is_manager() && claims.user_id != id {
        return to_error(StatusCode::FORBIDDEN, "权限不足");
    }
    let mut db = state.db.db().clone();
    let mut user = match User::filter_by_id(&id).first().exec(&mut db).await {
        Ok(Some(u)) => u,
        _ => return to_error(StatusCode::NOT_FOUND, "用户不存在"),
    };
    if let Err(e) = user
        .update()
        .phone(Some(body.phone))
        .email(Some(body.email))
        .exec(&mut db)
        .await
    {
        error!("{e}");
        return to_error(StatusCode::INTERNAL_SERVER_ERROR, "修改用户信息失败");
    }
    (
        StatusCode::OK,
        Json(ErrorResponse {
            success: true,
            message: "修改用户信息成功",
        }),
    )
}

// ==================== 书籍借阅 ====================

#[derive(Deserialize)]
struct BorrowBookRequest {
    user_id: usize,
    expire_date: String,
}

async fn borrow_book(
    claims: Claims,
    State(state): State<AppState>,
    Path(book_id): Path<usize>,
    Json(body): Json<BorrowBookRequest>,
) -> impl IntoResponse {
    if !claims.is_manager() {
        return to_error(StatusCode::FORBIDDEN, "权限不足");
    }
    let expire_date = match parse_date(&body.expire_date) {
        Ok(d) => d,
        Err(msg) => return to_error(StatusCode::BAD_REQUEST, msg),
    };
    let mut db = state.db.db().clone();
    let _book = match Book::filter_by_id(&book_id).first().exec(&mut db).await {
        Ok(Some(b)) => b,
        _ => return to_error(StatusCode::NOT_FOUND, "书籍不存在"),
    };
    let records = match Record::filter_by_book_id(&book_id).exec(&mut db).await {
        Ok(r) => r,
        Err(e) => {
            error!("{e}");
            return to_error(StatusCode::INTERNAL_SERVER_ERROR, "服务器内部错误");
        }
    };
    if records.iter().any(|r| r.return_date.is_none()) {
        return to_error(StatusCode::CONFLICT, "该书籍已被借出");
    }
    let now = jiff::Timestamp::now();
    if let Err(e) = Record::create()
        .user_id(body.user_id)
        .book_id(book_id)
        .borrow_date(now)
        .expire_date(expire_date)
        .return_date(None)
        .exec(&mut db)
        .await
    {
        error!("{e}");
        return to_error(StatusCode::INTERNAL_SERVER_ERROR, "借阅失败");
    }
    (
        StatusCode::OK,
        Json(ErrorResponse {
            success: true,
            message: "借阅成功",
        }),
    )
}

#[derive(Deserialize)]
struct ReturnBookRequest {
    user_id: usize,
}

async fn return_book(
    claims: Claims,
    State(state): State<AppState>,
    Path(book_id): Path<usize>,
    Json(body): Json<ReturnBookRequest>,
) -> impl IntoResponse {
    if !claims.is_manager() {
        return to_error(StatusCode::FORBIDDEN, "权限不足");
    }
    let mut db = state.db.db().clone();
    let records = match Record::filter_by_book_id(&book_id)
        .filter_by_user_id(&body.user_id)
        .exec(&mut db)
        .await
    {
        Ok(r) => r,
        Err(e) => {
            error!("{e}");
            return to_error(StatusCode::INTERNAL_SERVER_ERROR, "服务器内部错误");
        }
    };
    let record_id = match records.iter().find(|r| r.return_date.is_none()) {
        Some(r) => r.id,
        None => return to_error(StatusCode::NOT_FOUND, "未找到借阅记录"),
    };
    let mut record = match Record::filter_by_id(&record_id).first().exec(&mut db).await {
        Ok(Some(r)) => r,
        _ => return to_error(StatusCode::INTERNAL_SERVER_ERROR, "服务器内部错误"),
    };
    if let Err(e) = record
        .update()
        .return_date(Some(jiff::Timestamp::now()))
        .exec(&mut db)
        .await
    {
        error!("{e}");
        return to_error(StatusCode::INTERNAL_SERVER_ERROR, "归还失败");
    }
    (
        StatusCode::OK,
        Json(ErrorResponse {
            success: true,
            message: "归还成功",
        }),
    )
}

async fn book_borrow_records(
    claims: Claims,
    State(state): State<AppState>,
    Path(book_id): Path<usize>,
) -> impl IntoResponse {
    if !claims.is_manager() {
        return (
            StatusCode::FORBIDDEN,
            Json(BorrowRecordsResponse {
                success: false,
                borrow_records: None,
                message: "权限不足",
            }),
        );
    }
    let mut db = state.db.db().clone();
    let records = match Record::filter_by_book_id(&book_id).exec(&mut db).await {
        Ok(r) => r,
        Err(e) => {
            error!("{e}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(BorrowRecordsResponse {
                    success: false,
                    borrow_records: None,
                    message: "服务器内部错误",
                }),
            );
        }
    };
    let infos: Vec<_> = records.iter().map(record_to_info).collect();
    (
        StatusCode::OK,
        Json(BorrowRecordsResponse {
            success: true,
            borrow_records: Some(infos),
            message: "查询成功",
        }),
    )
}

async fn my_borrow_records(claims: Claims, State(state): State<AppState>) -> impl IntoResponse {
    let mut db = state.db.db().clone();
    let records = match Record::filter_by_user_id(&claims.user_id)
        .exec(&mut db)
        .await
    {
        Ok(r) => r,
        Err(e) => {
            error!("{e}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(BorrowRecordsResponse {
                    success: false,
                    borrow_records: None,
                    message: "服务器内部错误",
                }),
            );
        }
    };
    let infos: Vec<_> = records.iter().map(record_to_info).collect();
    (
        StatusCode::OK,
        Json(BorrowRecordsResponse {
            success: true,
            borrow_records: Some(infos),
            message: "查询成功",
        }),
    )
}

async fn user_borrow_records(
    claims: Claims,
    State(state): State<AppState>,
    Path(user_id): Path<usize>,
) -> impl IntoResponse {
    if !claims.is_manager() {
        return (
            StatusCode::FORBIDDEN,
            Json(BorrowRecordsResponse {
                success: false,
                borrow_records: None,
                message: "权限不足",
            }),
        );
    }
    let mut db = state.db.db().clone();
    let records = match Record::filter_by_user_id(&user_id).exec(&mut db).await {
        Ok(r) => r,
        Err(e) => {
            error!("{e}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(BorrowRecordsResponse {
                    success: false,
                    borrow_records: None,
                    message: "服务器内部错误",
                }),
            );
        }
    };
    let infos: Vec<_> = records.iter().map(record_to_info).collect();
    (
        StatusCode::OK,
        Json(BorrowRecordsResponse {
            success: true,
            borrow_records: Some(infos),
            message: "查询成功",
        }),
    )
}

#[derive(Deserialize)]
struct ExpiredQuery {
    expired_after: Option<String>,
}

async fn expired_borrow_records(
    claims: Claims,
    State(state): State<AppState>,
    Query(query): Query<ExpiredQuery>,
) -> impl IntoResponse {
    if !claims.is_manager() {
        return (
            StatusCode::FORBIDDEN,
            Json(BorrowRecordsResponse {
                success: false,
                borrow_records: None,
                message: "权限不足",
            }),
        );
    }
    let mut db = state.db.db().clone();
    let all_records = match Record::all().exec(&mut db).await {
        Ok(r) => r,
        Err(e) => {
            error!("{e}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(BorrowRecordsResponse {
                    success: false,
                    borrow_records: None,
                    message: "服务器内部错误",
                }),
            );
        }
    };
    let threshold = match &query.expired_after {
        Some(d) => match parse_date(d) {
            Ok(ts) => ts,
            Err(msg) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(BorrowRecordsResponse {
                        success: false,
                        borrow_records: None,
                        message: msg,
                    }),
                );
            }
        },
        None => jiff::Timestamp::now(),
    };
    let now = jiff::Timestamp::now();
    let expired: Vec<_> = all_records
        .iter()
        .filter(|r| r.return_date.is_none() && r.expire_date < now && r.expire_date > threshold)
        .map(record_to_info)
        .collect();
    (
        StatusCode::OK,
        Json(BorrowRecordsResponse {
            success: true,
            borrow_records: Some(expired),
            message: "查询成功",
        }),
    )
}

async fn all_borrow_records(claims: Claims, State(state): State<AppState>) -> impl IntoResponse {
    if !claims.is_manager() {
        return (
            StatusCode::FORBIDDEN,
            Json(BorrowRecordsResponse {
                success: false,
                borrow_records: None,
                message: "权限不足",
            }),
        );
    }
    let mut db = state.db.db().clone();
    let records = match Record::all().exec(&mut db).await {
        Ok(r) => r,
        Err(e) => {
            error!("{e}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(BorrowRecordsResponse {
                    success: false,
                    borrow_records: None,
                    message: "服务器内部错误",
                }),
            );
        }
    };
    let infos: Vec<_> = records.iter().map(record_to_info).collect();
    (
        StatusCode::OK,
        Json(BorrowRecordsResponse {
            success: true,
            borrow_records: Some(infos),
            message: "查询成功",
        }),
    )
}

// ==================== 续借申请 ====================

#[derive(Deserialize)]
struct RenewRequest {
    expired_after: String,
}

async fn submit_renewal(
    claims: Claims,
    State(state): State<AppState>,
    Path(record_id): Path<usize>,
    Json(body): Json<RenewRequest>,
) -> impl IntoResponse {
    let expired_after = match parse_date(&body.expired_after) {
        Ok(d) => d,
        Err(msg) => return to_error(StatusCode::BAD_REQUEST, msg),
    };
    let mut db = state.db.db().clone();
    let record = match Record::filter_by_id(&record_id).first().exec(&mut db).await {
        Ok(Some(r)) => r,
        _ => return to_error(StatusCode::NOT_FOUND, "借阅记录不存在"),
    };
    if record.user_id != claims.user_id {
        return to_error(StatusCode::FORBIDDEN, "只能续借自己的书籍");
    }
    if record.return_date.is_some() {
        return to_error(StatusCode::CONFLICT, "该书籍已归还");
    }
    if let Err(e) = Renew::create()
        .request_date(jiff::Timestamp::now())
        .expired_after(expired_after)
        .status(RenewStatus::Pending)
        .record_id(record_id)
        .exec(&mut db)
        .await
    {
        error!("{e}");
        return to_error(StatusCode::INTERNAL_SERVER_ERROR, "提交续借申请失败");
    }
    (
        StatusCode::OK,
        Json(ErrorResponse {
            success: true,
            message: "续借申请已提交",
        }),
    )
}

async fn list_renewals(claims: Claims, State(state): State<AppState>) -> impl IntoResponse {
    let mut db = state.db.db().clone();
    let all_renewals = match Renew::all().exec(&mut db).await {
        Ok(r) => r,
        Err(e) => {
            error!("{e}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(RenewalsResponse {
                    success: false,
                    renewals: None,
                    message: "服务器内部错误",
                }),
            );
        }
    };
    let renewals: Vec<&Renew> = if claims.is_manager() {
        all_renewals
            .iter()
            .filter(|r| matches!(r.status, RenewStatus::Pending))
            .collect()
    } else {
        let mut user_renewals = Vec::new();
        for renewal in &all_renewals {
            if let Ok(Some(record)) = Record::filter_by_id(&renewal.record_id)
                .first()
                .exec(&mut db)
                .await
            {
                if record.user_id == claims.user_id {
                    user_renewals.push(renewal);
                }
            }
        }
        user_renewals
    };
    let mut renewal_infos = Vec::new();
    for renewal in &renewals {
        let record = match Record::filter_by_id(&renewal.record_id)
            .first()
            .exec(&mut db)
            .await
        {
            Ok(Some(r)) => r,
            _ => continue,
        };
        let status = if claims.is_manager() {
            None
        } else {
            Some(
                match renewal.status {
                    RenewStatus::Pending => "pending",
                    RenewStatus::Approved => "approved",
                    RenewStatus::Rejected => "rejected",
                }
                .to_string(),
            )
        };
        renewal_infos.push(RenewalInfo {
            id: renewal.id,
            user_id: record.user_id,
            book_id: record.book_id,
            request_date: format_date(&renewal.request_date),
            expired_after: format_date(&renewal.expired_after),
            status,
        });
    }
    (
        StatusCode::OK,
        Json(RenewalsResponse {
            success: true,
            renewals: Some(renewal_infos),
            message: "查询成功",
        }),
    )
}

#[derive(Deserialize)]
struct ApproveRenewalRequest {
    approved: bool,
}

async fn approve_renewal(
    claims: Claims,
    State(state): State<AppState>,
    Path(renewal_id): Path<usize>,
    Json(body): Json<ApproveRenewalRequest>,
) -> impl IntoResponse {
    if !claims.is_manager() {
        return to_error(StatusCode::FORBIDDEN, "权限不足");
    }
    let mut db = state.db.db().clone();
    let mut renewal = match Renew::filter_by_id(&renewal_id).first().exec(&mut db).await {
        Ok(Some(r)) => r,
        _ => return to_error(StatusCode::NOT_FOUND, "续借申请不存在"),
    };
    if !matches!(renewal.status, RenewStatus::Pending) {
        return to_error(StatusCode::CONFLICT, "该申请已被处理");
    }
    let new_status = if body.approved {
        RenewStatus::Approved
    } else {
        RenewStatus::Rejected
    };
    if let Err(e) = renewal.update().status(new_status).exec(&mut db).await {
        error!("{e}");
        return to_error(StatusCode::INTERNAL_SERVER_ERROR, "审批失败");
    }
    if body.approved {
        if let Ok(Some(mut record)) = Record::filter_by_id(&renewal.record_id)
            .first()
            .exec(&mut db)
            .await
        {
            if let Err(e) = record
                .update()
                .expire_date(renewal.expired_after)
                .exec(&mut db)
                .await
            {
                error!("{e}");
                return to_error(StatusCode::INTERNAL_SERVER_ERROR, "更新到期日期失败");
            }
        }
    }
    (
        StatusCode::OK,
        Json(ErrorResponse {
            success: true,
            message: if body.approved {
                "续借申请已通过"
            } else {
                "续借申请已拒绝"
            },
        }),
    )
}
