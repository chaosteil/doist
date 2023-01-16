use color_eyre::{eyre::eyre, Result};

use crate::{
    api::rest::{Gateway, TaskID},
    config::Config,
};

use super::state::State;

pub const DEFAULT_FILTER: &str = "(today | overdue)";

#[derive(clap::Parser, Debug)]
pub struct Filter {
    /// When selecting tasks, this will specify a filter query to run against the Todoist API to narrow down possibilities.
    #[arg(short='f', long="filter", default_value_t=String::from(DEFAULT_FILTER))]
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
    pub async fn task_id(&self, gw: &Gateway, cfg: &Config) -> Result<TaskID> {
        let (id, _) = self.task(gw, cfg).await?;
        Ok(id)
    }

    pub async fn task<'a>(
        &'_ self,
        gw: &'_ Gateway,
        cfg: &'a Config,
    ) -> Result<(TaskID, State<'a>)> {
        let state = State::fetch_tree(Some(&self.filter.filter), gw, cfg).await?;
        let id = match &self.id {
            Some(id) => id.clone(),
            None => state
                .select_task()?
                .map(|t| t.id.clone())
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
