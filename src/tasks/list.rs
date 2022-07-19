use std::collections::HashMap;

use color_eyre::{eyre::WrapErr, Result};

use crate::{
    api::{
        rest::{FullTask, Gateway, Project, ProjectID, TableTask, Task},
        tree::Tree,
    },
    tasks::{close, edit},
};
use strum::{Display, EnumVariantNames, FromRepr, VariantNames};

#[derive(clap::Parser, Debug)]
pub struct Params {
    /// Specify a filter query to run against the Todoist API.
    #[clap(short='f', long="filter", default_value_t=String::from("(today | overdue)"))]
    filter: String,
    /// Disables interactive mode and simply displays the list.
    #[clap(short = 'n', long = "nointeractive")]
    nointeractive: bool,
}

/// List lists the tasks of the current user accessing the gateway with the given filter.
pub async fn list(params: Params, gw: &Gateway) -> Result<()> {
    let tasks = gw.tasks(Some(&params.filter)).await?;
    let projects = gw
        .projects()
        .await?
        .into_iter()
        .map(|p| (p.id, p))
        .collect();
    let tree = Tree::from_items(tasks).wrap_err("tasks do not form clean tree")?;
    if params.nointeractive {
        list_tasks(&tree, &projects);
    } else {
        match get_interactive_tasks(&tree, &projects)? {
            Some(task) => select_task_option(task, &projects, gw).await?,
            None => println!("No selection was made"),
        }
    }
    Ok(())
}

pub fn get_interactive_tasks<'a, 'b>(
    tree: &'a [Tree<Task>],
    projects: &'b HashMap<ProjectID, Project>,
) -> Result<Option<&'a Tree<Task>>> {
    let result = dialoguer::FuzzySelect::with_theme(&dialoguer::theme::ColorfulTheme::default())
        .items(
            &tree
                .iter()
                .map(|t| TableTask(t, projects.get(&t.project_id)))
                .collect::<Vec<_>>(),
        )
        .with_prompt("Select task")
        .default(0)
        .interact_opt()
        .wrap_err("Unable to make a selection")?;
    Ok(result.map(|index| &tree[index]))
}

fn list_tasks(tree: &[Tree<Task>], projects: &HashMap<ProjectID, Project>) {
    for task in tree.iter() {
        println!("{}", TableTask(task, projects.get(&task.project_id)));
    }
}

#[derive(Display, FromRepr, EnumVariantNames)]
enum TaskOptions {
    Close,
    Complete,
    Edit,
    Quit,
}

async fn select_task_option(
    task: &Tree<Task>,
    projects: &HashMap<ProjectID, Project>,
    gw: &Gateway,
) -> Result<()> {
    println!(
        "{}",
        FullTask(&task.item, projects.get(&task.item.project_id))
    );
    let result = match make_selection(TaskOptions::VARIANTS)? {
        Some(index) => TaskOptions::from_repr(index).unwrap(),
        None => {
            println!("No selection made");
            return Ok(());
        }
    };
    match result {
        TaskOptions::Close => {
            close::close(
                close::Params {
                    id: task.id,
                    complete: false,
                },
                gw,
            )
            .await?
        }
        TaskOptions::Complete => {
            close::close(
                close::Params {
                    id: task.id,
                    complete: true,
                },
                gw,
            )
            .await?
        }
        TaskOptions::Edit => edit_task(task, gw).await?,
        TaskOptions::Quit => {}
    };
    Ok(())
}

#[derive(Display, FromRepr, EnumVariantNames)]
enum EditOptions {
    Name,
    Description,
    Due,
    Priority,
    // Project, TODO: allow to edit project
    Quit,
}

async fn edit_task(task: &Tree<Task>, gw: &Gateway) -> Result<()> {
    // edit::edit(edit::Params { id: task.task.id }, gw).await?,
    let result = match make_selection(EditOptions::VARIANTS)? {
        Some(index) => EditOptions::from_repr(index).unwrap(),
        None => {
            println!("No selection made");
            return Ok(());
        }
    };
    match result {
        EditOptions::Quit => {}
        EditOptions::Priority => {
            let selection = dialoguer::Select::new()
                .with_prompt("Set priority")
                .items(&["1 - Urgent", "2 - Very High", "3 - High", "4 - Normal"])
                .default((4 - task.priority as u8) as usize)
                .interact()
                .wrap_err("Bad user input")?
                + 1;
            let mut params = edit::Params::new(task.id);
            params.priority = Some(selection.try_into()?);
            edit::edit(params, gw).await?;
        }
        _ => {
            let text = dialoguer::Input::new()
                .with_prompt("New value")
                .interact_text()
                .wrap_err("Bad user input")?;
            let mut params = edit::Params::new(task.id);
            match result {
                EditOptions::Name => {
                    params.name = Some(text);
                }
                EditOptions::Description => {
                    params.desc = Some(text);
                }
                EditOptions::Due => {
                    params.due = Some(text);
                }
                EditOptions::Priority => unreachable!(),
                EditOptions::Quit => unreachable!(),
            };
            edit::edit(params, gw).await?;
        }
    };
    Ok(())
}

fn make_selection<T: ToString + std::fmt::Display>(variants: &[T]) -> Result<Option<usize>> {
    dialoguer::FuzzySelect::with_theme(&dialoguer::theme::ColorfulTheme::default())
        .items(variants)
        .default(0)
        .interact_opt()
        .wrap_err("Unable to make a selection")
}
