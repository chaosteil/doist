use std::collections::HashMap;

use color_eyre::Result;

use crate::{
    api::{
        rest::{CreateTask, Gateway, Project, Section, TableTask, TaskDue},
        tree::Tree,
    },
    interactive,
    labels::{self, LabelSelect},
    tasks::Priority,
};

#[derive(clap::Parser, Debug)]
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
    project: interactive::Selection<Project>,
    #[clap(flatten)]
    section: interactive::Selection<Section>,
    #[clap(flatten)]
    labels: LabelSelect,
}

pub async fn add(params: Params, gw: &Gateway) -> Result<()> {
    let (projects, sections) = tokio::try_join!(gw.projects(), gw.sections())?;
    let project = params.project.optional(&projects)?;
    let section = params.section.optional(&sections)?;
    let labels = params
        .labels
        .labels(&gw.labels().await?, labels::Selection::AllowEmpty)?;
    let mut create = CreateTask {
        content: params.name,
        description: params.desc,
        priority: params.priority.map(|p| p.into()),
        project_id: project.map(|p| p.id),
        section_id: section.map(|s| s.id),
        label_ids: labels.iter().map(|l| l.id).collect(),
        ..Default::default()
    };
    if let Some(due) = params.due {
        create.due = Some(TaskDue::String(due));
    }
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
    table.1 = project;
    table.2 = section;
    table.3 = labels.iter().collect();
    println!("created task: {}", table);
    Ok(())
}
