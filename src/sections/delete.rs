use crate::{
    api::rest::{Gateway, Section},
    interactive,
};
use color_eyre::Result;

#[derive(clap::Parser, Debug)]
pub struct Params {
    // TODO: make soft dependency on project selection here
    #[clap(flatten)]
    section: interactive::Selection<Section>,
}

pub async fn delete(params: Params, gw: &Gateway) -> Result<()> {
    let sections = gw.sections().await?;
    let section = params.section.mandatory(&sections)?;
    gw.delete_section(&section.id).await?;
    println!("deleted section: {}", &section);
    Ok(())
}
