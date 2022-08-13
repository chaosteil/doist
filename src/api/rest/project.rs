use crate::api::deserialize::deserialize_zero_to_none;
use crate::api::{tree::Treeable, Color};
use owo_colors::OwoColorize;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DefaultOnError};

/// ProjectID is the unique ID of a [`Project`]
pub type ProjectID = u64;
/// ProjectSyncID is an identifier to mark between copies of shared projects.
pub type ProjectSyncID = u64;

/// Project as described by the Todoist API.
///
/// Taken from the [Developer Documentation](https://developer.todoist.com/rest/v1/#projects).
#[serde_as]
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone)]
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
    #[serde_as(deserialize_as = "DefaultOnError")]
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
    type ID = ProjectID;

    fn id(&self) -> ProjectID {
        self.id
    }

    fn parent_id(&self) -> Option<ProjectID> {
        self.parent_id
    }

    fn reset_parent(&mut self) {
        self.parent_id = None;
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

/// Command used with [`super::Gateway::create_project`] to create a new [`Projectk`].
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CreateProject {
    /// Name of the project to create.
    pub name: String,
    /// Makes the newly created project a child of this parent project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<ProjectID>,
    /// Color of the project icon.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
    /// Mark as favorite or not.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favorite: Option<bool>,
}

#[cfg(test)]
impl Project {
    /// This is initializer is used for tests, as in general the tool relies on the API and not
    /// local state.
    pub fn new(id: ProjectID, name: &str) -> Project {
        Project {
            id,
            name: name.to_string(),
            parent_id: None,
            comment_count: 0,
            color: crate::api::Color::Unknown,
            shared: false,
            order: None,
            inbox_project: None,
            team_inbox: None,
            sync_id: None,
            favorite: false,
            url: "http://localhost".to_string().parse().unwrap(),
        }
    }
}
