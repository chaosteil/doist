use std::collections::HashMap;

use crate::{
    api::rest::{FullLabel, Label},
    interactive,
};
use color_eyre::{eyre::eyre, Result};
use serde::{Deserialize, Serialize};

use crate::api::rest::{Gateway, LabelID};

#[derive(clap::Args, Debug, Serialize, Deserialize)]
pub struct LabelSelect {
    /// Uses the label with the closest name, if possible. Does fuzzy matching for the name. Can
    /// be used multiple times to use more labels.
    #[clap(short = 'L', long = "label")]
    label_names: Option<Vec<String>>,
    /// Uses the label with the given ID. Can be used multiple times to use more labels.
    #[clap(long = "label_id")]
    label_ids: Option<Vec<LabelID>>,
}

/// Selection changes the selection mode of [`LabelSelect::labels`].
#[derive(PartialEq, Eq)]
pub enum Selection {
    /// If no labels were chosen, an empty vector is returned
    AllowEmpty,
    /// If no labels were chosen, a selection of one label must be made.
    MustChoose,
}

impl LabelSelect {
    pub async fn labels(&self, gw: &Gateway, selection: Selection) -> Result<Vec<Label>> {
        let label_ids = self.label_ids.clone().unwrap_or_default();
        let mut all_labels = gw
            .labels()
            .await?
            .into_iter()
            .map(|l| (l.id, l))
            .collect::<HashMap<_, _>>();
        let label_list = all_labels
            .values()
            .map(ToOwned::to_owned)
            .collect::<Vec<_>>();

        let mut found_labels = label_ids
            .into_iter()
            .map(|l| {
                all_labels
                    .get(&l)
                    .ok_or_else(|| eyre!("could not find label with id {}", l))
            })
            .collect::<Result<Vec<_>>>()?
            .iter()
            .map(|&l| l.to_owned())
            .collect::<Vec<_>>();

        if self.label_names.is_none() {
            if found_labels.is_empty() && selection == Selection::MustChoose {
                return Ok(vec![all_labels
                    .remove(
                        &label_list[interactive::select(
                            "Select label",
                            &label_list.iter().map(FullLabel).collect::<Vec<_>>(),
                        )?
                        .ok_or_else(|| eyre!("no labels selected"))?]
                        .id,
                    )
                    .unwrap()]);
            }
            return Ok(found_labels);
        }

        found_labels.extend(
            self.label_names
                .as_ref()
                .unwrap()
                .iter()
                .map(|label| {
                    interactive::fuzz_select(&label_list, label)
                        .map(|label| all_labels.remove(&label.id).unwrap())
                })
                .collect::<Result<Vec<_>>>()?,
        );
        Ok(found_labels)
    }
}
