/// Trait for plugins that can be exported with [`export!`](crate::export!).
pub trait Plugin {
	/// Tries to load the plugin,
	/// returning `true` if it was successful.
	/// 
	/// # Safety
	/// This function must not be called again after a previous call which returned `true`.
	unsafe fn load(&mut self) -> bool;
	/// Unloads the plugin.
	/// 
	/// # Safety
	/// This function must only be called after a previous successful call to [`load`](Plugin::load).
	unsafe fn unload(&mut self);

	/// Notifies the plugin that all dependencies have loaded.
	/// 
	/// # Safety
	/// This function must be called after a previous successful call to [`load`](Plugin::load).
	unsafe fn all_loaded(&mut self);
	/// Runs a single "tick" of processing on the plugin,
	/// returning `true` if processing should continue after this call.
	/// 
	/// # Safety
	/// This function must be called after a previous call to [`all_loaded`](Plugin::all_loaded).
	unsafe fn tick(&self) -> bool;
}

/// Exports the given type which implements [`Plugin`].
/// 
/// # Examples
/// ```
/// # use plugins_core::{export, Plugin};
/// struct MyPlugin {}
/// impl Plugin for MyPlugin {
///     unsafe fn load(&mut self) -> bool { true }
///     // ...
/// # unsafe fn all_loaded(&mut self) {}
/// # unsafe fn tick(&mut self) -> bool {false}
/// # unsafe fn unload(&mut self) {}
/// }
/// export!(: MyPlugin = MyPlugin {});
/// ```
#[macro_export]
macro_rules! export {
	(: $Plugin:ty = $($init:tt)*) => {
		const _: () = {
			static mut PLUGIN: $Plugin = $($init)*;
			$crate::export! {
				@fn(LoadFn) plugins_core_export_load() -> bool {
					unsafe { $crate::Plugin::load(&mut PLUGIN) }
				}
			}
			$crate::export! {
				@fn(AllLoadedFn) plugins_core_export_all_loaded() {
					unsafe { $crate::Plugin::all_loaded(&mut PLUGIN) }
				}
			}
			$crate::export! {
				@fn(TickFn) plugins_core_export_tick() -> bool {
					unsafe { $crate::Plugin::tick(&mut PLUGIN) }
				}
			}
			$crate::export! {
				@fn(UnloadFn) plugins_core_export_unload() {
					unsafe { $crate::Plugin::unload(&mut PLUGIN) }
				}
			}
		};
	};

	{@fn($Func:ident) $name:ident($($params:tt)*) $(-> $return:ty)? $body:block} => {
		$crate::$Func! {
			unsafe extern "C" fn $name($($params)*) $(-> $return)? $body
		}
		const _: $crate::funcs::$Func = $crate::funcs::$Func($name);
	};

	{$($whatever:tt)*} => {
		::core::compile_error! {
			"expected `: <plugin type> = <initializer>`"
		}
	};
}
