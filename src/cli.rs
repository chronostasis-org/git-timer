// src/cli.rs
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "git-timer")]
#[command(about = "A coding time tracker", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Starts timer
    Start,
    /// Show status
    Status,
    /// Commit and add timer data
    Commit {
        /// Commit message (optional, opens editor if not provided)
        #[arg(short, long)]
        message: Option<String>,
    },
}

pub fn parse_args() -> Cli {
    Cli::parse()
}
