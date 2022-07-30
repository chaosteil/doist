use serde::{Deserialize, Serialize};

use crate::api::serialize::todoist_rfc3339;
use owo_colors::OwoColorize;

use super::{ProjectID, TaskID};

/// CommentID describes the unique ID of a [`Comment`].
type CommentID = u64;

/// Comment describes a Comment from the Todoist API.
///
/// Taken from the [Developer Documentation](https://developer.todoist.com/rest/v1/#comments)
#[derive(Debug, Serialize, Deserialize)]
pub struct Comment {
    /// The unique ID of a comment.
    pub id: CommentID,
    /// The ID of the project this comment is attached to.
    pub project_id: Option<ProjectID>,
    /// The ID of the task this comment is attached to.
    pub task_id: Option<TaskID>,
    /// The date when the comment was posted.
    #[serde(serialize_with = "todoist_rfc3339")]
    pub posted: chrono::DateTime<chrono::Utc>,
    /// Contains the comment text with markdown.
    pub content: String,
    /// Optional attachment file description.
    pub attachment: Option<Attachment>,
}

/// An optional attachment file attached to a comment.
/// TODO: empty for now, so it acts as a marker.
#[derive(Debug, Serialize, Deserialize)]
pub struct Attachment {}

/// FullComment allows to display full comment metadata when [std::fmt::Display]ing it.
pub struct FullComment<'a>(pub &'a Comment);

impl std::fmt::Display for FullComment<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let FullComment(comment) = self;
        writeln!(f, "ID: {}", comment.id.bright_yellow())?;
        writeln!(f, "Posted: {}", comment.posted)?;
        writeln!(
            f,
            "Attachment: {}",
            if comment.attachment.is_some() {
                "Yes"
            } else {
                "No"
            }
        )?;
        write!(f, "Content: {}", comment.content)?;
        Ok(())
    }
}
