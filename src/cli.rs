use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "pulse", 
    version = "1.0",
    author "Jason Huang (Github: github.com/hjiaxjason)",
    about = "A CLI application for water and stretching breaks",
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a manual entry with a work summary
    Log { message: String },

    /// Look at today's logs
    Today,

    /// Look at this week's logs
    Week,

    /// Installation
    Install,
    
    /// Configuring nudges
    Config,
}

