use crate::{
    errors::CLIError,
    operators::{
        clickhouse_operators::{
            check_if_migrations_table_exists, ensure_migrations_sync,
            get_clickhouse_client_and_ping, get_migrations_from_clickhouse, undo_migration,
        },
        migrations_operators::get_migrations_from_dir,
    },
    tools::migrations::SetupArgs,
};

pub async fn revert_commmand() -> Result<(), CLIError> {
    let config = SetupArgs::from_toml_file().await?;
    let client = get_clickhouse_client_and_ping(config).await?;

    if check_if_migrations_table_exists(client.clone()).await? == false {
        return Err(CLIError::BadArgs(
            "Migrations table does not exist. Run chm setup first!".to_string(),
        ));
    }

    let local_migrations = get_migrations_from_dir().await?;

    let applied_migrations = get_migrations_from_clickhouse(client.clone()).await?;

    dbg!(&local_migrations);
    dbg!(&applied_migrations);

    ensure_migrations_sync(local_migrations.clone(), applied_migrations).await?;

    undo_migration(
        client.clone(),
        local_migrations
            .last()
            .ok_or(CLIError::InternalError(
                "No migrations to revert".to_string(),
            ))?
            .clone(),
    )
    .await?;

    Ok(())
}
