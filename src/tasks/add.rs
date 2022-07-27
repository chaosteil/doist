use color_eyre::Result;
use serde::{Deserialize, Serialize};

use crate::{
    api::{
        rest::{CreateTask, Gateway, TableTask, TaskDue},
        tree::Tree,
    },
    tasks::Priority,
};

use super::project::ProjectSelect;

#[derive(clap::Parser, Debug, Deserialize, Serialize)]
pub struct Params {
    /// Name (title) of the task to add to the todo list.
    name: String,
    /// Set due with a human-readable text.
    ///
    /// Examples: "in two days" "tomorrow", "every 2 days from Monday"
    #[clap(short = 'd', long = "due")]
    due: Option<String>,
    /// Description that has more details about the task.
    #[clap(short = 'D', long = "desc")]
    desc: Option<String>,
    /// Sets the priority on the task. The higher the priority the more urgent the task.
    #[clap(value_enum, short = 'p', long = "priority")]
    priority: Option<Priority>,
    #[clap(flatten)]
    project: ProjectSelect,
}

pub async fn add(params: Params, gw: &Gateway) -> Result<()> {
    let project_id = params.project.project(gw).await?;
    let mut create = CreateTask {
        content: params.name,
        description: params.desc,
        priority: params.priority.map(|p| p.into()),
        project_id,
        ..Default::default()
    };
    if let Some(due) = params.due {
        create.due = Some(TaskDue::String(due));
    }
    let project = match project_id {
        Some(pid) => Some(gw.project(pid).await?),
        None => None,
    };
    let task = Tree::new(gw.create(&create).await?);
    let mut table = TableTask::from_task(&task);
    table.1 = project.as_ref();
    println!("created task: {}", table);
    Ok(())
}
