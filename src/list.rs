use color_eyre::{eyre::WrapErr, Result};

use crate::{
    api::rest::{Gateway, TableTask, TaskTree},
    close,
};
use strum::{Display, EnumVariantNames, FromRepr, VariantNames};

#[derive(clap::Parser, Debug)]
pub struct Params {
    /// Specify a filter query to run against the Todoist API.
    #[clap(short='f', long="filter", default_value_t=String::from("(today | overdue)"))]
    filter: String,
    /// Run the list display in interactive mode to perform various actions on the items.
    #[clap(short = 'i')]
    interactive: bool,
}

/// List lists the tasks of the current user accessing the gateway with the given filter.
pub async fn list(params: Params, gw: &Gateway) -> Result<()> {
    let tasks = gw.tasks(Some(&params.filter)).await?;
    let tree = TaskTree::from_tasks(tasks).wrap_err("tasks do not form clean tree")?;
    if params.interactive {
        list_interactive_tasks(&tree, gw).await?;
    } else {
        list_tasks(&tree);
    }
    Ok(())
}

fn list_tasks(tree: &[TaskTree]) {
    for task in tree.iter() {
        println!("{}", TableTask(&task.task));
    }
}

async fn list_interactive_tasks(tree: &[TaskTree], gw: &Gateway) -> Result<()> {
    let result = dialoguer::FuzzySelect::with_theme(&dialoguer::theme::ColorfulTheme::default())
        .items(&tree.iter().map(|t| TableTask(&t.task)).collect::<Vec<_>>())
        .default(0)
        .interact_opt()
        .wrap_err("Unable to make a selection")?;
    match result {
        Some(index) => select_task_option(&tree[index], gw).await?,
        None => println!("No selection made"),
    };
    Ok(())
}

#[derive(Display, FromRepr, EnumVariantNames)]
enum TaskOptions {
    Close,
    Quit,
}

async fn select_task_option(task: &TaskTree, gw: &Gateway) -> Result<()> {
    println!("{}", task.task);
    let result = dialoguer::FuzzySelect::with_theme(&dialoguer::theme::ColorfulTheme::default())
        .items(TaskOptions::VARIANTS)
        .default(0)
        .interact_opt()
        .wrap_err("Unable to make a selection")?;
    let option = match result {
        Some(index) => TaskOptions::from_repr(index).unwrap(),
        None => {
            println!("No selection made");
            return Ok(());
        }
    };
    match option {
        TaskOptions::Close => close::close(close::Params { id: task.task.id }, gw).await?,
        TaskOptions::Quit => {}
    };
    Ok(())
}
