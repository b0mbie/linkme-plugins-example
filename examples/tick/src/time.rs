use core::sync::atomic::{
	AtomicUsize, Ordering,
};
use plugins_module_rt::*;

pub struct Timekeeping {
	ticks: AtomicUsize,
}
register! {
	pub static TIME: Timekeeping = Timekeeping {
		ticks: AtomicUsize::new(0)
	};
}
impl Module for Timekeeping {
	unsafe fn load(&self) -> bool {
		self.ticks.store(0, Ordering::Relaxed);
		true
	}
	unsafe fn unload(&self) {}
	unsafe fn before_tick(&self, _: AllLoaded<'_>) {
		self.ticks.fetch_add(1, Ordering::Relaxed);
	}
}

impl Timekeeping {
	pub fn ticks(&self) -> usize {
		self.ticks.load(Ordering::Relaxed)
	}
}
