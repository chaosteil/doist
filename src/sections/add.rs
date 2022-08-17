use crate::{
    api::rest::{CreateSection, Gateway, Project},
    interactive,
};
use color_eyre::Result;

#[derive(clap::Parser, Debug)]
pub struct Params {
    #[clap(flatten)]
    project: interactive::Selection<Project>,
    /// Name of the section to create.
    name: String,
}

pub async fn add(params: Params, gw: &Gateway) -> Result<()> {
    let projects = gw.projects().await?;
    let project = params.project.mandatory(&projects)?;
    let section = gw
        .create_section(&CreateSection {
            name: params.name,
            project_id: project.id,
            ..Default::default()
        })
        .await?;
    println!("created section: {}", &section);
    Ok(())
}
