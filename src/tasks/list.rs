use std::collections::HashMap;

use color_eyre::{eyre::eyre, eyre::WrapErr, Result};

use crate::{
    api::{
        rest::{FullTask, Gateway, Project, ProjectID, TableTask, Task, TaskID},
        tree::Tree,
    },
    tasks::{close, edit},
};
use strum::{Display, EnumVariantNames, FromRepr, VariantNames};

const DEFAULT_FILTER: &str = "(today | overdue)";

#[derive(clap::Parser, Debug)]
pub struct Params {
    #[clap(flatten)]
    filter: Filter,
    /// Disables interactive mode and simply displays the list.
    #[clap(short = 'n', long = "nointeractive")]
    nointeractive: bool,
}

#[derive(clap::Parser, Debug)]
pub struct Filter {
    /// When selecting tasks, this will specify a filter query to run against the Todoist API to narrow down possibilities.
    #[clap(short='f', long="filter", default_value_t=String::from(DEFAULT_FILTER))]
    filter: String,
}

/// TaskOrInteractive is a helper struct to be embedded into other Params so that they can perform
/// selections based on Task ID or selecting interactively.
#[derive(clap::Parser, Debug)]
pub struct TaskOrInteractive {
    /// The Task ID as provided from the Todoist API. Use `list` to find out what ID your task has.
    /// If omitted, will interactively select task.
    id: Option<TaskID>,
    #[clap(flatten)]
    filter: Filter,
}

impl TaskOrInteractive {
    pub fn with_id(id: TaskID) -> Self {
        Self {
            id: Some(id),
            filter: Filter {
                filter: DEFAULT_FILTER.to_string(),
            },
        }
    }
    pub async fn task_id(&self, gw: &Gateway) -> Result<TaskID> {
        match self.id {
            Some(id) => Ok(id),
            None => select_task(Some(&self.filter.filter), gw)
                .await?
                .map(|t| t.id)
                .ok_or_else(|| eyre!("no task selected")),
        }
    }
}

impl From<TaskID> for TaskOrInteractive {
    fn from(id: TaskID) -> Self {
        Self::with_id(id)
    }
}

/// List lists the tasks of the current user accessing the gateway with the given filter.
pub async fn list(params: Params, gw: &Gateway) -> Result<()> {
    let (tree, projects) = fetch_tree(Some(&params.filter.filter), gw).await?;
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

pub async fn select_task(filter: Option<&str>, gw: &Gateway) -> Result<Option<Tree<Task>>> {
    let (mut tree, projects) = fetch_tree(filter, gw).await?;
    let task = get_interactive_tasks(&tree, &projects)?
        .and_then(|task| tree.iter().position(|t| t == task))
        .map(|t| tree.swap_remove(t));
    Ok(task)
}

async fn fetch_tree(
    filter: Option<&str>,
    gw: &Gateway,
) -> Result<(Vec<Tree<Task>>, HashMap<ProjectID, Project>)> {
    let tasks = gw.tasks(filter).await?;
    let projects = gw
        .projects()
        .await?
        .into_iter()
        .map(|p| (p.id, p))
        .collect();
    Ok((
        Tree::from_items(tasks).wrap_err("tasks do not form clean tree")?,
        projects,
    ))
}

pub fn get_interactive_tasks<'a, 'b>(
    tree: &'a [Tree<Task>],
    projects: &'b HashMap<ProjectID, Project>,
) -> Result<Option<&'a Tree<Task>>> {
    if tree.is_empty() {
        return Err(eyre!("no tasks were found using the current filter"));
    }
    let items = tree.iter().flat_map(Tree::flatten).collect::<Vec<_>>();
    let result = dialoguer::FuzzySelect::with_theme(&dialoguer::theme::ColorfulTheme::default())
        .items(
            &items
                .iter()
                .map(|t| TableTask(t, projects.get(&t.project_id)))
                .collect::<Vec<_>>(),
        )
        .with_prompt("Select task")
        .default(0)
        .interact_opt()
        .wrap_err("Unable to make a selection")?;
    Ok(result.map(|index| items[index]))
}

fn list_tasks(tree: &[Tree<Task>], projects: &HashMap<ProjectID, Project>) {
    for task in tree.iter() {
        println!("{}", TableTask(task, projects.get(&task.project_id)));
        list_tasks(&task.subitems, projects);
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
                    task: task.id.into(),
                    complete: false,
                },
                gw,
            )
            .await?
        }
        TaskOptions::Complete => {
            close::close(
                close::Params {
                    task: task.id.into(),
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
