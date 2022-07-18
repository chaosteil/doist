use color_eyre::Result;
use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};

use crate::api::{self, rest::Gateway};

#[derive(clap::Parser, Debug, Deserialize, Serialize)]
pub struct Params {
    /// The Task ID as provided from the Todoist API. Use `list` to find out what ID your task has.
    pub id: api::rest::TaskID,
    /// Complete will completely close a task, even if it's recurring.
    /// Since the REST API does not support completely closing tasks, this will change the due date
    /// of the task to "today" and then close it.
    #[clap(short = 'c', long = "complete")]
    pub complete: bool,
}

pub async fn close(params: Params, gw: &Gateway) -> Result<()> {
    if params.complete {
        return complete(params.id, gw).await;
    }
    gw.close(params.id).await?;
    println!("closed task {}", params.id.bright_red());
    let task = gw.task(params.id).await?;
    if !task.completed {
        if let Some(due) = task.due {
            if let Some(exact) = due.exact {
                println!("next due date: {}", exact.datetime);
            } else {
                println!("next due date: {}", due.date);
            }
        }
    }
    Ok(())
}

pub async fn complete(id: api::rest::TaskID, gw: &Gateway) -> Result<()> {
    gw.complete(id).await?;
    println!("completed task {}", id.bright_red());
    Ok(())
}
