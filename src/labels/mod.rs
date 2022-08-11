pub mod add;
pub mod delete;
mod label;
///! Controls things that work with [`crate::api::rest::Label`]s.
pub mod list;
pub use label::{LabelSelect, Selection};
