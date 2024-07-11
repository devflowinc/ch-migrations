use std::{cmp::Ordering, path::PathBuf};

use crate::errors::CLIError;

#[derive(Debug, Clone)]
pub struct MigrationOnDisk {
    pub version: String,
    pub name: String,
    pub path: PathBuf,
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

        let (version, _) = name.split_once("_").expect("Invalid migration file name");

        Ok(Self {
            version: version.into(),
            name,
            path,
        })
    }

    pub async fn get_up_query(&self) -> Result<String, CLIError> {
        tokio::fs::read_to_string(self.path.join("up.sql"))
            .await
            .map_err(Into::into)
    }

    pub async fn get_down_query(&self) -> Result<String, CLIError> {
        tokio::fs::read_to_string(self.path.join("down.sql"))
            .await
            .map_err(Into::into)
    }
}

pub async fn get_migrations_from_dir() -> Result<Vec<MigrationOnDisk>, CLIError> {
    let migrations_dir = std::env::current_dir()?.join("ch_migrations");
    let mut reader = tokio::fs::read_dir(migrations_dir).await?;
    let mut migrations = Vec::new();

    while let Some(entry) = reader.next_entry().await? {
        if entry
            .file_name()
            .to_str()
            .is_some_and(|s| s.starts_with("chm.toml"))
        {
            continue;
        }
        migrations.push(MigrationOnDisk::from_str(entry.path()).await?)
    }

    migrations.sort_by(|a, b| a.version.partial_cmp(&b.version).unwrap_or(Ordering::Less));

    Ok(migrations)
}
