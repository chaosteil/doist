//! doist is a command line tool to manage tasks on Todoist.
//!
//! This command line tool allows you to interact with the Todoist API using an ergonomic interface
//! to quickly manage tasks from the terminal.
//!
//! # Examples
//! ```bash
//! $ doist list
//! $ doist add "buy some flowers" -d tomorrow
//! ```
#![warn(missing_docs)]
pub mod api;
mod command;
mod comments;
mod config;
mod fuzz_select;
mod interactive;
mod labels;
mod projects;
mod sections;
mod tasks;

use fuzz_select::fuzz_select;

#[doc(hidden)]
pub use command::Args;
