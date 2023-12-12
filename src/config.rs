use core::time::Duration;
use std::fs;
use serde::Deserialize;

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
	pub fn init() -> Self {
		let dir = dirs::config_dir().unwrap();
		let path = dir.join("bat-notif.json");

		let file = fs::read_to_string(path);
		match file {
			Ok(file) => serde_json::from_str::<Config>(&file).unwrap(),
			Err(_) => Config::default(),
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
