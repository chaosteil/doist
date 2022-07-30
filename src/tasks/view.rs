use color_eyre::{eyre::eyre, Result};

use crate::{api::rest::Gateway, comments};

use super::list::TaskOrInteractive;

#[derive(clap::Parser, Debug)]
pub struct Params {
    #[clap(flatten)]
    task: TaskOrInteractive,
}

/// Displays full information about a task.
pub async fn view(params: Params, gw: &Gateway) -> Result<()> {
    let (id, list) = params.task.task(gw).await?;
    let task = list.full_task(list.task(id).ok_or_else(|| eyre!("no valid task"))?);
    println!("{}", task);
    if task.0.comment_count > 0 {
        let comments = gw.task_comments(id).await?;
        comments::list(&comments)
    }
    Ok(())
}
