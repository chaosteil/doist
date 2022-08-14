use crate::api::rest::Gateway;
use color_eyre::Result;

use crate::projects::filter::ProjectOrInteractive;

#[derive(clap::Parser, Debug)]
pub struct Params {
    #[clap(flatten)]
    project: ProjectOrInteractive,
}

pub async fn delete(params: Params, gw: &Gateway) -> Result<()> {
    /*
    let (id, projects) = params.project.project(gw).await?;
    gw.delete_project(id).await?;
    if let Some(project) = projects.project(id) {
        println!("deleted project: {}", &project.item);
    }*/
    Ok(())
}
