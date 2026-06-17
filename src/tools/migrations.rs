use crate::{
    errors::CLIError,
    operators::{
        clickhouse_operators::{
            apply_migrations, create_migrations_table, get_clickhouse_client_and_ping,
            get_migrations_from_clickhouse,
        },
        migrations_operators::{get_migrations_from_dir, MigrationOnDisk},
    },
};
use std::path::Path;

pub async fn run_pending_migrations(source: &Path) -> Result<(), CLIError> {
    let client = get_clickhouse_client_and_ping().await?;

    create_migrations_table(client.clone()).await?;

    let local_migrations = get_migrations_from_dir(source).await?;

    let applied_migrations = get_migrations_from_clickhouse(client.clone()).await?;

    let local_migrations_not_in_db: Vec<MigrationOnDisk> = local_migrations
        .iter()
        .filter_map(|m| {
            if applied_migrations
                .iter()
                .any(|applied_migration| applied_migration.version == m.version)
            {
                return None;
            }
            Some(m.clone())
        })
        .collect();

    apply_migrations(client.clone(), local_migrations_not_in_db).await?;

    Ok(())
}
