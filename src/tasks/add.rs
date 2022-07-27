use color_eyre::{eyre::eyre, Result};
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use serde::{Deserialize, Serialize};

use crate::{
    api::{
        rest::{CreateTask, Gateway, ProjectID, TableTask, TaskDue},
        tree::Tree,
    },
    tasks::Priority,
};

#[derive(clap::Parser, Debug, Deserialize, Serialize)]
pub struct Params {
    /// Name (title) of the task to add to the todo list.
    name: String,
    /// Set due with a human-readable text.
    ///
    /// Examples: "in two days" "tomorrow", "every 2 days from Monday"
    #[clap(short = 'd')]
    due: Option<String>,
    /// Description that has more details about the task.
    desc: Option<String>,
    /// Sets the priority on the task. The higher the priority the more urgent the task.
    #[clap(value_enum, short = 'p', long = "priority")]
    priority: Option<Priority>,
    #[clap(flatten)]
    project: ProjectSelect,
}

#[derive(clap::Args, Debug, Serialize, Deserialize)]
struct ProjectSelect {
    /// Assigns the project name with the closest name, if possible. Does fuzzy matching for the
    /// name.
    #[clap(short = 'P', long = "project")]
    project_name: Option<String>,
    /// ID of the project to attach this task to. Does nothing if -P is specified.
    #[clap(long = "project_id")]
    project_id: Option<ProjectID>,
}

impl ProjectSelect {
    // TODO: make this reusable by edit etc. into own module
    // TODO: make this generic over anything that has string input and todoist ID output
    async fn project(&self, gw: &Gateway) -> Result<Option<ProjectID>> {
        if self.project_name.is_none() {
            return Ok(self.project_id);
        }
        let input = self.project_name.as_ref().unwrap();
        let matcher = SkimMatcherV2::default();
        let projects = gw.projects().await?;
        let project = projects
            .iter()
            .filter_map(|p| matcher.fuzzy_match(&p.name, input).map(|s| (s, p.id)))
            .max_by(|left, right| left.0.cmp(&right.0));
        match project {
            Some((_, id)) => Ok(Some(id)),
            None => Err(eyre!("no suitable project found, aborting")),
        }
    }
}

pub async fn add(params: Params, gw: &Gateway) -> Result<()> {
    let project_id = params.project.project(gw).await?;
    let mut create = CreateTask {
        content: params.name,
        description: params.desc,
        priority: params.priority.map(|p| p.into()),
        project_id,
        ..Default::default()
    };
    if let Some(due) = params.due {
        create.due = Some(TaskDue::String(due));
    }
    let project = match project_id {
        Some(pid) => Some(gw.project(pid).await?),
        None => None,
    };
    let task = Tree::new(gw.create(&create).await?);
    let mut table = TableTask::from_task(&task);
    table.1 = project.as_ref();
    println!("created task: {}", table);
    Ok(())
}
