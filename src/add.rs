use color_eyre::Result;
use serde::{Deserialize, Serialize};

use crate::api::rest::{CreateTask, Gateway, TableTask, TaskDue};

#[derive(clap::Parser, Debug, Deserialize, Serialize)]
pub struct Params {
    /// Name (title) of the task to add to the todo list.
    name: String,
    /// Set due with a human-readable text.
    ///
    /// Examples: "in two days" "tomorrow", "every 2 days from Monday"
    #[clap(short = 'd')]
    due: Option<String>,
    /// Description that has more details about the task.
    desc: Option<String>,
}

pub async fn add(params: Params, gw: &Gateway) -> Result<()> {
    let mut create = CreateTask {
        content: params.name,
        description: params.desc,
        ..Default::default()
    };
    if let Some(due) = params.due {
        create.due = Some(TaskDue::String(due));
    }
    let task = gw.create(&create).await?;
    println!("created task: {}", TableTask(&task));
    Ok(())
}
