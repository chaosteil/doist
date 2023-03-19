use std::ops::Not;

use crate::{
    api::{
        rest::{Gateway, Project, Section, Task},
        tree::Tree,
    },
    config::Config,
    interactive, labels,
    tasks::{
        close, edit, filter,
        state::{State, TaskMenu},
    },
};
use color_eyre::{eyre::WrapErr, Result};
use owo_colors::OwoColorize;
use strum::{Display, EnumVariantNames, FromRepr, VariantNames};

use super::create;

#[derive(clap::Parser, Debug)]
pub struct Params {
    #[clap(flatten)]
    filter: filter::Filter,
    /// Disables interactive mode and simply displays the list.
    #[arg(short = 'n', long = "nointeractive")]
    nointeractive: bool,
    #[clap(flatten)]
    project: interactive::Selection<Project>,
    #[clap(flatten)]
    section: interactive::Selection<Section>,
    #[clap(flatten)]
    label: labels::LabelSelect,
    /// Expands to show all parents of tasks that are in the filter, even if the parent doesn't
    /// match the filter.
    #[arg(short = 'e', long = "expand")]
    expand: bool,
    /// Enables a continuous super-interactive mode, so that after each operation more operations
    /// can be done until the program is exited from.
    #[arg(short = 'i', long = "interactive")]
    continuous: bool,
}

/// List lists the tasks of the current user accessing the gateway with the given filter.
pub async fn list(params: Params, gw: &Gateway, cfg: &Config) -> Result<()> {
    if params.continuous && !params.nointeractive {
        return list_interactive(params, gw, cfg).await;
    }
    match list_action(&params, gw, cfg).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

async fn list_action(params: &Params, gw: &Gateway, cfg: &Config) -> Result<()> {
    let state = if params.expand {
        State::fetch_full_tree(Some(&params.filter.select(cfg)), gw, cfg).await
    } else {
        State::fetch_tree(Some(&params.filter.select(cfg)), gw, cfg).await
    }?;
    let state = filter_list(state, params).await?;
    if params.nointeractive {
        list_tasks(&state.tasks, &state);
    } else {
        match state.select_task()? {
            Some(task) => select_task_option(task, &state, gw).await?,
            None => {
                println!("No selection was made");
            }
        }
    }
    Ok(())
}

async fn list_interactive(params: Params, gw: &Gateway, cfg: &Config) -> Result<()> {
    let mut params = params;
    loop {
        match list_interactive_action(&mut params, gw, cfg).await {
            Ok(ListAction::Cancel) => return Ok(()),
            Ok(_) => {}
            Err(e) => return Err(e),
        }
    }
}

/// Describes the action the user made when calling [`list_interactive_action`].
pub enum ListAction {
    Action,
    Cancel,
}

async fn list_interactive_action(
    params: &mut Params,
    gw: &Gateway,
    cfg: &Config,
) -> Result<ListAction> {
    let filter = params.filter.select(cfg);
    let state = if params.expand {
        State::fetch_full_tree(Some(&filter), gw, cfg).await
    } else {
        State::fetch_tree(Some(&filter), gw, cfg).await
    }?;

    let state = filter_list(state, params).await?;
    match state.select_or_menu()? {
        TaskMenu::Menu => {
            match interactive::select(
                "Select Action:",
                &[
                    "Create Task...",
                    &format!(
                        "Set Filter{}...",
                        filter
                            .is_empty()
                            .not()
                            .then(|| format!(" ({})", filter.yellow()))
                            .unwrap_or_default()
                    ),
                    "| Show All Tasks",
                    "| Inbox",
                    "| Upcoming",
                    "| Default Filter",
                ],
            )? {
                // TODO change this once we have async closures and can iterate over a Vec<(str, async Fn)>
                Some(0) => create::create(create::Params {}, gw, cfg).await?,
                Some(1) => {
                    let filter = filter.is_empty().not().then_some(filter);
                    params.filter.set_filter(Some(
                        &interactive::input_optional("Filter", filter)?.unwrap_or_default(),
                    ));
                }
                Some(2) => params.filter.set_filter(Some("all")),
                Some(3) => params.filter.set_filter(Some("#inbox")),
                Some(4) => params.filter.set_filter(Some(&cfg.default_filter)),
                Some(5) => params.filter.set_filter(Some("(today | overdue)")),
                Some(_) => unreachable!(),
                None => {}
            };
            Ok(ListAction::Action)
        }
        TaskMenu::Select(task) => {
            select_task_option(task, &state, gw).await?;
            Ok(ListAction::Action)
        }
        TaskMenu::None => {
            println!("No selection was made");
            Ok(ListAction::Cancel)
        }
    }
}

/// Show a list that's filtered down based on the params.
async fn filter_list<'a>(state: State<'a>, params: &'_ Params) -> Result<State<'a>> {
    let projects = state
        .projects
        .values()
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();
    let sections = state
        .sections
        .values()
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();
    let labels = state
        .labels
        .values()
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();
    let project = params.project.optional(&projects)?;
    let section = params.section.optional(&sections)?;
    let labels = params
        .label
        .labels(&labels, labels::Selection::AllowEmpty)?;
    let mut state = state;
    if let Some(p) = project {
        state = state.filter(|tree| tree.project_id == *p.id);
    }
    if let Some(s) = section {
        state = state.filter(|tree| tree.section_id.as_ref() == Some(&s.id));
    }
    if !labels.is_empty() {
        state = state.filter(|tree| {
            labels
                .iter()
                .map(|l| l.id.clone())
                .any(|l| tree.labels.contains(&l))
        });
    }
    Ok(state)
}

fn list_tasks<'a>(tasks: &'a [Tree<Task>], state: &'a State) {
    let mut tasks = tasks.to_vec();
    tasks.sort();
    for task in tasks.iter() {
        println!("{}", state.table_task(task));
        list_tasks(&task.subitems, state);
    }
}

#[derive(Display, FromRepr, EnumVariantNames)]
enum TaskOptions {
    Close,
    Complete,
    Edit,
    Quit,
}

async fn select_task_option<'a, 'b>(
    task: &'a Tree<Task>,
    state: &'a State<'_>,
    gw: &'b Gateway,
) -> Result<()> {
    println!("{}", state.full_task(task));
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
                    task: task.id.clone().into(),
                    complete: false,
                },
                gw,
                state.config,
            )
            .await?
        }
        TaskOptions::Complete => {
            close::close(
                close::Params {
                    task: task.id.clone().into(),
                    complete: true,
                },
                gw,
                state.config,
            )
            .await?
        }
        TaskOptions::Edit => edit_task(task, gw, state.config).await?,
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
    // Project, TODO: allow to edit project and section when API supports it
    // TODO: allow adding, removing labels
    Quit,
}

async fn edit_task(task: &Tree<Task>, gw: &Gateway, cfg: &Config) -> Result<()> {
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
            let mut params = edit::Params::new(task.id.clone());
            params.priority = Some(selection.try_into()?);
            edit::edit(params, gw, cfg).await?;
        }
        _ => {
            let text = dialoguer::Input::new()
                .with_prompt("New value")
                .interact_text()
                .wrap_err("Bad user input")?;
            let mut params = edit::Params::new(task.id.clone());
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
            edit::edit(params, gw, cfg).await?;
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
