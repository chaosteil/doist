use serde::{Deserialize, Serialize};

use crate::api::serialize::todoist_rfc3339;

use super::{ProjectID, TaskID};

/// CommentID describes the unique ID of a [`Comment`].
pub type CommentID = String;

/// ThreadID is the ID of the location where the comment is posted.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ThreadID {
    /// The ID of the project this comment is attached to.
    Project {
        /// The ID of the [`super::Project`].
        project_id: ProjectID,
    },
    /// The ID of the task this comment is attached to.
    Task {
        /// The ID of the [`super::Task`].
        task_id: TaskID,
    },
}

/// Comment describes a Comment from the Todoist API.
///
/// Taken from the [Developer Documentation](https://developer.todoist.com/rest/v2/#comments)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Comment {
    /// The unique ID of a comment.
    pub id: CommentID,
    /// Where the comment is attached to.
    #[serde(flatten)]
    pub thread: ThreadID,
    /// The date when the comment was posted.
    #[serde(serialize_with = "todoist_rfc3339")]
    pub posted_at: chrono::DateTime<chrono::Utc>,
    /// Contains the comment text with markdown.
    pub content: String,
    /// Optional attachment file description.
    pub attachment: Option<Attachment>,
}

/// An optional attachment file attached to a comment.
/// TODO: empty for now, so it acts as a marker.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Attachment {}

/// CreateComment allows to create a new comment through the API.
#[derive(Debug, Serialize)]
pub struct CreateComment {
    /// The thread to attach the comment to.
    #[serde(flatten)]
    pub thread: ThreadID,
    /// The text of the comment. Supports markdown.
    pub content: String,
    // TODO: pub attachment: Option<Attachment>,
}
