use plugins_module_rt::*;
use std::{
	fmt,
	io::{Stdout, Write, stdout},
};

pub struct LoggerModule;
impl Module for LoggerModule {
	unsafe fn load(&self) -> bool {
		true
	}
	unsafe fn unload(&self) {}
}

register! {
	pub static LOGGER: LoggerModule = LoggerModule;
}

impl LoggerModule {
	pub fn info<'al>(&self, al: AllLoaded<'al>) -> Writer<'al> {
		Writer {
			stdout: stdout(),
			_al: al,
		}
	}
}

pub struct Writer<'al> {
	stdout: Stdout,
	_al: AllLoaded<'al>,
}
impl fmt::Write for Writer<'_> {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		self.stdout.write_all(s.as_bytes()).map_err(|_| fmt::Error)
	}
	fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> fmt::Result {
		self.stdout.write_fmt(args).map_err(|_| fmt::Error)
	}
}

#[macro_export]
macro_rules! info {
	($al:expr $(, $($arg:tt)+)?) => {{
		use ::core::fmt::Write as _;
		let _ = ::core::writeln!(
			$crate::LOGGER.info($al)
			$(, $($arg)+)?
		);
	}};
}
