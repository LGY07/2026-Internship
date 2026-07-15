use crate::config::{AppConfig, DatabaseBackend};
use anyhow::{Result, anyhow};
use toasty::Db;

#[derive(Clone)]
pub struct Database {
    db: Db,
}

impl Database {
    pub async fn open(config: &AppConfig) -> Result<Self> {
        let db_url = if let DatabaseBackend::Sqlite = config.database.backend {
            if !config.service.data_path.is_dir() {
                std::fs::create_dir_all(&config.service.data_path)?;
            }
            let db_path = config.service.data_path.join("db.sqlite");
            let db_path_str = db_path
                .to_str()
                .expect("FATAL: Database path is not a valid UTF-8 string!");
            format!("sqlite:{}", db_path_str)
        } else if let Some(url) = config.database.postgres_url.clone() {
            url
        } else {
            return Err(anyhow!("Database URL has not been set."));
        };

        let db = Db::builder()
            .models(toasty::models!(crate::*))
            .connect(&db_url)
            .await?;

        let _ = db.push_schema().await;

        Ok(Self { db })
    }
    pub fn db(&self) -> &Db {
        &self.db
    }
}
