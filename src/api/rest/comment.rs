use serde::{Deserialize, Serialize};

use super::{timestamp::todoist_rfc3339, ProjectID, TaskID};

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
