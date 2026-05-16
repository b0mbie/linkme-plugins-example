use crate::AllLoaded;

use core::ptr::NonNull;

/// Trait for modules that can be registered with [`register!`](crate::register!).
/// 
/// # Execution order
/// Modules are placed in memory in an arbitrary, undefined order.
/// However, the order in which module methods are executed is consistent;
/// that is, if one method of some module **A** is called before the implementation of that method of module **B**,
/// then a different method of **A** will also be called *before* the implementation of **B**,
/// and this ordering will be constant throughout the lifetime of the plugin.
pub trait Module {
	/// Tries to load the module,
	/// returning `true` if it was successful.
	/// 
	/// # Safety
	/// This function must not be called again after a previous call which returned `true`.
	unsafe fn load(&self) -> bool;
	/// Unloads the module.
	/// 
	/// # Safety
	/// This function must only be called after a previous successful call to [`load`](Module::load).
	unsafe fn unload(&self);

	/// Notifies the module that all modules in the system have loaded.
	fn all_loaded(&self, al: AllLoaded<'_>) {
		let _ = al;
	}
	/// Runs code that is supposed to run before a "tick".
	/// 
	/// # Safety
	/// This function must be called before [`tick`](Module::tick).
	unsafe fn before_tick(&self, al: AllLoaded<'_>) {
		let _ = al;
	}
	/// Runs a single "tick" of processing on the module,
	/// returning `true` if processing should continue after this call.
	/// 
	/// # Safety
	/// This function must be called after [`before_tick`](Module::before_tick).
	unsafe fn tick(&self, al: AllLoaded<'_>) -> bool {
		let _ = al;
		false
	}
}

pub(crate) mod detail {
	use super::*;

	pub struct ModuleFuncs {
		pub(crate) this: ModuleThis,
		pub(crate) load: unsafe fn(ModuleThis) -> bool,
		pub(crate) unload: unsafe fn(ModuleThis),
		pub(crate) all_loaded: unsafe fn(ModuleThis, AllLoaded<'_>),
		pub(crate) before_tick: unsafe fn(ModuleThis, AllLoaded<'_>),
		pub(crate) tick: unsafe fn(ModuleThis, AllLoaded<'_>) -> bool,
	}
	unsafe impl Sync for ModuleFuncs {}
	pub type ModuleThis = core::ptr::NonNull<()>;

	impl ModuleFuncs {
		pub const fn new<M: Module>(this: &'static M) -> Self {
			let this = NonNull::from_ref(this).cast();
			Self {
				this,
				load: Self::load::<M>,
				unload: Self::unload::<M>,
				all_loaded: Self::all_loaded::<M>,
				before_tick: Self::before_tick::<M>,
				tick: Self::tick::<M>,
			}
		}

		unsafe fn load<M: Module>(this: ModuleThis) -> bool {
			let mo = unsafe { this.cast::<M>().as_ref() };
			unsafe { mo.load() }
		}

		unsafe fn all_loaded<M: Module>(this: ModuleThis, al: AllLoaded<'_>) {
			let mo = unsafe { this.cast::<M>().as_ref() };
			mo.all_loaded(al)
		}

		unsafe fn before_tick<M: Module>(this: ModuleThis, al: AllLoaded<'_>) {
			let mo = unsafe { this.cast::<M>().as_ref() };
			unsafe { mo.before_tick(al) }
		}

		unsafe fn tick<M: Module>(this: ModuleThis, al: AllLoaded<'_>) -> bool {
			let mo = unsafe { this.cast::<M>().as_ref() };
			unsafe { mo.tick(al) }
		}

		unsafe fn unload<M: Module>(this: ModuleThis) {
			let mo = unsafe { this.cast::<M>().as_ref() };
			unsafe { mo.unload() }
		}
	}
}
