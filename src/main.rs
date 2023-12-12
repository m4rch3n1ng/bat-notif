use crate::{
	bat::{BatState, Bats},
	notif::Notif,
};
use std::{
	sync::{
		atomic::{AtomicBool, Ordering},
		Arc,
	},
	time::Duration,
};

mod bat;
mod notif;

struct Config {
	interval: u64,
	low_pct: usize,
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
	fn interval(&self) -> Duration {
		Duration::from_secs(self.interval)
	}
}

struct App {
	bats: Bats,
	state: BatState,
	notif: Notif,
}

impl App {
	fn init() -> App {
		let mut bats = Bats::init();
		let state = bats.state();
		let notif = Notif::new();

		App { bats, state, notif }
	}

	fn run(&mut self, config: &Config) {
		let should_term = Arc::new(AtomicBool::new(false));
		let should_term_ctrlc = Arc::clone(&should_term);
		let (mut timer, canceller) = cancellable_timer::Timer::new2().unwrap();

		ctrlc::set_handler(move || {
			should_term_ctrlc.store(true, Ordering::Relaxed);
			let _ = canceller.cancel();
		})
		.expect("failed to set ctrlc handler");

		let interval = config.interval();
		while !should_term.load(Ordering::Relaxed) {
			self.once(config);
			timer.sleep(interval).unwrap();
		}
	}

	fn once(&mut self, config: &Config) {
		let state = self.bats.state();
		match (self.state, state) {
			(BatState::Discharging | BatState::Unknown, BatState::Charging) => {
				self.state = BatState::Charging;
				self.notif.charging();
			}
			(BatState::Charging | BatState::Unknown, BatState::Discharging) => {
				self.state = BatState::Discharging;
				self.notif.discharging();
			}
			(BatState::Charging, BatState::Charging)
			| (BatState::Discharging, BatState::Discharging)
			| (_, BatState::Unknown) => (),
		}

		let level = self.bats.level();
		if level < config.low_pct && self.state != BatState::Charging {
			self.notif.low();
		}
	}
}

fn main() {
	let mut app = App::init();
	let config = Config::default();
	app.run(&config);
}
