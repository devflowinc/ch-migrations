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
    let up_file = migration_dir.clone().join("up.sql");
    let down_file = migration_dir.clone().join("down.sql");
    tokio::fs::File::create(up_file.clone()).await?;
    tokio::fs::File::create(down_file.clone()).await?;

    println!("Up.sql created at: {}", up_file.display());
    println!("Down.sql created at: {}", down_file.display());
    Ok(())
}
