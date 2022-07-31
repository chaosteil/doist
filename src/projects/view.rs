use crate::{api::rest::Gateway, comments};
use color_eyre::{eyre::eyre, Result};

use super::filter::ProjectOrInteractive;

#[derive(clap::Parser, Debug)]
pub struct Params {
    #[clap(flatten)]
    project: ProjectOrInteractive,
}

pub async fn view(params: Params, gw: &Gateway) -> Result<()> {
    let (id, list) = params.project.project(gw).await?;
    let project = match list.project(id) {
        Some(project) => project,
        None => return Err(eyre!("no valid project")),
    };
    println!("Project: {}", project.item);
    let sections = list.sections(project.id);
    if !sections.is_empty() {
        println!("Sections:");
        for section in sections {
            println!("{}", section)
        }
    }
    if project.comment_count > 0 {
        let comments = gw.project_comments(id).await?;
        comments::list(&comments)
    }
    Ok(())
}
