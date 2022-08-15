use crate::{
    api::rest::{CreateSection, Gateway},
    projects::project::ProjectSelect,
};
use color_eyre::{eyre::eyre, Result};

#[derive(clap::Parser, Debug)]
pub struct Params {
    #[clap(flatten)]
    project: ProjectSelect,
    /// Name of the section to create.
    name: String,
}

pub async fn add(params: Params, gw: &Gateway) -> Result<()> {
    let section = gw
        .create_section(&CreateSection {
            name: params.name,
            project_id: params
                .project
                .project(gw)
                .await?
                .ok_or_else(|| eyre!("must provide project to add section to"))?,
            ..Default::default()
        })
        .await?;
    println!("created section: {}", &section);
    Ok(())
}
