//! Controls things that work with [`crate::api::rest::Task`]s.
pub mod add;
pub mod close;
pub mod edit;
pub mod list;
mod priority;

pub use priority::*;
