//! # Rust Basic
//!
//! `RustBasic` is a planned development that aims to make Rust easy to learn, teach, and use.

// rustbasic - lib.rs

pub use rustbasic_macro::*;

pub mod macros;
pub mod rename;
pub mod stopwatch;

pub use crate::macros::*;
pub use crate::rename::*;
pub use crate::stopwatch::*;
