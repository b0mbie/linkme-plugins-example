//! Core traits, types and macros for an example plugin runtime.

#![no_std]

pub mod funcs;

mod export;
pub use export::*;
