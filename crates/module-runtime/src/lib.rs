//! Abstraction over [`plugins_core`] that allows defining plugin functionality with modules.
//! 
//! Internally, this crate uses [`linkme`] to collect all modules in a binary,
//! and exports a plugin that calls methods of the modules.

#![no_std]

/// Implementation detail for linking several instances of something together into the runtime.
/// This is intended for usage by macros only!
#[doc(hidden)]
pub mod detail {
	pub use crate::module::detail::ModuleFuncs;

	pub use linkme;

	#[linkme::distributed_slice]
	pub static MODULES: [ModuleFuncs];
}

mod all_loaded;
pub use all_loaded::*;
mod module;
pub use module::*;

mod exported;

/// Registers into the plugin the given type which implements [`Module`].
#[macro_export]
macro_rules! register {
	(: $Module:ty = $($init:tt)*) => {
		const _: () = {
			$crate::register! {
				static REGISTERED_MODULE_IMPL: $Module = $($init)*;
			}
		};
	};

	{
		$(#[$attr:meta])*
		$vis:vis static $STATIC:ident: $Module:ty = $init:expr;
	} => {
		$(#[$attr])*
		$vis static $STATIC: $Module = $init;
		const _: () = {
			#[$crate::detail::linkme::distributed_slice($crate::detail::MODULES)]
			#[linkme(crate = $crate::detail::linkme)]
			static REGISTERED_MODULE: $crate::detail::ModuleFuncs = $crate::detail::ModuleFuncs::new(&$STATIC);
		};
	};

	{$($whatever:tt)*} => {
		::core::compile_error! {
			"expected `[<visibility> static <name>]: <module type> = <initializer>`"
		}
	};
}
