use crate::api::rest::Gateway;
use color_eyre::Result;

#[derive(clap::Parser, Debug)]
pub struct Params {
    /// Name of the section to create.
    name: String,
}

pub async fn add(params: Params, gw: &Gateway) -> Result<()> {
    /*let section = gw
        .create_section(&CreateSection {
            name: params.name,
            ..Default::default()
        })
        .await?;
    println!("created section: {}", &section);*/
    Ok(())
}
