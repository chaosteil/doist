use std::collections::HashMap;

use color_eyre::Result;
use serde::{Deserialize, Serialize};

use crate::{
    api::{
        rest::{CreateTask, Gateway, TableTask, TaskDue},
        tree::Tree,
    },
    tasks::Priority,
};

use super::{label::LabelSelect, project::ProjectSelect, section::SectionSelect};

#[derive(clap::Parser, Debug, Deserialize, Serialize)]
pub struct Params {
    /// Name (title) of the task to add to the todo list.
    name: String,
    /// Set due with a human-readable text.
    ///
    /// Examples: "in two days" "tomorrow", "every 2 days from Monday"
    #[clap(short = 'd', long = "due")]
    due: Option<String>,
    /// Description that has more details about the task.
    #[clap(short = 'D', long = "desc")]
    desc: Option<String>,
    /// Sets the priority on the task. The higher the priority the more urgent the task.
    #[clap(value_enum, short = 'p', long = "priority")]
    priority: Option<Priority>,
    #[clap(flatten)]
    project: ProjectSelect,
    #[clap(flatten)]
    section: SectionSelect,
    #[clap(flatten)]
    labels: LabelSelect,
}

pub async fn add(params: Params, gw: &Gateway) -> Result<()> {
    let project_id = params.project.project(gw).await?;
    let section_id = params.section.section(project_id, gw).await?;
    let label_ids = params.labels.labels(gw).await?;
    let mut create = CreateTask {
        content: params.name,
        description: params.desc,
        priority: params.priority.map(|p| p.into()),
        project_id,
        section_id,
        label_ids,
        ..Default::default()
    };
    if let Some(due) = params.due {
        create.due = Some(TaskDue::String(due));
    }
    let project = match project_id {
        Some(pid) => Some(gw.project(pid).await?),
        None => None,
    };
    let section = match section_id {
        Some(sid) => Some(gw.section(sid).await?),
        None => None,
    };
    let labels = if !create.label_ids.is_empty() {
        let mut labels: HashMap<_, _> = gw
            .labels()
            .await?
            .into_iter()
            .map(|label| (label.id, label))
            .collect();
        create
            .label_ids
            .iter()
            .filter_map(|l| labels.remove(l))
            .collect()
    } else {
        Vec::new()
    };
    let task = Tree::new(gw.create(&create).await?);
    let mut table = TableTask::from_task(&task);
    table.1 = project.as_ref();
    table.2 = section.as_ref();
    table.3 = labels.iter().collect();
    println!("created task: {}", table);
    Ok(())
}
