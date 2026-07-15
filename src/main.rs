use crate::api::route;
use crate::config::AppConfig;
use crate::database::Database;
use crate::types::User;
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHasher};
use rand::Rng;
use tracing::info;

mod api;
pub mod config;
pub mod database;
mod types;

#[derive(Clone)]
pub struct AppState {
    db: Database,
}

async fn init_admin(db: &Database) {
    let mut conn = db.db().clone();
    let count = match User::all().count().exec(&mut conn).await {
        Ok(c) => c,
        Err(_) => return,
    };
    if count > 0 {
        return;
    }

    let password: String = rand::rng()
        .sample_iter(&rand::distr::Alphanumeric)
        .take(12)
        .map(char::from)
        .collect();

    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let hashed = argon2
        .hash_password(password.as_bytes(), &salt)
        .expect("Argon2 error")
        .to_string();

    if let Err(e) = User::create()
        .username("admin".to_string())
        .password(hashed)
        .permission("manager".to_string())
        .phone(None)
        .email(None)
        .token(None)
        .register_date(jiff::Timestamp::now())
        .exec(&mut conn)
        .await
    {
        tracing::error!("创建初始管理员失败: {e}");
        return;
    }

    info!("========================================");
    info!("  初始管理员已创建");
    info!("  用户名: admin");
    info!("  密码:   {password}");
    info!("========================================");
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let cfg = AppConfig::from_file("config.toml".as_ref())?;
    let db = Database::open(&cfg).await?;

    // 首次启动时创建初始管理员
    init_admin(&db).await;

    let listener = tokio::net::TcpListener::bind(cfg.service.listen).await?;

    info!("服务端启动，监听: {}", cfg.service.listen.to_string());
    let state = AppState { db };
    let app = axum::Router::new().nest("/api", route()).with_state(state);

    axum::serve(listener, app).await?;
    Ok(())
}
