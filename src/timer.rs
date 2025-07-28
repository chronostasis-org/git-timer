use anyhow::{Context, Result};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct TimerData {
    pub name: Option<String>,
    pub start: Option<DateTime<Local>>,
    pub end: Option<DateTime<Local>>,
}

impl TimerData {
    pub fn new() -> Self {
        TimerData {
            name: None,
            start: None,
            end: None,
        }
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        log::debug!("Saving timer data to {:?}", path);
        let data = serde_json::to_string_pretty(&self)?;
        fs::write(path, &data)
            .with_context(|| format!("Failed to write timer data to {:?}", path))?;
        log::debug!("Timer data saved successfully");
        Ok(())
    }

    pub fn load(path: &Path) -> Result<Self> {
        if path.exists() {
            log::debug!("Loading data from existing file: {:?}", path);
            let data =
                fs::read_to_string(path).with_context(|| format!("Failed to read {:?}", path))?;
            let timer_data: TimerData =
                serde_json::from_str(&data).with_context(|| "Failed to parse timer data")?;
            log::debug!("Timer data loaded successfully");
            Ok(timer_data)
        } else {
            log::info!("No existing timer data found, creating new timer");
            Ok(Self::new())
        }
    }

    pub fn is_running(&self) -> bool {
        self.start.is_some() && self.end.is_none()
    }

    pub fn calculate_duration(&self) -> Option<(i64, i64)> {
        match (&self.start, &self.end) {
            (Some(start), Some(end)) => {
                let duration = end.signed_duration_since(*start);
                let mins = duration.num_minutes();
                let secs = duration.num_seconds() % 60;
                Some((mins, secs))
            }
            (Some(start), None) => {
                let now = Local::now();
                let duration = now.signed_duration_since(*start);
                let mins = duration.num_minutes();
                let secs = duration.num_seconds() % 60;
                Some((mins, secs))
            }
            _ => None,
        }
    }
}
