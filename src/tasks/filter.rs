use color_eyre::{eyre::eyre, Result};

use crate::{
    api::rest::{Gateway, TaskID},
    config::Config,
};

use super::state::State;

#[derive(clap::Parser, Debug)]
pub struct Filter {
    /// When selecting tasks, this will specify a filter query to run against the Todoist API to narrow down possibilities.
    #[arg(short = 'f', long = "filter")]
    filter: Option<String>,
}

impl Filter {
    pub fn new(filter: Option<String>) -> Self {
        Self { filter }
    }
    pub fn set_filter(&mut self, filter: Option<&str>) {
        self.filter = filter.map(str::to_string);
    }
    pub fn select(&self, cfg: &Config) -> String {
        self.filter
            .clone()
            .unwrap_or_else(|| cfg.default_filter.to_owned())
    }
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
            filter: Filter::new(None),
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
        let state = State::fetch_tree(Some(&self.filter.select(cfg)), gw, cfg).await?;
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

#[cfg(test)]
mod tests {
    use crate::config::Config;

    use super::Filter;

    #[test]
    fn select_filter() {
        let mut cfg = Config {
            default_filter: "all".to_owned(),
            ..Default::default()
        };

        let f = Filter::new(None);
        assert!(f.select(&cfg) == "all".to_owned());
        let f = Filter::new(Some("today".to_owned()));
        assert!(f.select(&cfg) == "today".to_owned());
    }
}
