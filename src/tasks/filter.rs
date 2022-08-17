use color_eyre::{eyre::eyre, Result};

use crate::api::rest::{Gateway, TaskID};

use super::state::State;

const DEFAULT_FILTER: &str = "(today | overdue)";

#[derive(clap::Parser, Debug)]
pub struct Filter {
    /// When selecting tasks, this will specify a filter query to run against the Todoist API to narrow down possibilities.
    #[clap(short='f', long="filter", default_value_t=String::from(DEFAULT_FILTER))]
    pub filter: String,
}

/// TaskOrInteractive is a helper struct to be embedded into other Params so that they can perform
/// selections based on Task ID or selecting interactively.
#[derive(clap::Parser, Debug)]
pub struct TaskOrInteractive {
    /// The Task ID as provided from the Todoist API. Use `list` to find out what ID your task has.
    /// If omitted, will interactively select task.
    id: Option<TaskID>,
    #[clap(flatten)]
    filter: Filter,
}

impl TaskOrInteractive {
    pub fn with_id(id: TaskID) -> Self {
        Self {
            id: Some(id),
            filter: Filter {
                filter: DEFAULT_FILTER.to_string(),
            },
        }
    }
    pub async fn task_id(&self, gw: &Gateway) -> Result<TaskID> {
        let (id, _) = self.task(gw).await?;
        Ok(id)
    }

    pub async fn task(&self, gw: &Gateway) -> Result<(TaskID, State)> {
        let state = State::fetch_tree(Some(&self.filter.filter), gw).await?;
        let id = match self.id {
            Some(id) => id,
            None => state
                .select_task()?
                .map(|t| t.id)
                .ok_or_else(|| eyre!("no task selected"))?,
        };
        Ok((id, state))
    }
}

impl From<TaskID> for TaskOrInteractive {
    fn from(id: TaskID) -> Self {
        Self::with_id(id)
    }
}
