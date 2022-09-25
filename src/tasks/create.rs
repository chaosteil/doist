use color_eyre::{eyre::WrapErr, Result};
use owo_colors::OwoColorize;

use crate::{
    api::rest::{CreateTask, Gateway, Priority, TaskDue},
    config::Config,
    interactive,
};

#[derive(clap::Parser, Debug)]
pub struct Params {}

pub async fn create(params: Params, gw: &Gateway, cfg: &Config) -> Result<()> {
    let (projects, sections, labels) = tokio::try_join!(gw.projects(), gw.sections(), gw.labels())?;

    let mut create = CreateTask::default();

    let mut input = dialoguer::Input::new();
    input
        .with_prompt("Task name")
        .allow_empty(false)
        .validate_with(|input: &String| -> Result<(), &str> {
            if !input.is_empty() {
                Ok(())
            } else {
                Err("empty task description")
            }
        });
    create.content = input.interact_text().wrap_err("No input made")?;
    let mut due: Option<String> = None;

    let items = vec![
        format!("{}", "Submit".bold().bright_blue()),
        format!("{}: {}", "Task Name".bold(), create.content),
        format!(
            "{}: {}",
            "Due".bold(),
            due.as_ref().unwrap_or(&"".to_owned())
        ),
        format!(
            "{}: {}",
            "Description".bold(),
            create.description.unwrap_or_default()
        ),
        format!(
            "{}: {}",
            "Priority".bold(),
            create.priority.unwrap_or(Priority::Normal)
        ),
    ];
    let selection = interactive::select("Edit task fields or submit", &items)?;
    //
    // let mut create = CreateTask {
    //     content: params.name,
    //     description: params.desc,
    //     priority: params.priority.map(|p| p.into()),
    //     project_id: project.map(|p| p.id),
    //     section_id: section.map(|s| s.id),
    //     label_ids: labels.iter().map(|l| l.id).collect(),
    //     ..Default::default()
    // };
    if let Some(due) = due {
        create.due = Some(TaskDue::String(due));
    }
    Ok(())
}
