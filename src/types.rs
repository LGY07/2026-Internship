use toasty::{Deferred, Embed, Model};
use uuid::Uuid;

#[derive(Model)]
pub struct User {
    #[key]
    #[auto]
    pub id: usize,

    #[index]
    pub username: String,
    pub password: String,

    pub permission: String,

    pub phone: Option<String>,
    pub email: Option<String>,

    register_date: jiff::Timestamp,

    #[has_one]
    pub token: Deferred<Option<Token>>,

    #[has_many]
    pub records: Deferred<Vec<Record>>,
}

#[derive(Model)]
pub struct Book {
    #[key]
    #[auto]
    pub id: usize,

    pub title: String,
    pub author: String,
    pub description: Option<String>,
    pub published_date: Option<jiff::Timestamp>,

    #[has_many]
    pub records: Deferred<Vec<Record>>,
    #[has_many]
    pub category: Deferred<Vec<BookCategory>>,
}

#[derive(Model)]
pub struct Record {
    #[key]
    #[auto]
    pub id: usize,

    pub borrow_date: jiff::Timestamp,
    pub expire_date: jiff::Timestamp,
    pub return_date: Option<jiff::Timestamp>,

    #[index]
    pub user_id: usize,
    #[index]
    pub book_id: usize,

    #[belongs_to]
    pub user: Deferred<User>,
    #[belongs_to]
    pub book: Deferred<Book>,
    #[has_many]
    pub renew: Deferred<Vec<Renew>>,
}

#[derive(Model)]
pub struct Category {
    #[key]
    #[auto]
    pub id: usize,

    #[index]
    pub name: String,

    #[has_many]
    pub books: Deferred<Vec<BookCategory>>,
}

#[derive(Model)]
pub struct BookCategory {
    #[key]
    #[auto]
    pub id: usize,

    #[index]
    pub book_id: usize,
    #[index]
    pub category_id: usize,

    #[belongs_to]
    pub book: Deferred<Book>,

    #[belongs_to]
    pub category: Deferred<Category>,
}

#[derive(Model)]
pub struct Renew {
    #[key]
    #[auto]
    pub id: usize,

    pub request_date: jiff::Timestamp,
    pub expired_after: jiff::Timestamp,
    pub status: RenewStatus,

    #[index]
    pub record_id: usize,

    #[belongs_to]
    pub record: Deferred<Record>,
}

#[derive(Embed)]
pub enum RenewStatus {
    #[column(variant = 0)]
    Pending,
    #[column(variant = 1)]
    Approved,
    #[column(variant = 2)]
    Rejected,
}

#[derive(Model)]
pub struct Token {
    #[key]
    #[auto]
    id: usize,

    #[index]
    pub token: Uuid,
    pub expired_date: jiff::Timestamp,

    #[index]
    pub user_id: usize,

    #[belongs_to]
    user: Deferred<User>,
}
