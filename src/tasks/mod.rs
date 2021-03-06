//! Controls things that work with [`crate::api::rest::Task`]s.
pub mod add;
pub mod close;
pub mod comment;
pub mod edit;
mod filter;
mod fuzz_select;
mod label;
pub mod list;
mod priority;
mod project;
mod section;
pub mod view;

pub use priority::*;
