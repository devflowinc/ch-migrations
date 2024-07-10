use crate::{errors::CLIError, GenerateArgs};
use chrono::prelude::*;

pub async fn generate_command(args: GenerateArgs) -> Result<(), CLIError> {
    let migrations_dir = std::env::current_dir()?.join("migrations");

    let migration_dir = migrations_dir.join(format!(
        "{}_{}",
        Utc::now().format("%Y-%m-%d-%H%M%S"),
        args.name
    ));

    tokio::fs::create_dir(migration_dir.clone()).await?;

    tokio::fs::File::create(migration_dir.clone().join("up.sql")).await?;
    tokio::fs::File::create(migration_dir.clone().join("down.sql")).await?;

    Ok(())
}
