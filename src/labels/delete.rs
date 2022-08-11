use crate::api::rest::Gateway;
use color_eyre::{eyre::eyre, Result};

use super::{label::Selection, LabelSelect};

#[derive(clap::Parser, Debug)]
pub struct Params {
    #[clap(flatten)]
    labels: LabelSelect,
}

pub async fn delete(params: Params, gw: &Gateway) -> Result<()> {
    let labels = params.labels.labels(gw, Selection::MustChoose).await?;
    if labels.is_empty() {
        return Err(eyre!("no labels selected"));
    }
    for label in labels {
        gw.delete_label(label.id).await?;
        println!("deleted label: {}", &label);
    }
    Ok(())
}
