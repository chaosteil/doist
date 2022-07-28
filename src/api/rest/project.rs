use crate::api::deserialize::deserialize_zero_to_none;
use crate::api::{tree::Treeable, Color};
use owo_colors::OwoColorize;
use reqwest::Url;
use serde::{Deserialize, Serialize};

/// ProjectID is the unique ID of a [`Project`]
pub type ProjectID = u64;
/// ProjectSyncID is an identifier to mark between copies of shared projects.
pub type ProjectSyncID = u64;

/// Project as described by the Todoist API.
///
/// Taken from the [Developer Documentation](https://developer.todoist.com/rest/v1/#projects).
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd)]
pub struct Project {
    /// ID of the Project.
    pub id: ProjectID,
    /// The direct parent of the project if it exists.
    pub parent_id: Option<ProjectID>,
    /// The name of the Project. Displayed in the project list in the UI.
    pub name: String,
    /// How many project comments.
    pub comment_count: usize,
    /// Color as used by the Todoist UI.
    pub color: Color,
    /// Whether the project is shared with someone else.
    pub shared: bool,
    /// Project order under the same parent.
    pub order: Option<usize>,
    /// This marks the project as the initial Inbox project if it exists.
    pub inbox_project: Option<bool>,
    /// This markes the project as a TeamInbox project if it exists.
    pub team_inbox: Option<bool>,
    /// Identifier to match between different copies of shared projects.
    #[serde(deserialize_with = "deserialize_zero_to_none")]
    pub sync_id: Option<ProjectSyncID>,
    /// Toggle to mark this project as a favorite.
    pub favorite: bool,
    /// URL to the Todoist UI.
    pub url: Url,
}

impl Treeable for Project {
    fn id(&self) -> u64 {
        self.id
    }

    fn parent_id(&self) -> Option<u64> {
        self.parent_id
    }
}

impl std::fmt::Display for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}",
            self.id.bright_yellow(),
            self.name.default_color()
        )
    }
}
