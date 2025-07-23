use clap::{Parser, Subcommand};
use anyhow::Result;

#[derive(Debug, Parser)]
#[command(name = "git-timer")]
#[command(about = "A coding time tracker", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
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
    }
}

fn main() -> Result<()> {
    let args = Cli::parse();

    match args.command {
        Commands::Start { timer_name } => {
            start_timer(&timer_name)?;
        },
        Commands::Status => {
            show_status()?;
        },
        Commands::Commit { message } => {
            commit_with_timer(&message)?;
        },
    }
    Ok(())
}

fn start_timer(timer_name: &str) -> Result<()> {
    println!("Started timer {timer_name}");
    Ok(())
}

fn show_status() -> Result<()> {
    println!("Status info");
    Ok(())
}

fn commit_with_timer(message: &str) -> Result<()> {
    println!("Ran command: git commit -m \"{message}\"");
    Ok(())
}