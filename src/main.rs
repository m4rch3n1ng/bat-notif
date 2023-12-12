use crate::{
	bat::{BatState, Bats},
	config::Config,
	notif::Notif,
};
use std::sync::{
	atomic::{AtomicBool, Ordering},
	Arc,
};

mod bat;
mod config;
mod notif;

struct App {
	bats: Bats,
	state: BatState,
	notif: Notif,
}

impl App {
	fn init() -> color_eyre::Result<App> {
		let mut bats = Bats::init()?;
		let state = bats.state();
		let notif = Notif::new();

		Ok(App { bats, state, notif })
	}

	fn run(&mut self, config: &Config) -> color_eyre::Result<()> {
		let should_term = Arc::new(AtomicBool::new(false));
		let should_term_ctrlc = Arc::clone(&should_term);
		let (mut timer, canceller) = cancellable_timer::Timer::new2()?;

		ctrlc::set_handler(move || {
			should_term_ctrlc.store(true, Ordering::Relaxed);
			let _ = canceller.cancel();
		})?;

		let interval = config.interval();
		while !should_term.load(Ordering::Relaxed) {
			self.once(config);
			timer.sleep(interval)?;
		}

		Ok(())
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
		if level <= config.low_pct && self.state != BatState::Charging {
			self.notif.low();
		}
	}
}

fn main() -> color_eyre::Result<()> {
	color_eyre::install()?;

	let mut app = App::init()?;
	let config = Config::init()?;
	app.run(&config)?;

	Ok(())
}
