//! Abstraction over [`plugins_core`] that allows defining plugin functionality with macros.
//! 
//! Internally, this crate uses [`linkme`] to collect functions defined by the macros,
//! and exports a plugin that calls the appropriate functions.

#![no_std]

/// Implementation detail for linking several instances of something together into the runtime.
/// Only let macros use this!
#[doc(hidden)]
pub mod detail {
	pub use linkme;

	#[linkme::distributed_slice]
	pub static LOAD: [unsafe fn() -> bool];
	#[linkme::distributed_slice]
	pub static ALL_LOADED: [unsafe fn()];
	#[linkme::distributed_slice]
	pub static UNLOAD: [unsafe fn()];
}

#[cfg(doc)]
use plugins_core::Plugin;

/// See [`Plugin::load`].
#[macro_export]
macro_rules! on_load {
	{() $body:block} => {
		const _: () = {
			#[$crate::detail::linkme::distributed_slice($crate::detail::LOAD)]
			#[linkme(crate = $crate::detail::linkme)]
			unsafe fn plugins_macro_rt_on_load() -> bool $body
		};
	};
	{$($whatever:tt)*} => {
		::core::compile_error! {
			"expected `() <function body>`"
		}
	};
}
/// See [`Plugin::all_loaded`].
#[macro_export]
macro_rules! on_all_loaded {
	{() $body:block} => {
		const _: () = {
			#[$crate::detail::linkme::distributed_slice($crate::detail::ALL_LOADED)]
			#[linkme(crate = $crate::detail::linkme)]
			unsafe fn plugins_macro_rt_on_all_loaded() $body
		};
	};
	{$($whatever:tt)*} => {
		::core::compile_error! {
			"expected `() <function body>`"
		}
	};
}
/// See [`Plugin::unload`].
#[macro_export]
macro_rules! on_unload {
	{() $body:block} => {
		const _: () = {
			#[$crate::detail::linkme::distributed_slice($crate::detail::UNLOAD)]
			#[linkme(crate = $crate::detail::linkme)]
			unsafe fn plugins_macro_rt_on_unload() $body
		};
	};
	{$($whatever:tt)*} => {
		::core::compile_error! {
			"expected `() <function body>`"
		}
	};
}

mod exported;
