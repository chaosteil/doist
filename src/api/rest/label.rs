use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DefaultOnError};

use crate::api::Color;

/// LabelID specifies the unique ID of a [`Label`].
pub type LabelID = usize;

/// Label is a tag associated with a Task. Marked with `@name` in the UI.
///
/// Taken from the [Developer Documentation](https://developer.todoist.com/rest/v1/#labels).
#[serde_as]
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct Label {
    /// Unique ID of a label.
    pub id: LabelID,
    /// Name of the label. Written as `@name` in the UI.
    pub name: String,
    /// The display color of the label as given from the API.
    #[serde_as(deserialize_as = "DefaultOnError")]
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

/// FullLabel shows label including ID
pub struct FullLabel<'a>(pub &'a Label);

impl<'a> std::fmt::Display for FullLabel<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.0.id.bright_yellow(), self.0)
    }
}

/// Command used with [`super::Gateway::create_label`] to create a new [`Label`].
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CreateLabel {
    /// Name of the label to create.
    pub name: String,
    /// Order of the label in lists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<isize>,
    /// Color of the label icon.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
    /// Mark as favorite or not.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favorite: Option<bool>,
}

#[cfg(test)]
impl Label {
    /// This is initializer is used for tests, as in general the tool relies on the API and not
    /// local state.
    pub fn new(id: LabelID, name: &str) -> Label {
        Label {
            id,
            name: name.to_string(),
            color: crate::api::Color::Unknown,
            order: 0,
            favorite: false,
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn succeeds_with_bad_color() {
        let label = r#"{"id":123,"name":"hello","color":7,"order":0,"favorite":false}"#;
        assert!(serde_json::from_str::<'_, super::Label>(label).is_ok());
    }
}
