use core::fmt;
use std::fmt::Display;

use crate::api::serialize::todoist_rfc3339;
use crate::api::tree::Treeable;
use chrono::{DateTime, FixedOffset, Utc};
use owo_colors::{OwoColorize, Stream};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::{ProjectID, SectionID};

/// TaskID describes the unique ID of a [`Task`].
pub type TaskID = String;
/// UserID is the unique ID of a User.
pub type UserID = String;

/// Task describes a Task from the Todoist API.
///
/// Taken from the [Developer Documentation](https://developer.todoist.com/api/v1#tag/Tasks).
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct Task {
    /// Unique ID of a Task.
    pub id: TaskID,
    /// Shows which [`super::Project`] the Task belongs to.
    pub project_id: ProjectID,
    /// Set if the Task is also in a subsection of a Project.
    pub section_id: Option<SectionID>,
    /// The main content of the Task, also known as Task name.
    pub content: String,
    /// Description is the description found under the content.
    pub description: String,
    /// Completed is set if this task was completed.
    pub checked: bool,
    /// All associated [`super::Label`]s to this Task. Just label names are used here.
    pub labels: Vec<String>,
    /// If set, this Task is a subtask of another.
    pub parent_id: Option<TaskID>,
    /// Order within the subtasks or project of a Task.
    pub child_order: isize,
    /// Priority is how urgent the task is.
    pub priority: Priority,
    /// The due date of the Task.
    pub due: Option<DueDate>,
    /// Links the Task to a URL in the Todoist UI.
    pub url: Url,
    /// How many comments are written for this Task.
    pub note_count: usize,
    /// Who created this task.
    pub user_id: UserID,
    /// Who this task is assigned to.
    pub responsible_uid: Option<UserID>,
    /// Who assigned this task to the [`Task::assignee`]
    pub assigned_by_uid: Option<UserID>,
    /// Exact date when the task was created.
    #[serde(serialize_with = "todoist_rfc3339")]
    pub added_at: DateTime<Utc>,
}

impl Treeable for Task {
    type ID = TaskID;

    fn id(&self) -> TaskID {
        self.id.clone()
    }

    fn parent_id(&self) -> Option<TaskID> {
        self.parent_id.clone()
    }

    fn reset_parent(&mut self) {
        self.parent_id = None;
    }
}

impl Ord for Task {
    /// Sorts on a best-attempt to make it sort similar to the Todoist UI.
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Exact times ignore even priority in the UI
        match (
            self.due
                .as_ref()
                .map(|d| d.exact.as_ref().map(|e| e.datetime))
                .unwrap_or_default(),
            other
                .due
                .as_ref()
                .map(|d| d.exact.as_ref().map(|e| e.datetime))
                .unwrap_or_default(),
        ) {
            (Some(left), Some(right)) => match left.cmp(&right) {
                std::cmp::Ordering::Equal => {}
                ord => return ord,
            },
            (Some(_left), None) => return std::cmp::Ordering::Less,
            (None, Some(_right)) => return std::cmp::Ordering::Greater,
            (None, None) => {}
        }

        // Lower priority in API is lower in list
        match self.priority.cmp(&other.priority).reverse() {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        match self.child_order.cmp(&other.child_order) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Priority as is given from the Todoist API.
///
/// 1 for Normal up to 4 for Urgent.
#[derive(
    Default, Debug, Copy, Clone, Serialize_repr, Deserialize_repr, PartialEq, Eq, PartialOrd, Ord,
)]
#[repr(u8)]
pub enum Priority {
    /// p1 in the Todoist UI.
    #[default]
    Normal = 1,
    /// p3 in the Todoist UI.
    High = 2,
    /// p2 in the Todoist UI.
    VeryHigh = 3,
    /// p1 in the Todoist UI.
    Urgent = 4,
}

impl Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // The priority display is reversed as in the actual desktop client compared to the API.
        match self {
            Priority::Normal => write!(f, "p4"),
            Priority::High => write!(
                f,
                "{}",
                "p3".if_supports_color(Stream::Stdout, |text| text.blue())
            ),
            Priority::VeryHigh => write!(
                f,
                "{}",
                "p2".if_supports_color(Stream::Stdout, |text| text.yellow())
            ),
            Priority::Urgent => write!(
                f,
                "{}",
                "p1".if_supports_color(Stream::Stdout, |text| text.red())
            ),
        }
    }
}

/// ExactTime exists in DueDate if this is an exact DueDate.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct ExactTime {
    /// Exact DateTime for when the task is due.
    pub datetime: DateTime<FixedOffset>,
    /// Timezone string or UTC offset. // TODO: currently will not interpret correctly if it's a UTC offset.
    pub timezone: String,
}

impl Display for ExactTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Ok(tz) = self.timezone.parse::<chrono_tz::Tz>() {
            write!(f, "{}", self.datetime.with_timezone(&tz))
        } else {
            write!(f, "{}", self.datetime)
        }
    }
}

/// DueDate is the Due object from the Todoist API.
///
/// Mostly contains human-readable content for easier display.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct DueDate {
    /// Human-redable form of the due date.
    #[serde(rename = "string")]
    pub string: String,
    /// The date on which the Task is due.
    pub date: chrono::NaiveDate,
    /// Lets us know if it is recurring (reopens after close).
    pub is_recurring: bool,
    /// If set, this shows the exact time the task is due.
    #[serde(flatten)]
    pub exact: Option<ExactTime>,
}

/// Formats a [`DueDate`] using the given [`DateTime`], by coloring the output based on if it's
/// too late or too soon.
pub struct DueDateFormatter<'a>(pub &'a DueDate, pub &'a DateTime<Utc>);

impl Display for DueDateFormatter<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0.is_recurring {
            write!(
                f,
                "{}",
                "[REPEAT] ".if_supports_color(Stream::Stdout, |_| "🔁 ")
            )?;
        }
        if let Some(exact) = &self.0.exact {
            if exact.datetime >= *self.1 {
                write!(
                    f,
                    "{}",
                    exact.if_supports_color(Stream::Stdout, |text| text.bright_green())
                )
            } else {
                write!(
                    f,
                    "{}",
                    exact.if_supports_color(Stream::Stdout, |text| text.bright_red())
                )
            }
        } else if self.0.date >= self.1.date_naive() {
            write!(
                f,
                "{}",
                self.0
                    .string
                    .if_supports_color(Stream::Stdout, |text| text.bright_green())
            )
        } else {
            write!(
                f,
                "{}",
                self.0
                    .string
                    .if_supports_color(Stream::Stdout, |text| text.bright_red())
            )
        }
    }
}

/// Human representation of the due date.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum TaskDue {
    /// Human readable representation of the date.
    #[serde(rename = "due_string")]
    String(String),
    /// Loose target date with no exact time. TODO: should use way to encode it as a type.
    #[serde(rename = "due_date")]
    Date(String),
    /// Exact DateTime in UTC for the due date.
    #[serde(rename = "due_datetime", serialize_with = "todoist_rfc3339")]
    DateTime(DateTime<Utc>),
}
/// Command used with [`super::Gateway::create`] to create a new Task.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CreateTask {
    /// Sets the [`Task::content`] on the new [`Task`].
    pub content: String,
    /// Sets the [`Task::description`] on the new [`Task`].
    pub description: Option<String>,
    /// Sets the [`Task::project_id`] on the new [`Task`].
    pub project_id: Option<ProjectID>,
    /// Sets the [`Task::section_id`] on the new [`Task`].
    pub section_id: Option<SectionID>,
    /// Sets the [`Task::parent_id`] on the new [`Task`].
    pub parent_id: Option<TaskID>,
    /// Sets the [`Task::order`] on the new [`Task`].
    pub order: Option<isize>,
    /// Sets the [`Task::labels`] on the new [`Task`].
    pub labels: Vec<String>,
    /// Sets the [`Task::priority`] on the new [`Task`].
    pub priority: Option<Priority>,
    /// Sets the [`Task::due`] on the new [`Task`].
    #[serde(flatten)]
    pub due: Option<TaskDue>,
    /// If due is [TaskDue::String], this two-letter code optionally specifies the language if it's not english.
    pub due_lang: Option<String>,
    /// Sets the [`Task::assignee`] on the new [`Task`].
    pub assignee: Option<UserID>,
}

/// Command used with [`super::Gateway::update`] to update a [`Task`].
///
/// Each field is optional, so if something exists, that part of the [`Task`] will get overwritten.
#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct UpdateTask {
    /// Overwrites [`Task::content`] if set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Overwrites [`Task::description`] if set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Overwrites [`Task::labels`] if set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    /// Overwrites [`Task::priority`] if set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<Priority>,
    /// Overwrites [`Task::due`] if set.
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub due: Option<TaskDue>,
    /// If due is [TaskDue::String], this two-letter code optionally specifies the language if it's not english.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_lang: Option<String>,
    /// Overwrites [`Task::assignee`] if set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee: Option<UserID>,
}

#[cfg(test)]
impl Task {
    /// This is initializer is used for tests, as in general the tool relies on the API and not
    /// local state.
    pub fn new(id: &str, content: &str) -> Task {
        Task {
            id: id.to_string(),
            project_id: "".to_string(),
            section_id: None,
            content: content.to_string(),
            description: String::new(),
            checked: false,
            labels: Vec::new(),
            parent_id: None,
            child_order: 0,
            priority: Priority::default(),
            due: None,
            url: "http://localhost".to_string().parse().unwrap(),
            note_count: 0,
            user_id: "0".to_string(),
            responsible_uid: None,
            assigned_by_uid: None,
            added_at: Utc::now(),
        }
    }
}
