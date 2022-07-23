use core::fmt;
use std::fmt::Display;

use crate::api::tree::Treeable;
use crate::api::{deserialize::deserialize_zero_to_none, tree::Tree};
use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize, Serializer};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::{Project, ProjectID, Section, SectionID};

pub type TaskID = usize;
pub type LabelID = usize;
pub type UserID = usize;

/// Priority as is given from the Todoist API.
///
/// 1 for Normal up to 4 for Urgent.
#[derive(Debug, Copy, Clone, Serialize_repr, Deserialize_repr, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Priority {
    Normal = 1,
    High = 2,
    VeryHigh = 3,
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
    pub id: TaskID,
    pub project_id: ProjectID,
    #[serde(deserialize_with = "deserialize_zero_to_none")]
    pub section_id: Option<SectionID>,
    pub content: String,
    pub description: String,
    pub completed: bool,
    pub label_ids: Vec<LabelID>,
    pub parent_id: Option<TaskID>,
    pub order: isize,
    pub priority: Priority,
    pub due: Option<DueDate>,
    pub url: String,
    pub comment_count: usize,
    pub assignee: Option<UserID>,
    #[serde(deserialize_with = "deserialize_zero_to_none")]
    pub assigner: Option<UserID>,
    #[serde(serialize_with = "todoist_rfc3339")]
    pub created: chrono::DateTime<chrono::Utc>,
}

impl Treeable for Task {
    fn id(&self) -> usize {
        self.id
    }

    fn parent_id(&self) -> Option<usize> {
        self.parent_id
    }
}

/// Used to display full information about a Task.
pub struct FullTask<'a, 'b, 'c>(
    pub &'a Task,
    pub Option<&'b Project>,
    pub Option<&'c Section>,
);

impl Display for FullTask<'_, '_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ID: {}\nPriority: {}\nContent: {}\nDescription: {}\n",
            self.0.id.bright_yellow(),
            self.0.priority,
            self.0.content.default_color(),
            self.0.description.default_color()
        )?;
        if let Some(due) = &self.0.due {
            writeln!(f, "Due: {}", due)?;
        }
        if let Some(project) = &self.1 {
            writeln!(f, "Project: {}", project.name)?;
        }
        if let Some(section) = &self.2 {
            writeln!(f, "Section: {}", section)?;
        }
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
pub struct TableTask<'a, 'b, 'c>(
    pub &'a Tree<Task>,
    pub Option<&'b Project>,
    pub Option<&'c Section>,
);

impl Display for TableTask<'_, '_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let TableTask::<'_, '_, '_>(task, project, section) = self;
        let subtask_padding = if self.0.depth > 0 {
            format!("{}⌞ ", "  ".repeat(self.0.depth))
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
    pub datetime: chrono::DateTime<chrono::FixedOffset>,
    pub timezone: String, // TODO: fix for when it's a UTC offset
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
    #[serde(rename = "string")]
    pub human_readable: String,
    pub date: chrono::NaiveDate,
    pub recurring: bool,
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
    #[serde(rename = "due_string")]
    String(String),
    #[serde(rename = "due_date")]
    Date(String), // TODO: chrono day
    #[serde(rename = "due_datetime", serialize_with = "todoist_rfc3339")]
    DateTime(chrono::DateTime<chrono::Utc>),
}
/// This function is there to serialize the datetime into something that the Todoist API can
/// understand, as it doesn't quite implement the full rfc3339 spec and breaks with the default
/// chrono formatter.
fn todoist_rfc3339<S>(dt: &chrono::DateTime<chrono::Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let dt = dt.format("%Y-%m-%dT%H:%M:%SZ").to_string();
    serializer.serialize_str(&dt)
}

/// Command used with [`super::Gateway::create`] to create a new Task.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CreateTask {
    pub content: String,
    pub description: Option<String>,
    pub project_id: Option<ProjectID>,
    pub section_id: Option<SectionID>,
    pub parent_id: Option<TaskID>,
    pub order: Option<isize>,
    pub label_ids: Vec<LabelID>,
    pub priority: Option<Priority>,
    #[serde(flatten)]
    pub due: Option<TaskDue>,
    pub due_lang: Option<String>,
    pub assignee: Option<UserID>,
}

/// Command used with [`super::Gateway::update`] to update a Task.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UpdateTask {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_ids: Option<Vec<LabelID>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<Priority>,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub due: Option<TaskDue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_lang: Option<String>,
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
            url: String::new(),
            comment_count: 0,
            assignee: None,
            assigner: None,
            created: chrono::Utc::now(),
        }
    }
}
