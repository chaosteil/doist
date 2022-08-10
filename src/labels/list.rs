use crate::api::rest::Gateway;
use color_eyre::Result;

#[derive(clap::Parser, Debug)]
pub struct Params {}

pub async fn list(_params: Params, gw: &Gateway) -> Result<()> {
    let labels = gw.labels().await?;
    for label in labels {
        println!("{}", &label);
    }
    Ok(())
}
