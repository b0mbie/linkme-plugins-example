use crate::{
	detail::MODULES,
	AllLoaded,
};

pub(crate) struct PluginRt;
impl plugins_core::Plugin for PluginRt {
	unsafe fn load(&mut self) -> bool {
		for mo in MODULES {
			let did_load = unsafe { (mo.load)(mo.this) };
			if !did_load {
				return false
			}
		}
		true
	}
	unsafe fn all_loaded(&mut self) {
		let token = unsafe { AllLoaded::forge() };
		for mo in MODULES {
			unsafe { (mo.all_loaded)(mo.this, token) }
		}
	}
	unsafe fn tick(&self) -> bool {
		let token = unsafe { AllLoaded::forge() };
		for mo in MODULES {
			unsafe { (mo.before_tick)(mo.this, token) }
		}

		let mut should_continue = false;
		for mo in MODULES {
			should_continue |= unsafe { (mo.tick)(mo.this, token) }
		}
		should_continue
	}
	unsafe fn unload(&mut self) {
		for mo in MODULES {
			unsafe { (mo.unload)(mo.this) }
		}
	}
}
plugins_core::export!(: PluginRt = PluginRt);
