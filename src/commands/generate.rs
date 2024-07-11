use crate::{errors::CLIError, GenerateArgs};
use chrono::prelude::*;

pub async fn generate_command(args: GenerateArgs) -> Result<(), CLIError> {
    let migrations_dir = std::env::current_dir()?.join("ch_migrations");

    if !migrations_dir.is_dir() {
        return Err(CLIError::BadArgs(
            "Migrations directory does not exist. Please run chm setup first!".to_string(),
        ));
    }

    let migration_dir = migrations_dir.join(format!("{}_{}", Utc::now().timestamp(), args.name));

    tokio::fs::create_dir(migration_dir.clone()).await?;

    tokio::fs::File::create(migration_dir.clone().join("up.sql")).await?;
    tokio::fs::File::create(migration_dir.clone().join("down.sql")).await?;

    Ok(())
}
