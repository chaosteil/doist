use color_eyre::Result;

use crate::{
    api::rest::{CreateComment, FullComment, Gateway, Project, ThreadID},
    interactive,
};

#[derive(clap::Parser, Debug)]
pub struct Params {
    /// The text of the comment. Supports Markdown.
    content: String,
    #[clap(flatten)]
    project: interactive::Selection<Project>,
}

/// Creates a new comment for a project.
pub async fn comment(params: Params, gw: &Gateway) -> Result<()> {
    let projects = gw.projects().await?;
    let project = params.project.mandatory(&projects)?;
    let comment = gw
        .create_comment(&CreateComment {
            thread: ThreadID::Project {
                project_id: project.id,
            },
            content: params.content,
        })
        .await?;
    println!("created comment: {}", FullComment(&comment));
    Ok(())
}
