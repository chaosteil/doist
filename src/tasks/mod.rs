//! Controls things that work with [`crate::api::rest::Task`]s.
pub mod add;
pub mod close;
pub mod comment;
pub mod create;
pub mod edit;
mod filter;
pub mod list;
mod priority;
mod state;
pub mod view;

pub use priority::*;
