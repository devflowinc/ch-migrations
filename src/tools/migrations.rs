use crate::{
    errors::CLIError,
    operators::{
        clickhouse_operators::{
            apply_migrations, create_migrations_table, ensure_migrations_sync,
            get_migrations_from_clickhouse,
        },
        migrations_operators::{get_migrations_from_dir, MigrationOnDisk},
    },
};

pub async fn run_pending_migrations(client: clickhouse::Client) -> Result<(), CLIError> {
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
