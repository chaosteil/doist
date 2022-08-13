use crate::api::rest::{CreateProject, Gateway};
use color_eyre::Result;

#[derive(clap::Parser, Debug)]
pub struct Params {
    /// Name of the project to create.
    name: String,
}

pub async fn add(params: Params, gw: &Gateway) -> Result<()> {
    let project = gw
        .create_project(&CreateProject {
            name: params.name,
            ..Default::default()
        })
        .await?;
    println!("created project: {}", &project);
    Ok(())
}
