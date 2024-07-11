use clap::Args;
use serde::{Deserialize, Serialize};

use crate::{
    errors::CLIError,
    operators::{
        clickhouse_operators::{
            apply_migrations, create_migrations_table, ensure_migrations_sync,
            get_clickhouse_client_and_ping, get_migrations_from_clickhouse,
        },
        migrations_operators::{get_migrations_from_dir, MigrationOnDisk},
    },
};

#[derive(Args, Clone, Deserialize, Serialize)]
pub struct SetupArgs {
    /// Clickhouse URL
    #[arg(env = "CLICKHOUSE_URL", default_value = None)]
    pub url: Option<String>,
    /// Clickhouse User
    #[arg(env = "CLICKHOUSE_USER", default_value = None)]
    pub user: Option<String>,
    /// Clickhouse Password
    #[arg(env = "CLICKHOUSE_PASSWORD", default_value = None)]
    pub password: Option<String>,
    /// Clickhouse Database
    #[arg(env = "CLICKHOUSE_DB", default_value = None)]
    pub database: Option<String>,
}

pub async fn run_pending_migrations(config: SetupArgs) -> Result<(), CLIError> {
    let client = get_clickhouse_client_and_ping(config).await?;

    create_migrations_table(client.clone()).await?;

    let local_migrations = get_migrations_from_dir().await?;

    let applied_migrations = get_migrations_from_clickhouse(client.clone()).await?;

    let local_migrations_not_in_db: Vec<MigrationOnDisk> = local_migrations
        .iter()
        .filter_map(|m| {
            if applied_migrations
                .iter()
                .find(|applied_migration| {
                    return applied_migration.version == m.version;
                })
                .is_some()
            {
                return None;
            }
            Some(m.clone())
        })
        .collect();

    ensure_migrations_sync(local_migrations.clone(), applied_migrations).await?;

    apply_migrations(client.clone(), local_migrations_not_in_db).await?;

    Ok(())
}
