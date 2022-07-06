use color_eyre::{eyre::WrapErr, Result};

use crate::api::rest::{Gateway, TableTask, TaskTree};

#[derive(clap::Parser, Debug)]
pub struct Params {
    /// Specify a filter query to run against the Todoist API.
    #[clap(short='f', long="filter", default_value_t=String::from("(today | overdue)"))]
    filter: String,
    #[clap(short = 'i')]
    interactive: bool,
}

/// List lists the tasks of the current user accessing the gateway with the given filter.
pub async fn list(params: Params, gw: &Gateway) -> Result<()> {
    let tasks = gw.tasks(Some(&params.filter)).await?;
    let tree = TaskTree::from_tasks(tasks).wrap_err("tasks do not form clean tree")?;
    if params.interactive {
        list_interactive_tasks(&tree);
    } else {
        list_tasks(&tree);
    }
    Ok(())
}

fn list_tasks(tree: &Vec<TaskTree>) {
    for task in tree.iter() {
        println!("{}", TableTask(&task.task));
    }
}

fn list_interactive_tasks(_tree: &Vec<TaskTree>) {}
