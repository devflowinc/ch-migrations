use chrono::{DateTime, Utc};

use crate::errors::CLIError;

pub struct MigrationOnDisk {
    pub timestamp: DateTime<Utc>,
    pub name: String,
}

impl MigrationOnDisk {
    fn from_str(s: String) -> Result<Self, CLIError> {
        let (timestamp, name) = s.split_once("_").expect("valid migration file name");
        let timestamp =
            chrono::DateTime::parse_from_str(timestamp, "%Y-%m-%d-%H%M%S").map_err(|_| {
                CLIError::InternalError("Failed to parse migration file timestamp".to_string())
            })?;

        Ok(Self {
            timestamp: timestamp.into(),
            name: name.to_string(),
        })
    }
}

pub async fn get_migrations_from_dir() -> Result<Vec<MigrationOnDisk>, CLIError> {
    let migrations_dir = std::env::current_dir()?.join("migrations");
    let mut reader = tokio::fs::read_dir(migrations_dir).await?;
    let mut migrations = Vec::new();
    while let Some(entry) = reader.next_entry().await? {
        let file_name = entry
            .file_name()
            .into_string()
            .expect("Valid unicode string");

        migrations.push(MigrationOnDisk::from_str(file_name)?)
    }

    Ok(migrations)
}
