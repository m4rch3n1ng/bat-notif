use color_eyre::eyre::eyre;
use core::time::Duration;
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
	#[serde(default = "default_interval")]
	interval: u64,
	#[serde(default = "default_low_pct")]
	pub low_pct: usize,
}

impl Default for Config {
	fn default() -> Self {
		Config {
			interval: 10,
			low_pct: 15,
		}
	}
}

impl Config {
	pub fn init() -> color_eyre::Result<Self> {
		let dir = dirs::config_dir().ok_or_else(|| eyre!("couldn't get config_dir"))?;
		let path = dir.join("bat-notif.json");

		let file = fs::read_to_string(path);
		match file {
			Ok(file) => {
				let config = serde_json::from_str::<Config>(&file)?;
				Ok(config)
			}
			Err(_) => Ok(Config::default()),
		}
	}

	pub fn interval(&self) -> Duration {
		Duration::from_secs(self.interval)
	}
}

const fn default_interval() -> u64 {
	10
}

const fn default_low_pct() -> usize {
	15
}
