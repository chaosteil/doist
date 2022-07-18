use crate::api::rest::Gateway;
use color_eyre::Result;

#[derive(clap::Parser, Debug)]
pub struct Params {}

/// Lists available projects.
pub async fn list(_params: Params, gw: &Gateway) -> Result<()> {
    let projects = gw.projects().await?;
    for project in projects.iter() {
        println!("{}", &project);
    }
    Ok(())
}
