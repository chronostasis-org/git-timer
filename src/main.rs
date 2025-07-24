use clap::{Parser, Subcommand};
use anyhow::Result;
use chrono::prelude::*;

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

struct TimerData {
    name: Option<String>,
    start: Option<DateTime<Local>>,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let mut timer_data = TimerData {
        name: None,
        start: None,
    };

    match args.command {
        Commands::Start { timer_name } => {
            start_timer(&timer_name, &mut timer_data)?;
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

fn start_timer(timer_name: &str, timer_data: &mut TimerData) -> Result<()> {
    let cur_local_time: DateTime<Local> = Local::now();
    timer_data.name = Some(timer_name.to_owned());
    timer_data.start = Some(cur_local_time);
    println!("Started timer '{}' at {}", timer_name, cur_local_time.format("%H:%M:%S"));
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