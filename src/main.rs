use clap::Parser;

// Root modules
mod cli;
mod commands;

use cli::{Cli, Commands};

fn main() {
    let args = Cli::Parse();

    match args.command {
        Commands::Log { message } => {
            commands::log::execute(message);
        }
        Commands::Today => {
            commands::log::execute();
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
