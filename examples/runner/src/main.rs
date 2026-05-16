use libloading::{Library, Symbol, Error as LoadError};
use plugins_core::funcs::*;

trait LibraryExt {
	unsafe fn get_fn<F: PluginFn>(&self) -> Result<Symbol<'_, F>, LoadError>;
}
impl LibraryExt for Library {
	unsafe fn get_fn<F: PluginFn>(&self) -> Result<Symbol<'_, F>, LoadError> {
		unsafe { self.get(F::SYMBOL_C_STR) }
	}
}

fn main() {
	let binary_path = std::env::args_os().nth(1).expect("expected path to plugin binary");
	
	let plugin = unsafe { Library::new(binary_path) }.expect("failed to load plugin binary");
	{
		let load = unsafe { plugin.get_fn::<LoadFn>() }.expect("could not find `LoadFn`");
		let all_loaded = unsafe { plugin.get_fn::<AllLoadedFn>() }.expect("could not find `AllLoadedFn`");
		let unload = unsafe { plugin.get_fn::<UnloadFn>() }.expect("could not find `UnloadFn`");

		println!("Loading plugin");
		assert!(unsafe { (load.0)() }, "failed to load plugin");

		println!("Notifying plugin that everything has loaded");
		unsafe { (all_loaded.0)() };

		println!("Unloading plugin");
		unsafe { (unload.0)() };
	}
}
