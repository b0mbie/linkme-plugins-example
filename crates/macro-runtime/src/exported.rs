use crate::detail::*;

struct PluginRt;
impl plugins_core::Plugin for PluginRt {
	unsafe fn load(&mut self) -> bool {
		for f in LOAD {
			let did_load = unsafe { f() };
			if !did_load {
				return false
			}
		}
		true
	}
	fn all_loaded(&mut self) {
		for f in ALL_LOADED {
			unsafe { f() }
		}
	}
	unsafe fn unload(&mut self) {
		for f in UNLOAD {
			unsafe { f() }
		}
	}
}
plugins_core::export!(: PluginRt = PluginRt);
