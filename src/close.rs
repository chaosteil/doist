use color_eyre::{eyre::WrapErr, Result};
use owo_colors::OwoColorize;
use serde::{Deserialize, Serialize};

use crate::api::{self, rest::Gateway};

#[derive(clap::Parser, Debug, Deserialize, Serialize)]
pub struct Params {
    /// The Task ID as provided from the todoist API. Use `list` to find out what ID your task has.
    pub id: api::rest::TaskID,
}

pub async fn close(params: Params, gw: &Gateway) -> Result<()> {
    gw.close(params.id).await.wrap_err("unable to close task")?;
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
