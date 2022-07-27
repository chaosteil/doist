//! Controls things that work with [`crate::api::rest::Task`]s.
pub mod add;
pub mod close;
pub mod edit;
mod fuzz_select;
pub mod list;
mod priority;
mod project;

pub use priority::*;
