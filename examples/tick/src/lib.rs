use core::sync::atomic::{
	AtomicUsize, Ordering,
};
use plugins_module_rt::*;

mod time;
use time::*;

struct Tick {
	ticks_start: AtomicUsize,
}

impl Module for Tick {
	unsafe fn load(&self) -> bool {
		true
	}
	fn all_loaded(&self, al: AllLoaded<'_>) {
		log::info!(al, "Module ready with token: {al:?}");
		self.ticks_start.store(TIME.ticks(), Ordering::Relaxed);
	}
	unsafe fn tick(&self, al: AllLoaded<'_>) -> bool {
		let times_ticked = TIME.ticks().wrapping_sub(self.ticks_start.load(Ordering::Relaxed));
		log::info!(al, "Module ticked {times_ticked} time(s)");
		if times_ticked >= 10 {
			log::info!(al, "Module tick limit reached, stopping");
			return false
		}

		true
	}
	unsafe fn unload(&self) {}
}

register!(: Tick = Tick {
	ticks_start: AtomicUsize::new(0),
});
