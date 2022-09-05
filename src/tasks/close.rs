use color_eyre::{eyre::WrapErr, Result};
use owo_colors::OwoColorize;

use crate::{
    api::{self, rest::Gateway},
    config::Config,
};

use super::filter;

#[derive(clap::Parser, Debug)]
pub struct Params {
    #[clap(flatten)]
    pub task: filter::TaskOrInteractive,
    /// Complete will completely close a task, even if it's recurring.
    /// Since the REST API does not support completely closing tasks, this will change the due date
    /// of the task to "today" and then close it.
    #[clap(short = 'c', long = "complete")]
    pub complete: bool,
}

pub async fn close(params: Params, gw: &Gateway, cfg: &Config) -> Result<()> {
    let id = params
        .task
        .task_id(gw, cfg)
        .await
        .wrap_err("no task selected for closing")?;
    if params.complete {
        return complete(id, gw).await;
    }
    gw.close(id).await?;
    println!("closed task {}", id.bright_red());
    let task = gw.task(id).await?;
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
