//! todoist is a command line tool to manage tasks on Todoist.
//!
//! This command line tool allows you to interact with the Todoist API using an ergonomic interface
//! to quickly manage tasks from the terminal.
//!
//! # Examples
//! ```bash
//! $ todoist list
//! $ todoist add "buy some flowers tomorrow"
//! ```
mod api;
mod close;
mod command;
mod config;
mod list;

#[doc(hidden)]
pub use command::Args;
