use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::net::{Ipv4Addr, SocketAddr};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use tracing::info;

#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub service: ServiceConfig,
    pub database: DatabaseConfig,
}

#[derive(Serialize, Deserialize)]
pub struct ServiceConfig {
    pub listen: SocketAddr,
    pub data_path: PathBuf,
}

#[derive(Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub backend: DatabaseBackend,
    pub postgres_url: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DatabaseBackend {
    Sqlite,
    Postgres,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            service: ServiceConfig {
                listen: SocketAddr::new(Ipv4Addr::new(127, 0, 0, 1).into(), 3000),
                data_path: PathBuf::from_str("./data").unwrap(),
            },
            database: DatabaseConfig {
                backend: DatabaseBackend::Sqlite,
                postgres_url: None,
            },
        }
    }
}

impl AppConfig {
    pub fn from_file(path: &Path) -> Result<Self> {
        if !path.is_file() {
            info!("不存在配置文件，创建默认配置");
            let conf = AppConfig::default();
            let mut file = std::fs::File::create(path)?;
            file.write(toml::to_string_pretty(&conf)?.as_bytes())?;

            return Ok(conf);
        }

        let mut file = std::fs::File::open(path)?;
        let mut toml = String::new();
        file.read_to_string(&mut toml)?;
        let conf = toml::from_str(&toml)?;
        Ok(conf)
    }
}
