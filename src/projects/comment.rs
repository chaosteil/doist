use color_eyre::Result;

use crate::api::rest::{CreateComment, FullComment, Gateway, ThreadID};

use super::filter::ProjectOrInteractive;

#[derive(clap::Parser, Debug)]
pub struct Params {
    /// The text of the comment. Supports Markdown.
    content: String,
    #[clap(flatten)]
    project: ProjectOrInteractive,
}

/// Creates a new comment for a project.
pub async fn comment(params: Params, gw: &Gateway) -> Result<()> {
    let (id, _) = params.project.project(gw).await?;
    let comment = gw
        .create_comment(&CreateComment {
            thread: ThreadID::Project { project_id: id },
            content: params.content,
        })
        .await?;
    println!("created comment: {}", FullComment(&comment));
    Ok(())
}
