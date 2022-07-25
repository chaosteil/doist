use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};

use crate::api::Color;

/// LabelID specifies the unique ID of a [`Label`].
pub type LabelID = usize;

/// Label is a tag associated with a Task. Marked with `@name` in the UI.
///
/// Taken from the [Developer Documentation](https://developer.todoist.com/rest/v1/#labels).
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Label {
    /// Unique ID of a label.
    pub id: LabelID,
    /// Name of the label. Written as `@name` in the UI.
    pub name: String,
    /// The display color of the label as given from the API.
    pub color: Color,
    /// The order among labels if we were to sort them.
    pub order: isize,
    /// Toggle for marking a label as a favorite.
    pub favorite: bool,
}

impl Ord for Label {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.order.cmp(&other.order) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for Label {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::fmt::Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!("@{}", self.name).bright_blue().fmt(f)
    }
}
