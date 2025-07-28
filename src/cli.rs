// src/cli.rs - Simple version
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
    Start {
        /// Timer's name
        timer_name: String,
    },
    /// Show status
    Status,
    /// Commit and add timer data
    Commit {
        /// Commit message
        #[arg(short, long)]
        message: String,
    },
}

pub fn parse_args() -> Cli {
    Cli::parse()
}
