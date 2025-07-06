use crate::{
    api::rest::{Gateway, Project},
    interactive,
};
use color_eyre::Result;

#[derive(clap::Parser, Debug)]
pub struct Params {
    #[clap(flatten)]
    project: interactive::Selection<Project>,
}

/// Lists available sections in a project.
pub async fn list(params: Params, gw: &Gateway) -> Result<()> {
    let projects = gw.projects().await?;
    let project = params.project.mandatory(&projects)?;
    let sections = gw
        .sections()
        .await?
        .into_iter()
        .filter(|s| s.project_id == project.id)
        .collect::<Vec<_>>();
    println!("{project} sections:");
    for s in sections {
        println!("{s}");
    }
    Ok(())
}
