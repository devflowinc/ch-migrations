use clap::Parser;
use commands::{
    generate::generate_command, redo::redo_command, revert::revert_commmand, run::run_command, Cli,
    Commands, MigrationCommands,
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

    let source = args.source;

    let res: Result<(), CLIError> = match args.command {
        Commands::Migration(commands) => match commands {
            MigrationCommands::Generate(gen_args) => generate_command(gen_args, &source).await,
            MigrationCommands::Run => run_command(&source).await,
            MigrationCommands::Redo => redo_command(&source).await,
            MigrationCommands::Revert => revert_commmand(&source).await,
        },
    };

    if let Err(e) = res {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
