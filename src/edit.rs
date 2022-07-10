use color_eyre::Result;
use serde::{Deserialize, Serialize};

use crate::api::{
    self,
    rest::{Gateway, TaskDue, UpdateTask},
};

#[derive(clap::Parser, Debug, Deserialize, Serialize)]
pub struct Params {
    /// The Task ID as provided from the todoist API. Use `list` to find out what ID your task has.
    pub id: api::rest::TaskID,
    pub name: Option<String>,
    #[clap(short = 'd')]
    pub due: Option<String>,
    pub desc: Option<String>,
}

impl Params {
    pub fn new(id: api::rest::TaskID) -> Self {
        Self {
            id,
            name: None,
            due: None,
            desc: None,
        }
    }
}

pub async fn edit(params: Params, gw: &Gateway) -> Result<()> {
    let mut update = UpdateTask {
        content: params.name,
        description: params.desc,
        ..Default::default()
    };
    if let Some(due) = params.due {
        update.due = Some(TaskDue::String(due))
    }
    gw.update(params.id, &update).await
}
