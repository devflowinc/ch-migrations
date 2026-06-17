use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

pub mod generate;
pub mod redo;
pub mod revert;
pub mod run;

#[derive(Parser)]
#[command(author, version)]
#[command(name = "chm", about = "Clickhouse migration tool")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(long, short = 's', global = true, default_value = "ch_migrations", help = "Directory containing migrations")]
    pub source: PathBuf,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Commands to mutate migrations
    #[command(subcommand)]
    Migration(MigrationCommands),
}

#[derive(Subcommand)]
pub enum MigrationCommands {
    Generate(GenerateArgs),
    /// Command to first identify pending migrations and run the new up migrations
    Run,
    /// Command to revert and then apply the latest migration
    Redo,
    /// Command to revert last migration
    Revert,
}

#[derive(Args, Debug)]
pub struct GenerateArgs {
    /// Name of the migration to be generated
    pub name: String,
}
