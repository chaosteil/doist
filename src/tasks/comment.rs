use color_eyre::Result;

use crate::api::rest::{CreateComment, FullComment, Gateway, ThreadID};

use super::filter::TaskOrInteractive;

#[derive(clap::Parser, Debug)]
pub struct Params {
    /// The text of the comment. Supports Markdown.
    content: String,
    #[clap(flatten)]
    task: TaskOrInteractive,
}

/// Creates a new comment for a task.
pub async fn comment(params: Params, gw: &Gateway) -> Result<()> {
    let (id, _) = params.task.task(gw).await?;
    let comment = gw
        .create_comment(&CreateComment {
            thread: ThreadID::Task { task_id: id },
            content: params.content,
        })
        .await?;
    println!("created comment: {}", FullComment(&comment));
    Ok(())
}
