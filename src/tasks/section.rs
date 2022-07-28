use super::fuzz_select::fuzz_select;
use color_eyre::Result;
use serde::{Deserialize, Serialize};

use crate::api::rest::{Gateway, ProjectID, SectionID};

/// Helper struct to get section information as command line parameter
#[derive(clap::Args, Debug, Serialize, Deserialize)]
pub struct SectionSelect {
    /// Assigns the section name with the closest name, if possible. Does fuzzy matching for the
    /// name. Can be used without project definition, but will match better if project is also
    /// provided.
    #[clap(short = 'S', long = "section")]
    section_name: Option<String>,
    /// ID of the section to attach this task to. Does nothing if -S is specified.
    #[clap(long = "section_id")]
    section_id: Option<SectionID>,
}

impl SectionSelect {
    pub async fn section(
        &self,
        project_id: Option<ProjectID>,
        gw: &Gateway,
    ) -> Result<Option<SectionID>> {
        let section_name = match &self.section_name {
            Some(name) => name,
            None => return Ok(self.section_id),
        };
        let sections = gw.sections().await?;
        let sections = match project_id {
            Some(project_id) => sections
                .into_iter()
                .filter(|s| s.project_id == project_id)
                .collect(),
            None => sections,
        };
        Ok(Some(fuzz_select(&sections, section_name)?))
    }
}
