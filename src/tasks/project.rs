use super::fuzz_select::fuzz_select;
use color_eyre::{eyre::WrapErr, Result};
use serde::{Deserialize, Serialize};

use crate::api::rest::{Gateway, ProjectID};

/// Helper struct to get project information as command line parameter
#[derive(clap::Args, Debug, Serialize, Deserialize)]
pub struct ProjectSelect {
    /// Uses the project name with the closest name, if possible. Does fuzzy matching for the name.
    #[clap(short = 'P', long = "project")]
    project_name: Option<String>,
    /// ID of the project to use. Does nothing if -P is specified.
    #[clap(long = "project_id")]
    project_id: Option<ProjectID>,
}

impl ProjectSelect {
    pub async fn project(&self, gw: &Gateway) -> Result<Option<ProjectID>> {
        let project_name = match &self.project_name {
            Some(name) => name,
            None => return Ok(self.project_id),
        };
        Ok(Some(
            fuzz_select(&gw.projects().await?, project_name)
                .wrap_err("could not select project")?,
        ))
    }
}
