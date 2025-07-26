use clap::{Parser, Subcommand};
use anyhow::{Result, Context};
use chrono::{DateTime, Local};
use std::path::{Path, PathBuf};
use std::{fs, env};
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize, Deserialize)]
struct TimerData {
    name: Option<String>,
    start: Option<DateTime<Local>>,
    end: Option<DateTime<Local>>,
}

impl TimerData {
    fn default_path() -> PathBuf {
        match env::var("XDG_RUNTIME_DIR") {
            Ok(runtime_dir) => {
                println!("XDG_RUNTIME_DIR is found. Writing data to $XDG_RUNTIME_DIR/git-timer.json");
                PathBuf::from(runtime_dir).join("git-timer.json")
            },
            Err(e) => {
                eprintln!("Could not get XDG_RUNTIME_DIR, falling back to local file: {}", e);
                PathBuf::from("./.git-timer.json")
            }
        }
    }

    fn save(&self, path: &Path) -> Result<()> {
        // let data = String::from("mock data\n");
        let data = serde_json::to_string_pretty(&self)?;
        fs::write(path, data)
            .with_context(|| "Failed to write timer data")?;
        Ok(())
    }

    fn load(path: &Path) -> Result<Self> {
        if path.exists() {
            let data = fs::read_to_string(&path)
                .with_context(|| format!("Failed to read {:?}", path))?;
            let timer_data: TimerData = serde_json::from_str(&data)
                .with_context(|| "Failed to parse timer data")?;
            Ok(timer_data)
        } else {
            Ok( TimerData {
                    name: None,
                    start: None,
                    end: None,
            })
        }
    }
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let data_path = TimerData::default_path();
    let mut timer_data: TimerData = TimerData::load(&data_path)?;

    match args.command {
        Commands::Start { timer_name } => {
            start_timer(&timer_name, &mut timer_data, &data_path)?;
        },
        Commands::Status => {
            show_status(&timer_data)?;
        },
        Commands::Commit { message } => {
            commit_with_timer(&message, &mut timer_data, &data_path)?;
        },
    }
    Ok(())
}

fn start_timer(timer_name: &str, timer_data: &mut TimerData, data_path: &Path) -> Result<()> {
    let cur_local_time: DateTime<Local> = Local::now(); // Think about a better variable name
    timer_data.name = Some(timer_name.to_owned());
    timer_data.start = Some(cur_local_time);
    timer_data.end = None;
    timer_data.save(data_path)?;
    println!("Started timer '{}' at {}", timer_name, cur_local_time.format("%H:%M:%S"));
    Ok(())
}

fn show_status(timer_data: &TimerData) -> Result<()> {
    match (&timer_data.name, &timer_data.start) {
        (Some(name), Some(start)) => {
            println!("Timer '{}' started at {}", name, start.format("%Y-%m-%d %H:%M:%S"));
        },
        _ => println!("No timer running."),
    }
    Ok(())
}

fn commit_with_timer(message: &str, timer_data: &mut TimerData, data_path: &Path) -> Result<()> {
    let cur_local_time: DateTime<Local> = Local::now(); // Think about a better variable name
    timer_data.end = Some(cur_local_time);
    println!("Ran command: git commit -m \"{message}\"");
    println!("Timer end: {}", cur_local_time.format("%H:%M:%S"));
    timer_data.save(data_path)?;
    Ok(())
}