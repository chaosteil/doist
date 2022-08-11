use crate::api::rest::{CreateLabel, Gateway};
use color_eyre::Result;

#[derive(clap::Parser, Debug)]
pub struct Params {
    /// Name of the label to create.
    name: String,
}

pub async fn add(params: Params, gw: &Gateway) -> Result<()> {
    let label = gw
        .create_label(&CreateLabel {
            name: params.name,
            ..Default::default()
        })
        .await?;
    println!("created label: {}", &label);
    Ok(())
}
