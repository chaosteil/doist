use crate::{api::rest::Gateway, projects::filter::ProjectOrInteractive};
use color_eyre::{eyre::eyre, Result};

#[derive(clap::Parser, Debug)]
pub struct Params {
    #[clap(flatten)]
    project: ProjectOrInteractive,
}

/// Lists available sections in a project.
pub async fn list(params: Params, gw: &Gateway) -> Result<()> {
    let (id, projects) = params.project.project(gw).await?;
    let sections = gw
        .sections()
        .await?
        .into_iter()
        .filter(|s| s.project_id == id)
        .collect::<Vec<_>>();
    println!(
        "{} sections:",
        projects
            .project(id)
            .ok_or_else(|| eyre!("project not found in full project list"))?
            .item
    );
    for s in sections {
        println!("{}", s);
    }
    Ok(())
}
