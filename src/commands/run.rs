use crate::{
    errors::CLIError,
    operators::{
        clickhouse_operators::{
            apply_migrations_from_local, create_migrations_table_if_exists,
            drop_migrations_table_if_exists, get_clickhouse_client_and_ping,
            get_migrations_from_clickhouse, MigrationRow,
        },
        migrations_operators::{get_migrations_from_dir, MigrationOnDisk},
    },
};

use super::setup::RequiredSetupArgs;

pub async fn run_command() -> Result<(), CLIError> {
    let config = RequiredSetupArgs::from_toml_file().await?;
    let client = get_clickhouse_client_and_ping(config).await?;

    drop_migrations_table_if_exists(client.clone()).await?;
    create_migrations_table_if_exists(client.clone()).await?;

    let local_migrations = get_migrations_from_dir().await?;

    let db_migrations = get_migrations_from_clickhouse(client.clone()).await?;

    let db_migrations_not_in_local: Vec<MigrationRow> = db_migrations
        .iter()
        .filter_map(|m| {
            let db_migration_timestamp =
                chrono::NaiveDateTime::parse_from_str(&m.version, "%Y-%m-%d-%H-%M-%S");
            if db_migration_timestamp.is_err() {
                return None;
            }
            let db_migration_timestamp = db_migration_timestamp.unwrap();
            if local_migrations
                .iter()
                .find(|lm| lm.timestamp == db_migration_timestamp)
                .is_some()
            {
                return None;
            }
            Some(m.clone())
        })
        .collect();

    let local_migrations_not_in_db: Vec<MigrationOnDisk> = local_migrations
        .iter()
        .filter_map(|m| {
            if db_migrations
                .iter()
                .find(|db_migration| {
                    let db_migration_timestamp = chrono::NaiveDateTime::parse_from_str(
                        &db_migration.version,
                        "%Y-%m-%d-%H-%M-%S",
                    );
                    if db_migration_timestamp.is_err() {
                        return false;
                    }
                    return db_migration_timestamp.unwrap() == m.timestamp;
                })
                .is_some()
            {
                return None;
            }
            Some(m.clone())
        })
        .collect();

    if db_migrations_not_in_local.len() > 0 {
        return Err(CLIError::BadArgs("Migrations not in local".to_string()));
    }

    apply_migrations_from_local(client.clone(), local_migrations_not_in_db).await?;

    Ok(())
}
