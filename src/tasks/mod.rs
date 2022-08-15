//! Controls things that work with [`crate::api::rest::Task`]s.
pub mod add;
pub mod close;
pub mod comment;
pub mod edit;
mod filter;
pub mod list;
mod priority;
pub mod view;

pub use priority::*;
