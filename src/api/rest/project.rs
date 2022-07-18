use crate::api::deserialize::deserialize_zero_to_none;
use crate::api::{tree::Treeable, Color};
use owo_colors::OwoColorize;
use reqwest::Url;
use serde::{Deserialize, Serialize};

pub type ProjectID = usize;
pub type ProjectSyncID = usize;

/// Project as described by the Todoist API.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd)]
pub struct Project {
    pub id: ProjectID,
    pub parent_id: Option<ProjectID>,
    pub name: String,
    pub comment_count: usize,
    pub color: Color,
    pub shared: bool,
    pub order: Option<usize>,
    pub inbox_project: Option<bool>,
    pub team_inbox: Option<bool>,
    #[serde(deserialize_with = "deserialize_zero_to_none")]
    pub sync_id: Option<ProjectSyncID>,
    pub favorite: bool,
    pub url: Url,
}

impl Treeable for Project {
    fn id(&self) -> usize {
        self.id
    }

    fn parent_id(&self) -> Option<usize> {
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
