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

pub async fn delete(params: Params, gw: &Gateway) -> Result<()> {
    let projects = gw.projects().await?;
    let project = params.project.mandatory(&projects)?;
    gw.delete_project(&project.id).await?;
    println!("deleted project: {}", &project);
    Ok(())
}
