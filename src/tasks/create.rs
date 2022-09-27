use color_eyre::{eyre::eyre, eyre::WrapErr, Result};
use owo_colors::OwoColorize;
use strum::EnumIter;

use crate::{
    api::rest::{CreateTask, Gateway, Priority, TaskDue},
    config::Config,
    interactive,
};

use super::add::create_task;

#[derive(clap::Parser, Debug)]
pub struct Params {}

#[derive(EnumIter)]
#[repr(usize)]
enum Selection {
    TaskName = 0,
    Due = 1,
    Description = 2,
    Priority = 3,
}

impl std::fmt::Display for Selection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Selection::TaskName => "Task Name",
                Selection::Due => "Due",
                Selection::Description => "Description",
                Selection::Priority => "Priority",
            }
        )
    }
}

impl From<usize> for Selection {
    fn from(s: usize) -> Self {
        match s {
            0 => Selection::TaskName,
            1 => Selection::Due,
            2 => Selection::Description,
            3 => Selection::Priority,
            _ => panic!("bad selection input"),
        }
    }
}

pub async fn create(_params: Params, gw: &Gateway, cfg: &Config) -> Result<()> {
    let mut create = CreateTask {
        content: input_content("")?,
        ..Default::default()
    };

    let mut due: Option<String> = None;
    loop {
        let mut items = vec![format!("{}", "Submit".bold().bright_blue())];
        items.extend(
            [
                (Selection::TaskName, create.content.to_owned()),
                (Selection::Due, due.clone().unwrap_or_default()),
                (
                    Selection::Description,
                    create.description.clone().unwrap_or_default(),
                ),
                (
                    Selection::Priority,
                    create.priority.unwrap_or_default().to_string(),
                ),
            ]
            .iter()
            .map(|(name, content)| format!("{}: {}", name.bold(), content)),
        );
        let selection = match interactive::select("Edit task fields or submit", &items)? {
            Some(0) => break,
            Some(s) => Selection::from(s - 1),
            None => return Err(eyre!("No selection made")),
        };
        match selection {
            Selection::TaskName => create.content = input_content(&create.content)?,
            Selection::Due => due = input_optional("Due", due)?,
            Selection::Description => {
                create.description = input_optional("Description", create.description)?
            }
            Selection::Priority => create.priority = input_priority()?,
        }
    }
    if let Some(due) = due {
        create.due = Some(TaskDue::String(due));
    }
    create_task(create, None, None, &[], gw, cfg).await
}

fn input_content(content: &str) -> Result<String> {
    let mut input = dialoguer::Input::new();
    input
        .with_prompt("Task Name")
        .allow_empty(false)
        .validate_with(|input: &String| -> Result<(), &str> {
            if !input.is_empty() {
                Ok(())
            } else {
                Err("empty task description")
            }
        });
    if !content.is_empty() {
        input.with_initial_text(content.to_owned());
    }
    input.interact_text().wrap_err("No input made")
}

fn input_optional(prompt: &str, default: Option<String>) -> Result<Option<String>> {
    let mut input = dialoguer::Input::<'_, String>::new();
    input.with_prompt(prompt).allow_empty(true);
    if let Some(d) = default {
        input.with_initial_text(d);
    }
    match input.interact_text().wrap_err("No input made")?.as_str() {
        "" => Ok(None),
        s => Ok(Some(s.to_owned())),
    }
}

fn input_priority() -> Result<Option<Priority>> {
    let items = [
        Priority::Normal,
        Priority::High,
        Priority::VeryHigh,
        Priority::Urgent,
    ];
    let selection = interactive::select("Priority", &items)?;
    Ok(selection.map(|s| items[s]))
}
