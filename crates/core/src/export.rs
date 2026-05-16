/// Trait for plugins that can be exported with [`export!`](crate::export!).
pub trait Plugin {
	/// Tries to load the plugin,
	/// returning `true` if it was successful.
	/// 
	/// # Safety
	/// This function must not be called again after a previous call which returned `true`.
	unsafe fn load(&mut self) -> bool;
	/// Notifies the plugin that all other plugins have loaded.
	fn all_loaded(&mut self);
	/// Unloads the plugin.
	/// 
	/// # Safety
	/// This function must be paired with *exactly one* previous successful call to [`load`](Plugin::load).
	unsafe fn unload(&mut self);
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
/// # fn all_loaded(&mut self) {}
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

	($($whatever:tt)*) => {
		::core::compile_error! {
			"expected `: <plugin type> = <init>`"
		}
	};
}
