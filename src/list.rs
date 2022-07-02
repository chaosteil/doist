use color_eyre::{eyre::Context, Result};

use crate::api::rest::{Gateway, TableTask, TaskTree};

#[derive(clap::Parser, Debug)]
pub struct Params {
    /// Specify a filter query to run against the Todoist API.
    #[clap(short='f', long="filter", default_value_t=String::from("(today | overdue)"))]
    filter: String,
}

/// List lists the tasks of the current user accessing the gateway with the given filter.
pub async fn list(params: Params, gw: &Gateway) -> Result<()> {
    let tasks = gw.tasks(Some(&params.filter)).await?;
    let tree = TaskTree::from_tasks(tasks).context("tasks do not form clean tree")?;
    for task in tree.iter() {
        println!("{}", TableTask(&task.task));
    }
    Ok(())
}
