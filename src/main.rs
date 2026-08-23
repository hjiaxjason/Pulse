use clap::Parser;

// Root modules
mod cli;
mod commands;
mod log;
mod config;
mod state;
pub mod notify;
mod tick;
mod daemon;

use cli::{Cli, Commands};

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Log { message, kind } => {
            commands::log::execute(message, kind).expect("Failed to execute log command");
        }
        Commands::Today => {
            commands::today::execute();
        }
        Commands::Week => {
            commands::week::execute();
        }
        Commands::Install => {
            commands::install::execute();
        }
        Commands::Config => {
            commands::config::execute();
        }
    }
}
