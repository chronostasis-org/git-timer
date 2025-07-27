use anyhow::{Context, Result};
use chrono::{DateTime, Local};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::{env, fs};

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
    },
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
            Ok(runtime_dir) if !runtime_dir.is_empty() => {
                let path = PathBuf::from(runtime_dir).join("git-timer.json");
                log::debug!("Using XDG_RUNTIME_DIR for timer data: {:?}", path);
                path
            }
            Ok(_) => {
                log::warn!("XDG_RUNTIME_DIR is set, but empty, using local file");
                let path = PathBuf::from("./.git-timer.json");
                log::debug!("Fallback path: {:?}", path);
                path
            }
            Err(e) => {
                log::warn!("XDG_RUNTIME_DIR not available ({}), using local file", e);
                let path = PathBuf::from("./.git-timer.json");
                log::debug!("Fallback path: {:?}", path);
                path
            }
        }
    }

    fn save(&self, path: &Path) -> Result<()> {
        log::debug!("Saving timer data to {:?}", path);
        let data = serde_json::to_string_pretty(&self)?;
        fs::write(path, &data)
            .with_context(|| format!("Failed to write timer data to {:?}", path))?;
        log::debug!("Timer data saved successfully");
        Ok(())
    }

    fn load(path: &Path) -> Result<Self> {
        if path.exists() {
            log::debug!("Loading data from existing file: {:?}", path);
            let data =
                fs::read_to_string(&path).with_context(|| format!("Failed to read {:?}", path))?;
            let timer_data: TimerData =
                serde_json::from_str(&data).with_context(|| "Failed to parse timer data")?;
            log::debug!("Timer data loaded successfully");
            Ok(timer_data)
        } else {
            log::info!(
                "No existing timer data found at {:?}, creating new timer",
                path
            );
            Ok(TimerData {
                name: None,
                start: None,
                end: None,
            })
        }
    }
}

fn main() -> Result<()> {
    env_logger::init();
    log::info!("Main function start");

    log::debug!("Parsing command line arguments");
    let args = Cli::parse();

    log::debug!("Getting timer data path");
    let data_path = TimerData::default_path();

    log::debug!("Loading timer data from data_path: {:?}", data_path);
    let mut timer_data: TimerData = TimerData::load(&data_path)?;

    match args.command {
        Commands::Start { timer_name } => {
            log::debug!("Starting git timer");
            start_timer(&timer_name, &mut timer_data, &data_path)?;
        }
        Commands::Status => {
            log::debug!("Showing git timer's status");
            show_status(&timer_data)?;
        }
        Commands::Commit { message } => {
            log::debug!("Committing git timer data");
            commit_with_timer(&message, &mut timer_data, &data_path)?;
        }
    }
    Ok(())
}

fn start_timer(timer_name: &str, timer_data: &mut TimerData, data_path: &Path) -> Result<()> {
    log::info!("Starting timer '{}'", timer_name);

    let now: DateTime<Local> = Local::now();
    timer_data.name = Some(timer_name.to_owned());
    timer_data.start = Some(now);
    timer_data.end = None;

    timer_data.save(data_path)?;
    println!(
        "Started timer '{}' at {}",
        timer_name,
        now.format("%H:%M:%S")
    );
    Ok(())
}

fn show_status(timer_data: &TimerData) -> Result<()> {
    match (&timer_data.name, &timer_data.start, &timer_data.end) {
        // Timer is running
        (Some(name), Some(start), None) => {
            let now: DateTime<Local> = Local::now();
            let duration = now.signed_duration_since(*start);
            let mins = duration.num_minutes();
            let secs = duration.num_seconds() % 60;
            println!(
                "Timer '{}' has been running for {} minutes and {} seconds",
                name, mins, secs
            );
        }
        _ => println!("No timer running."),
    }
    Ok(())
}

fn commit_with_timer(message: &str, timer_data: &mut TimerData, data_path: &Path) -> Result<()> {
    log::info!("Committing with timer data");

    let now: DateTime<Local> = Local::now();
    timer_data.end = Some(now);

    log::debug!("Commit message: {}", message);
    timer_data.save(data_path)?;
    Ok(())
}
