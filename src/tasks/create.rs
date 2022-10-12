use color_eyre::Result;
use owo_colors::OwoColorize;
use strum::EnumIter;

use crate::{
    api::rest::{CreateTask, Gateway, TaskDue},
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
    Project = 3,
    Priority = 4,
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
                Selection::Project => "Project",
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
            3 => Selection::Project,
            4 => Selection::Priority,
            _ => panic!("bad selection input"),
        }
    }
}

pub async fn create(_params: Params, gw: &Gateway, cfg: &Config) -> Result<()> {
    let mut create = CreateTask {
        content: interactive::input_content("")?,
        ..Default::default()
    };

    let (projects, sections) = tokio::try_join!(gw.projects(), gw.sections())?;
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
                    Selection::Project,
                    match create
                        .project_id
                        .as_ref()
                        .and_then(|id| projects.iter().find(|p| p.id == *id))
                    {
                        Some(p) => match create
                            .section_id
                            .as_ref()
                            .and_then(|id| sections.iter().find(|s| s.id == *id))
                        {
                            Some(s) => format!("{}/{}", p, s),
                            None => p.to_string(),
                        },
                        None => "".to_string(),
                    },
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
            None => {
                println!("No selection was made");
                return Ok(());
            }
        };
        match selection {
            Selection::TaskName => create.content = interactive::input_content(&create.content)?,
            Selection::Due => due = interactive::input_optional("Due", due)?,
            Selection::Description => {
                create.description = interactive::input_optional("Description", create.description)?
            }
            Selection::Project => {
                if let Some((p, s)) = interactive::input_project(&projects, &sections)? {
                    create.project_id = Some(p);
                    create.section_id = s;
                };
            }
            Selection::Priority => create.priority = interactive::input_priority()?,
        }
    }
    if let Some(due) = due {
        create.due = Some(TaskDue::String(due));
    }
    create_task(create, None, None, &[], gw, cfg).await
}
