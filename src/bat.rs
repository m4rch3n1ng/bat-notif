use battery::{units::ratio::percent, Manager};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BatState {
	Discharging,
	Charging,
	Unknown,
}

pub struct Bats {
	manager: battery::Manager,
	bats: Vec<battery::Battery>,
}

impl Bats {
	pub fn init() -> Bats {
		let manager = Manager::new().unwrap();
		let bats = manager
			.batteries()
			.unwrap()
			.filter_map(Result::ok)
			.collect::<Vec<_>>();
		assert!(!bats.is_empty(), "no batteries detected");

		Bats { manager, bats }
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
