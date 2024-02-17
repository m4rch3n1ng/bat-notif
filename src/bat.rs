use battery::{units::ratio::percent, Manager};
use color_eyre::eyre::ensure;
use std::{fs, path::PathBuf};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BatState {
	Discharging,
	/// charging or plugged
	Charging,
	Unknown,
}

pub struct Bats {
	manager: battery::Manager,
	bats: Vec<battery::Battery>,
	adapter: Option<Adapter>,
}

impl Bats {
	pub fn init() -> color_eyre::Result<Bats> {
		let manager = Manager::new()?;
		let bats = manager
			.batteries()?
			.filter_map(Result::ok)
			.collect::<Vec<_>>();
		ensure!(!bats.is_empty(), "no batteries detected");

		let adapter = Adapter::create();

		Ok(Bats {
			manager,
			bats,
			adapter,
		})
	}

	fn update(&mut self) {
		for bat in &mut self.bats {
			self.manager.refresh(bat).unwrap();
		}
	}

	pub fn state(&mut self) -> BatState {
		self.update();

		if self
			.bats
			.iter()
			.any(|bat| bat.state() == battery::State::Charging)
		{
			BatState::Charging
		} else if self.online() {
			// this app treats charging
			// and plugged the same
			BatState::Charging
		} else if self
			.bats
			.iter()
			.any(|bat| bat.state() == battery::State::Discharging)
		{
			BatState::Discharging
		} else {
			BatState::Unknown
		}
	}

	fn online(&self) -> bool {
		if let Some(adapter) = &self.adapter {
			adapter.online()
		} else {
			false
		}
	}

	pub fn level(&mut self) -> usize {
		let bats = self
			.bats
			.iter()
			.map(|bat| bat.state_of_charge().get::<percent>())
			.map(|bat| bat.clamp(0.0, 100.0) as usize);

		let amt = bats.len();
		let sum = bats.sum::<usize>();

		sum / amt
	}
}

const POWER_DIRS: &str = "/sys/class/power_supply/";

struct Adapter(PathBuf);

impl Adapter {
	fn create() -> Option<Adapter> {
		let path = PathBuf::from(POWER_DIRS)
			.read_dir()
			.unwrap()
			.flatten()
			.find_map(|dir| {
				let path = dir.path();
				let r#type = path.join("type");
				let r#type = fs::read_to_string(r#type).ok()?;
				if r#type.trim().to_lowercase() != "mains" {
					return None
				}

				path.join("online").exists().then_some(path)
			})?;

		let adapter = Adapter(path);
		Some(adapter)
	}

	fn online(&self) -> bool {
		let path = self.0.join("online");
		let file = fs::read_to_string(path);
		let Ok(file) = file else { return false };

		let content = file.trim().parse::<u8>();
		let Ok(content) = content else { return false };

		content != 0
	}
}
