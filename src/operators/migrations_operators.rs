use std::{cmp::Ordering, path::PathBuf};

use chrono::NaiveDateTime;

use crate::errors::CLIError;

#[derive(Debug, Clone)]
pub struct MigrationOnDisk {
    pub timestamp: NaiveDateTime,
    pub up_query: String,
    pub down_query: String,
}

impl MigrationOnDisk {
    async fn from_str(path: PathBuf) -> Result<Self, CLIError> {
        if path.file_name().is_none() {
            return Err(CLIError::BadArgs(
                "Migration file has no file name".to_string(),
            ));
        }
        let name = path
            .file_name()
            .unwrap()
            .to_os_string()
            .into_string()
            .map_err(|_| CLIError::BadArgs("Invalid migration file name".to_string()))?;

        let (timestamp, _) = name.split_once("_").expect("valid migration file name");
        println!("{timestamp}");

        let timestamp = chrono::NaiveDateTime::parse_from_str(timestamp, "%Y-%m-%d-%H-%M-%S")
            .map_err(|_| {
                CLIError::InternalError("Failed to parse migration file timestamp".to_string())
            })?;

        let up_file = tokio::fs::read_to_string(path.join("up.sql")).await?;

        let down_file = tokio::fs::read_to_string(path.join("down.sql")).await?;

        Ok(Self {
            timestamp: timestamp.into(),
            up_query: up_file,
            down_query: down_file,
        })
    }
}

pub async fn get_migrations_from_dir() -> Result<Vec<MigrationOnDisk>, CLIError> {
    let migrations_dir = std::env::current_dir()?.join("migrations");
    let mut reader = tokio::fs::read_dir(migrations_dir).await?;
    let mut migrations = Vec::new();

    while let Some(entry) = reader.next_entry().await? {
        migrations.push(MigrationOnDisk::from_str(entry.path()).await?)
    }

    migrations.sort_by(|a, b| {
        a.timestamp
            .partial_cmp(&b.timestamp)
            .unwrap_or(Ordering::Less)
    });

    Ok(migrations)
}
