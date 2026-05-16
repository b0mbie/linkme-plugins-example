//! Plugin functions.

use core::ffi::CStr;

/// Trait for exported functions in a binary
/// which are specifically tied to the functionality of this crate.
pub trait PluginFn {
	/// Exported symbol as a [`str`].
	const SYMBOL_STR: &str;
	/// Exported symbol as a [`CStr`].
	const SYMBOL_C_STR: &CStr;
}

macro_rules! with_prefixed_str {
	($macro:ident $pre:tt $($whatever:tt)*) => {
		$macro! {
			$pre
			::core::concat! {
				"plugins_core__", $($whatever)*
			}
		}
	};
}

macro_rules! with_prefixed_c_str {
	($macro:ident $pre:tt $($whatever:tt)*) => {
		$macro! {
			$pre
			unsafe {
				::core::ffi::CStr::from_bytes_with_nul_unchecked(
					::core::concat! {
						::core::concat! {
							"plugins_core__", $($whatever)*
						},
						'\0',
					}.as_bytes()
				)
			}
		}
	};
}

macro_rules! identity {
	{() $($whatever:tt)*} => {
		$($whatever)*
	};
}

/// Prefix for symbol names of exported functions as a [`str`].
pub const PREFIX_STR: &str = with_prefixed_str!(identity ());
/// Prefix for symbol names of exported functions as a [`CStr`].
pub const PREFIX_C_STR: &CStr = with_prefixed_c_str!(identity ());

macro_rules! plugin_fn {
	{
		$(#[$attr:meta])*
		$name:ident($($param:tt)*) $(-> $result:ty)?;
		symbol = $symbol:ident;
	} => {
		$(#[$attr])*
		/// # Layout
		/// This type is guaranteed to always be a `repr(transparent)` wrapper over its only public field.
		#[repr(transparent)]
		pub struct $name(pub unsafe extern "C" fn($($param)*) $(-> $result)?);
		impl PluginFn for $name {
			const SYMBOL_STR: &str = with_prefixed_str!(identity () stringify! { $symbol });
			const SYMBOL_C_STR: &CStr = with_prefixed_c_str!(identity () stringify! { $symbol });
		}
		with_prefixed_str!(
			plugin_fn (@define_macro($) $name)
			stringify! { $symbol }
		);
	};

	{(@define_macro($d:tt) $name:ident) $($export_name:tt)*} => {
		/// Prepends `#[unsafe(export_name = "...")]` to the input,
		/// inserting the appropriate symbol name to export
		#[doc = concat! {"[`", stringify! { $name }, "`]."}]
		#[macro_export]
		macro_rules! $name {
			{$d ($d whatever:tt)*} => {
				#[unsafe(export_name = $($export_name)*)]
				$d ($d whatever)*
			};
		}
	};
}

#[cfg(doc)]
use crate::Plugin;

plugin_fn! {
	/// See [`Plugin::load`].
	LoadFn() -> bool;
	symbol = Load;
}
plugin_fn! {
	/// See [`Plugin::all_loaded`].
	AllLoadedFn();
	symbol = AllLoaded;
}
plugin_fn! {
	/// See [`Plugin::tick`].
	TickFn() -> bool;
	symbol = Tick;
}
plugin_fn! {
	/// See [`Plugin::unload`].
	UnloadFn();
	symbol = Unload;
}
