use super::fuzz_select::fuzz_select;
use color_eyre::Result;
use serde::{Deserialize, Serialize};

use crate::api::rest::{Gateway, LabelID};

#[derive(clap::Args, Debug, Serialize, Deserialize)]
pub struct LabelSelect {
    /// Uses the label with the closest name, if possible. Does fuzzy matching for the name. Can
    /// be used multiple times to attach more labels.
    #[clap(short = 'L', long = "label")]
    label_names: Option<Vec<String>>,
    /// Uses the label with the given ID. Can be used multiple times to attach more labels.
    #[clap(long = "label_id")]
    label_ids: Option<Vec<LabelID>>,
}

impl LabelSelect {
    pub async fn labels(&self, gw: &Gateway) -> Result<Vec<LabelID>> {
        let mut labels = self.label_ids.clone().unwrap_or_default();
        if self.label_names.is_none() {
            return Ok(labels.to_vec());
        }
        let all_labels = gw.labels().await?;
        labels.extend(
            self.label_names
                .as_ref()
                .unwrap()
                .iter()
                .map(|label| fuzz_select(&all_labels, label))
                .collect::<Result<Vec<_>>>()?,
        );
        Ok(labels)
    }
}
