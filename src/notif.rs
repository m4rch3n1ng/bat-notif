use notify_rust::{Notification, NotificationHandle, Urgency};

struct PersistentNotif {
	msg: &'static str,
	hnd: Option<NotificationHandle>,
}

impl PersistentNotif {
	fn new(msg: &'static str) -> Self {
		PersistentNotif { msg, hnd: None }
	}

	fn show(&mut self) {
		if self.hnd.is_none() {
			self.hnd = Notification::default()
				.summary(self.msg)
				.urgency(Urgency::Critical)
				.show()
				.ok();
		}
	}

	fn close(&mut self) {
		if let Some(hnd) = self.hnd.take() {
			hnd.close();
		}
	}
}

impl Drop for PersistentNotif {
	fn drop(&mut self) {
		self.close();
	}
}

#[derive(Default)]
struct SharedNotif(Option<NotificationHandle>);

impl SharedNotif {
	fn show(&mut self, msg: &str) {
		self.close();
		self.0 = Notification::default()
			.summary(msg)
			.urgency(Urgency::Normal)
			.show()
			.ok();
	}

	fn close(&mut self) {
		if let Some(hnd) = self.0.take() {
			hnd.close();
		}
	}
}

impl Drop for SharedNotif {
	fn drop(&mut self) {
		self.close();
	}
}

pub struct Notif {
	low_notif: PersistentNotif,
	stat_notif: SharedNotif,
}

impl Notif {
	pub fn new() -> Self {
		let low_notif = PersistentNotif::new("battery low");
		let stat_notif = SharedNotif::default();
		Notif {
			low_notif,
			stat_notif,
		}
	}

	pub fn discharging(&mut self) {
		self.stat_notif.show("battery now discharging");
	}

	pub fn charging(&mut self) {
		self.low_notif.close();
		self.stat_notif.show("battery now charging");
	}

	pub fn low(&mut self) {
		self.low_notif.show();
	}
}
