use color_eyre::Result;

use crate::{
    api::{
        self,
        rest::{Gateway, TaskDue, UpdateTask},
    },
    config::Config,
    labels::{self, LabelSelect},
    tasks::{Priority, filter::TaskOrInteractive},
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
        let labels = params
            .labels
            .labels(&gw.labels().await?, labels::Selection::AllowEmpty)?;
        if labels.is_empty() {
            None
        } else {
            Some(labels.into_iter().map(|l| l.name).collect())
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
    gw.update(&params.task.task_id(gw, cfg).await?, &update)
        .await
}
