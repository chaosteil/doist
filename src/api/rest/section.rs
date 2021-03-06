use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};

use super::ProjectID;

/// SectionID is the unique ID of a [`Section`].
pub type SectionID = usize;

/// Section describes a subsection of a [`super::Project`].
///
/// Taken from the [Developer Documentation](https://developer.todoist.com/rest/v1/#sections).
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Section {
    /// The unique ID of this section.
    pub id: SectionID,
    /// Project ID that this section belongs to.
    pub project_id: ProjectID,
    /// Position of the section amonst sections from the same project.
    pub order: isize,
    /// The actual name of the section.
    pub name: String,
}

impl Ord for Section {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.order.cmp(&other.order) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for Section {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::fmt::Display for Section {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}",
            self.id.bright_yellow(),
            self.name.default_color()
        )
    }
}
