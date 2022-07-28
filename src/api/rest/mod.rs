//! Contains low-level structs as used by the Todoist REST API and provides some tools to work
//! with them.
//!
//! This maps parts of the [API Documentation](https://developer.todoist.com/rest/v1/#overview) to
//! code that can be consumed by clients, including the actual network calls and
//! serialization/deserialization..
//!
//! To get started, take a look at [`Gateway`].
mod comment;
mod gateway;
mod label;
mod project;
mod section;
mod task;
mod timestamp;

pub use comment::*;
pub use gateway::*;
pub use label::*;
pub use project::*;
pub use section::*;
pub use task::*;
