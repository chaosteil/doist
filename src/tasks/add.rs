use std::collections::HashMap;

use color_eyre::Result;

use crate::{
    api::{
        rest::{CreateTask, Gateway, Label, Project, Section, TableTask, TaskDue},
        tree::Tree,
    },
    config::Config,
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
    #[arg(short = 'd', long = "due")]
    due: Option<String>,
    /// Description that has more details about the task.
    #[arg(short = 'D', long = "desc")]
    desc: Option<String>,
    /// Sets the priority on the task. The higher the priority the more urgent the task.
    #[arg(value_enum, short = 'p', long = "priority")]
    priority: Option<Priority>,
    #[clap(flatten)]
    project: interactive::Selection<Project>,
    #[clap(flatten)]
    section: interactive::Selection<Section>,
    #[clap(flatten)]
    labels: LabelSelect,
}

pub async fn add(params: Params, gw: &Gateway, cfg: &Config) -> Result<()> {
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
        project_id: project.map(|p| p.id.clone()),
        section_id: section.map(|s| s.id.clone()),
        labels: labels.iter().map(|l| l.name.clone()).collect(),
        ..Default::default()
    };
    if let Some(due) = params.due {
        create.due = Some(TaskDue::String(due));
    }
    let labels = if !create.labels.is_empty() {
        let mut labels: HashMap<_, _> = gw
            .labels()
            .await?
            .into_iter()
            .map(|label| (label.name.clone(), label))
            .collect();
        create
            .labels
            .iter()
            .filter_map(|l| labels.remove(l))
            .collect()
    } else {
        Vec::new()
    };
    create_task(create, project, section, &labels, gw, cfg).await
}

pub(super) async fn create_task(
    create: CreateTask,
    project: Option<&Project>,
    section: Option<&Section>,
    labels: &[Label],
    gw: &Gateway,
    cfg: &Config,
) -> Result<()> {
    let task = Tree::new(gw.create(&create).await?);
    let mut table = TableTask::from_task(&task, cfg);
    table.1 = project;
    table.2 = section;
    table.3 = labels.iter().collect();
    println!("created task: {}", table);
    Ok(())
}
