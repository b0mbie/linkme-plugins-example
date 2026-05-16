use core::marker::PhantomData;

/// Token for threads that are running after the binary's exported plugin has loaded.
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct AllLoaded<'s> {
	_life: PhantomData<&'s ()>,
	_not_send_sync: PhantomData<*mut ()>,
}

impl<'s> AllLoaded<'s> {
	/// Forges an "all-loaded" token.
	/// 
	/// # Safety
	/// This function must only be called after the
	/// [`Plugin::load`](plugins_core::Plugin::load) method
	/// of [`crate::exported::PluginRt`]
	/// returns.
	pub(crate) const unsafe fn forge() -> Self {
		Self {
			_life: PhantomData,
			_not_send_sync: PhantomData,
		}
	}
}
