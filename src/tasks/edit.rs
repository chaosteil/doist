use color_eyre::Result;

use crate::{
    api::{
        self,
        rest::{Gateway, TaskDue, UpdateTask},
    },
    tasks::Priority,
};

use super::list::TaskOrInteractive;

#[derive(clap::Parser, Debug)]
pub struct Params {
    #[clap(flatten)]
    pub task: TaskOrInteractive,
    // Name of a task
    #[clap(short = 'n', long = "name")]
    pub name: Option<String>,
    #[clap(short = 'd', long = "due")]
    pub due: Option<String>,
    // Description of a task.
    #[clap(short = 'D', long = "desc")]
    pub desc: Option<String>,
    /// Sets the priority on the task. The lower the priority the more urgent the task.
    #[clap(value_enum, short = 'p', long = "priority")]
    pub priority: Option<Priority>,
}

impl Params {
    pub fn new(id: api::rest::TaskID) -> Self {
        Self {
            task: TaskOrInteractive::with_id(id),
            name: None,
            due: None,
            desc: None,
            priority: None,
        }
    }
}

pub async fn edit(params: Params, gw: &Gateway) -> Result<()> {
    let mut update = UpdateTask {
        content: params.name,
        description: params.desc,
        priority: params.priority.map(|p| p.into()),
        ..Default::default()
    };
    if let Some(due) = params.due {
        update.due = Some(TaskDue::String(due))
    }
    gw.update(params.task.task_id(gw).await?, &update).await
}
