use core::fmt;
use std::fmt::Display;

use crate::api::tree::Treeable;
use crate::api::{deserialize::deserialize_zero_to_none, serialize::todoist_rfc3339, tree::Tree};
use owo_colors::OwoColorize;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::Label;
use super::{LabelID, Project, ProjectID, Section, SectionID};

/// TaskID describes the unique ID of a [`Task`].
pub type TaskID = u64;
/// UserID is the unique ID of a User.
pub type UserID = u64;

/// Priority as is given from the Todoist API.
///
/// 1 for Normal up to 4 for Urgent.
#[derive(Debug, Copy, Clone, Serialize_repr, Deserialize_repr, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Priority {
    /// p1 in the Todoist UI.
    Normal = 1,
    /// p2 in the Todoist UI.
    High = 2,
    /// p3 in the Todoist UI.
    VeryHigh = 3,
    /// p4 in the Todoist UI.
    Urgent = 4,
}

impl Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // The priority display is reversed as in the actual desktop client compared to the API.
        match self {
            Priority::Normal => write!(f, "{}", "p4".default_color()),
            Priority::High => write!(f, "{}", "p3".blue()),
            Priority::VeryHigh => write!(f, "{}", "p2".yellow()),
            Priority::Urgent => write!(f, "{}", "p1".red()),
        }
    }
}

impl Default for Priority {
    fn default() -> Self {
        Priority::Normal
    }
}

/// Task describes a Task from the Todoist API.
///
/// Taken from the [Developer Documentation](https://developer.todoist.com/rest/v1/#tasks).
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Task {
    /// Unique ID of a Task.
    pub id: TaskID,
    /// Shows which [`Project`] the Task belongs to.
    pub project_id: ProjectID,
    /// Set if the Task is also in a subsection of a Project.
    #[serde(deserialize_with = "deserialize_zero_to_none")]
    pub section_id: Option<SectionID>,
    /// The main content of the Task, also known as Task name.
    pub content: String,
    /// Description is the description found under the content.
    pub description: String,
    /// Completed is set if this task was completed.
    pub completed: bool,
    /// All associated [`Label`]s to this Task.
    pub label_ids: Vec<LabelID>,
    /// If set, this Task is a subtask of another.
    pub parent_id: Option<TaskID>,
    /// Order the order within the subtasks of a Task.
    pub order: isize,
    /// Priority is how urgent the task is.
    pub priority: Priority,
    /// The due date of the Task.
    pub due: Option<DueDate>,
    /// Links the Task to a URL in the Todoist UI.
    pub url: Url,
    /// How many comments are written for this Task.
    pub comment_count: usize,
    /// Who this task is assigned to.
    pub assignee: Option<UserID>,
    /// Who assigned this task to the [`Task::assignee`]
    #[serde(deserialize_with = "deserialize_zero_to_none")]
    pub assigner: Option<UserID>,
    /// Exact date when the task was created.
    #[serde(serialize_with = "todoist_rfc3339")]
    pub created: chrono::DateTime<chrono::Utc>,
}

impl Treeable for Task {
    type ID = TaskID;

    fn id(&self) -> TaskID {
        self.id
    }

    fn parent_id(&self) -> Option<TaskID> {
        self.parent_id
    }
}

/// Used to display full information about a Task.
pub struct FullTask<'a>(
    pub &'a Task,
    pub Option<&'a Project>,
    pub Option<&'a Section>,
    pub Vec<&'a Label>,
);

impl Display for FullTask<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let FullTask::<'_>(task, project, section, labels) = self;
        write!(
            f,
            "ID: {}\nPriority: {}\nContent: {}\nDescription: {}",
            task.id.bright_yellow(),
            task.priority,
            task.content.default_color(),
            task.description.default_color()
        )?;
        if let Some(due) = &task.due {
            write!(f, "\nDue: {}", due)?;
        }
        if !labels.is_empty() {
            write!(
                f,
                "\nLabels: {}",
                labels
                    .iter()
                    .map(|l| l.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            )?;
        }
        if let Some(project) = &project {
            write!(f, "\nProject: {}", project)?;
        }
        if let Some(section) = &section {
            write!(f, "\nSection: {}", section)?;
        }
        write!(f, "\nComments: {}", task.comment_count)?;
        Ok(())
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
        match self.order.cmp(&other.order) {
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

/// Used to display task as an item in a list.
pub struct TableTask<'a>(
    pub &'a Tree<Task>,
    pub Option<&'a Project>,
    pub Option<&'a Section>,
    pub Vec<&'a Label>,
);

impl TableTask<'_> {
    /// Initializes a TableTask item that only displays data that is directly available from a
    /// [`Task`].
    pub fn from_task(task: &Tree<Task>) -> TableTask {
        TableTask(task, None, None, vec![])
    }
}

impl Display for TableTask<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let TableTask::<'_>(task, project, section, labels) = self;
        let subtask_padding = if task.depth > 0 {
            format!("{}⌞ ", "  ".repeat(task.depth))
        } else {
            "".to_string()
        };
        write!(
            f,
            "{}{} {} {}",
            subtask_padding,
            task.id.bright_yellow(),
            task.priority,
            task.content.default_color(),
        )?;
        if let Some(due) = &task.due {
            write!(f, " {}", due)?;
        }
        if !labels.is_empty() {
            write!(
                f,
                " {}",
                labels
                    .iter()
                    .map(|l| l.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            )?;
        }
        if let Some(p) = &project {
            write!(f, " [{}", p.name)?;
            if let Some(s) = &section {
                write!(f, "/{}", s.name)?;
            }
            write!(f, "]")?;
        }
        Ok(())
    }
}

/// ExactTime exists in DueDate if this is an exact DueDate.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExactTime {
    /// Exact DateTime for when the task is due.
    pub datetime: chrono::DateTime<chrono::FixedOffset>,
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
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct DueDate {
    /// Human-redable form of the due date.
    #[serde(rename = "string")]
    pub human_readable: String,
    /// The date on which the Task is due.
    pub date: chrono::NaiveDate,
    /// Lets us know if it is recurring (reopens after close).
    pub recurring: bool,
    /// If set, this shows the exact time the task is due.
    #[serde(flatten)]
    pub exact: Option<ExactTime>,
}

impl Display for DueDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.recurring {
            write!(f, "🔁 ")?;
        }
        if let Some(exact) = &self.exact {
            if exact.datetime >= chrono::Utc::now() {
                write!(f, "{}", exact.bright_green())
            } else {
                write!(f, "{}", exact.bright_red())
            }
        } else if self.date >= chrono::Utc::now().date().naive_utc() {
            write!(f, "{}", self.human_readable.bright_green())
        } else {
            write!(f, "{}", self.human_readable.bright_red())
        }
    }
}

/// Human representation of the due date.
#[derive(Debug, Serialize, Deserialize)]
pub enum TaskDue {
    /// Human readable representation of the date.
    #[serde(rename = "due_string")]
    String(String),
    /// Loose target date with no exact time. TODO: should use way to encode it as a type.
    #[serde(rename = "due_date")]
    Date(String),
    /// Exact DateTime in UTC for the due date.
    #[serde(rename = "due_datetime", serialize_with = "todoist_rfc3339")]
    DateTime(chrono::DateTime<chrono::Utc>),
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
    /// Sets the [`Task::label_ids`] on the new [`Task`].
    pub label_ids: Vec<LabelID>,
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
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UpdateTask {
    /// Overwrites [`Task::content`] if set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Overwrites [`Task::description`] if set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Overwrites [`Task::label_ids`] if set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_ids: Option<Vec<LabelID>>,
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
    pub fn new(id: TaskID, content: &str) -> Task {
        Task {
            id,
            project_id: 0,
            section_id: None,
            content: content.to_string(),
            description: String::new(),
            completed: false,
            label_ids: Vec::new(),
            parent_id: None,
            order: 0,
            priority: Priority::default(),
            due: None,
            url: "http://localhost".to_string().parse().unwrap(),
            comment_count: 0,
            assignee: None,
            assigner: None,
            created: chrono::Utc::now(),
        }
    }
}
