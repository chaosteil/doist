use color_eyre::{Result, eyre::eyre};

use crate::{
    api::{
        self,
        rest::{Gateway, TaskDue, UpdateTask},
    },
    config::Config,
    labels::{self, LabelSelect},
    tasks::{filter::TaskOrInteractive, Priority},
};

#[derive(clap::Parser, Debug)]
pub struct Params {
    #[clap(flatten)]
    pub task: TaskOrInteractive,
    /// Name of a task
    #[arg(short = 'n', long = "name")]
    pub name: Option<String>,
    #[arg(short = 'd', long = "due")]
    pub due: Option<String>,
    /// Description of a task.
    #[arg(short = 'D', long = "desc")]
    pub desc: Option<String>,
    /// Sets the priority on the task. The lower the priority the more urgent the task.
    #[arg(value_enum, short = 'p', long = "priority")]
    pub priority: Option<Priority>,
    #[clap(flatten)]
    pub labels: LabelSelect,
}

impl Params {
    pub fn new(id: api::rest::TaskID) -> Self {
        Self {
            task: TaskOrInteractive::with_id(id),
            name: None,
            due: None,
            desc: None,
            priority: None,
            labels: LabelSelect::default(),
        }
    }
}

pub async fn edit(params: Params, gw: &Gateway, cfg: &Config) -> Result<()> {
    let labels = {
        if params.labels.is_empty() {
            None
        } else {
            let labels = params
                .labels
                .labels(&gw.labels().await?, labels::Selection::AllowEmpty)?;
            if labels.is_empty() {
                None
            } else {
                Some(labels.into_iter().map(|l| l.name).collect())
            }
        }
    };
    let mut update = UpdateTask {
        content: params.name,
        description: params.desc,
        priority: params.priority.map(|p| p.into()),
        labels,
        ..Default::default()
    };
    if let Some(due) = params.due {
        update.due = Some(TaskDue::String(due))
    }
    if update == UpdateTask::default() {
        return Err(eyre!(
            "No changes to apply. Use the CLI flags to set the desired fields."
        ));
    }
    gw.update(&params.task.task_id(gw, cfg).await?, &update)
        .await
}

#[cfg(test)]
mod test {
    use wiremock::MockServer;

    use super::*;

    #[tokio::test]
    async fn update_nochanges() {
        let mock_server = MockServer::start().await;
        let gw = Gateway::new("", &mock_server.uri().parse().unwrap());
        let result = edit(
            Params {
                task: TaskOrInteractive::with_id("123".into()),
                name: None,
                due: None,
                desc: None,
                priority: None,
                labels: LabelSelect::default(),
            },
            &gw,
            &Config::default(),
        )
        .await;
        mock_server.verify().await;
        assert!(result.is_err(), "{:?}", result.unwrap_err());
        let result = result.unwrap_err();
        assert!(
            result.to_string().contains("No changes to apply"),
            "{:?}",
            result
        );
    }
}
