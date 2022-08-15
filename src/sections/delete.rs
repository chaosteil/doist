use crate::{api::rest::Gateway, projects::project::ProjectSelect};
use color_eyre::{eyre::eyre, Result};

use super::section::SectionSelect;

#[derive(clap::Parser, Debug)]
pub struct Params {
    #[clap(flatten)]
    project: ProjectSelect,
    #[clap(flatten)]
    section: SectionSelect,
}

pub async fn delete(params: Params, gw: &Gateway) -> Result<()> {
    let project_id = params.project.project(gw).await?;
    let section_id = params
        .section
        .section(project_id, gw)
        .await?
        .ok_or_else(|| eyre!("must provide section to delete"))?;
    let section = gw.section(section_id).await?;
    gw.delete_section(section_id).await?;
    println!("deleted section: {}", &section);
    Ok(())
}
