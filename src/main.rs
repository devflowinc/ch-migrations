use clap::Parser;
use commands::{
    generate::generate_command, redo::redo_command, revert::revert_commmand, run::run_command,
    setup::setup_command, Cli, Commands, MigrationCommands,
};
use errors::CLIError;
mod commands;
mod errors;
mod operators;
mod tools;

#[tokio::main]
async fn main() {
    let err = dotenvy::dotenv();

    if let Err(e) = err {
        eprintln!("Env not found: {}", e);
        std::process::exit(1);
    }

    let args = Cli::parse();

    let res: Result<(), CLIError> = match args.command {
        Commands::Setup(args) => setup_command(args).await,
        Commands::Migration(commands) => match commands {
            MigrationCommands::Generate(args) => generate_command(args).await,
            MigrationCommands::Run => run_command().await,
            MigrationCommands::Redo => redo_command().await,
            MigrationCommands::Revert => revert_commmand().await,
        },
    };

    if let Err(e) = res {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
