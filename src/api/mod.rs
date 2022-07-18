//! Provides various lower-level mechanisms to interact with the Todoist API.
pub mod rest;
pub mod tree;

mod color;
mod deserialize;
mod sync;

pub use color::*;
