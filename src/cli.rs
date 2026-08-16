use clap::{Parser, Subcommand};
use commands::{log::Log, today::Today, week::Week, install::Install, config::Config};

#[derive(Parser)]
#[command(name = "pulse", about = "A CLI applicatino for water and stretching breaks")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Log { message: String },
    Today,
    Week,
    Install,
    Config,
}

