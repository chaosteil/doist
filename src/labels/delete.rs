use crate::api::rest::Gateway;
use color_eyre::{Result, eyre::eyre};

use super::{LabelSelect, label::Selection};

#[derive(clap::Parser, Debug)]
pub struct Params {
    #[clap(flatten)]
    labels: LabelSelect,
}

pub async fn delete(params: Params, gw: &Gateway) -> Result<()> {
    let labels = params
        .labels
        .labels(&gw.labels().await?, Selection::MustChoose)?;
    if labels.is_empty() {
        return Err(eyre!("no labels selected"));
    }
    for label in labels {
        gw.delete_label(&label.id).await?;
        println!("deleted label: {}", &label);
    }
    Ok(())
}
