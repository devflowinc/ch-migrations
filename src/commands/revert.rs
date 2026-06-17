use crate::{
    errors::CLIError,
    operators::{
        clickhouse_operators::{
            check_if_migrations_table_exists, get_clickhouse_client_and_ping,
            get_last_migration_from_clickhouse, undo_migration,
        },
        migrations_operators::get_migrations_from_dir,
    },
};
use std::path::Path;

pub async fn revert_commmand(source: &Path) -> Result<(), CLIError> {
    let client = get_clickhouse_client_and_ping().await?;

    if !(check_if_migrations_table_exists(client.clone()).await?) {
        return Err(CLIError::InternalError("Migrations table does not exist".to_string()));
    }

    let last = get_last_migration_from_clickhouse(client.clone())
        .await?
        .ok_or_else(|| CLIError::InternalError("No migrations to revert".to_string()))?;

    let local = get_migrations_from_dir(source).await?;
    let migration = local
        .into_iter()
        .find(|m| m.version == last.version)
        .ok_or_else(|| {
            CLIError::InternalError(format!(
                "Migration {} is applied in DB but not found on disk",
                last.version
            ))
        })?;

    undo_migration(client.clone(), migration).await?;
    Ok(())
}
