use color_eyre::Result;
use serde::{Deserialize, Serialize};

use crate::api::{
    self,
    rest::{Gateway, UpdateTask},
};

#[derive(clap::Parser, Debug, Deserialize, Serialize)]
pub struct Params {
    /// The Task ID as provided from the todoist API. Use `list` to find out what ID your task has.
    pub id: api::rest::TaskID,
}

pub async fn edit(params: Params, gw: &Gateway) -> Result<()> {
    gw.update(params.id, &UpdateTask::default()).await
}
