use std::collections::HashMap;

use color_eyre::{eyre::eyre, eyre::WrapErr, Result};

use crate::{
    api::{
        rest::{
            FullTask, Gateway, Label, LabelID, Project, ProjectID, Section, SectionID, TableTask,
            Task, TaskID,
        },
        tree::{Tree, TreeFlattenExt},
    },
    interactive, labels,
    tasks::{close, edit, filter, project, section},
};
use strum::{Display, EnumVariantNames, FromRepr, VariantNames};

#[derive(clap::Parser, Debug)]
pub struct Params {
    #[clap(flatten)]
    filter: filter::Filter,
    /// Disables interactive mode and simply displays the list.
    #[clap(short = 'n', long = "nointeractive")]
    nointeractive: bool,
    #[clap(flatten)]
    project: project::ProjectSelect,
    #[clap(flatten)]
    section: section::SectionSelect,
    #[clap(flatten)]
    label: labels::LabelSelect,
}

/// List is a helper to fully construct a tasks state for display.
/// TODO: rename and move to some other module.
pub struct List {
    tasks: Vec<Tree<Task>>,
    projects: HashMap<ProjectID, Project>,
    sections: HashMap<SectionID, Section>,
    labels: HashMap<LabelID, Label>,
}

impl List {
    pub async fn fetch_tree(filter: Option<&str>, gw: &Gateway) -> Result<List> {
        let (tasks, projects, sections, labels) =
            tokio::try_join!(gw.tasks(filter), gw.projects(), gw.sections(), gw.labels())?;
        let projects = projects.into_iter().map(|p| (p.id, p)).collect();
        let sections = sections.into_iter().map(|p| (p.id, p)).collect();
        let labels = labels.into_iter().map(|p| (p.id, p)).collect();
        let tasks = Tree::from_items(tasks).wrap_err("tasks do not form clean tree")?;
        Ok(List {
            tasks,
            projects,
            sections,
            labels,
        })
    }

    pub fn task(&self, id: TaskID) -> Option<&Tree<Task>> {
        self.tasks.find(id)
    }

    pub fn select_task(&self) -> Result<Option<&Tree<Task>>> {
        get_interactive_tasks(self)
    }

    pub fn filter<F>(self, filter: F) -> List
    where
        F: Fn(&Tree<Task>) -> bool,
    {
        let tasks: Vec<_> = self.tasks.into_iter().filter(&filter).collect();
        List {
            tasks,
            projects: self.projects,
            sections: self.sections,
            labels: self.labels,
        }
    }

    fn project<'a>(&'a self, task: &'a Tree<Task>) -> Option<&'a Project> {
        self.projects.get(&task.project_id)
    }

    fn section<'a>(&'a self, task: &'a Tree<Task>) -> Option<&'a Section> {
        task.section_id
            .as_ref()
            .map(|s| self.sections.get(s).unwrap())
    }

    fn labels<'a>(&'a self, task: &'a Tree<Task>) -> Vec<&'a Label> {
        task.label_ids
            .iter()
            .map(|l| self.labels.get(l).unwrap())
            .collect()
    }

    pub fn table_task<'a>(&'a self, task: &'a Tree<Task>) -> TableTask {
        TableTask(
            task,
            self.project(task),
            self.section(task),
            self.labels(task),
        )
    }

    pub fn full_task<'a>(&'a self, task: &'a Tree<Task>) -> FullTask {
        FullTask(
            task,
            self.project(task),
            self.section(task),
            self.labels(task),
        )
    }
}

/// List lists the tasks of the current user accessing the gateway with the given filter.
pub async fn list(params: Params, gw: &Gateway) -> Result<()> {
    let list = filter_list(
        List::fetch_tree(Some(&params.filter.filter), gw).await?,
        &params,
        gw,
    )
    .await?;
    if params.nointeractive {
        list_tasks(&list.tasks, &list);
    } else {
        match get_interactive_tasks(&list)? {
            Some(task) => select_task_option(task, &list, gw).await?,
            None => println!("No selection was made"),
        }
    }
    Ok(())
}

async fn filter_list(list: List, params: &Params, gw: &Gateway) -> Result<List> {
    let project = params.project.project(gw).await?;
    let section = params.section.section(project, gw).await?;
    let labels = params
        .label
        .labels(gw, labels::Selection::AllowEmpty)
        .await?;
    let mut list = list;
    if let Some(id) = project {
        list = list.filter(|tree| tree.project_id == id);
    }
    if let Some(id) = section {
        list = list.filter(|tree| tree.section_id == Some(id));
    }
    if !labels.is_empty() {
        list = list.filter(|tree| {
            labels
                .iter()
                .map(|l| l.id)
                .any(|l| tree.label_ids.contains(&l))
        });
    }
    Ok(list)
}

fn get_interactive_tasks(list: &List) -> Result<Option<&Tree<Task>>> {
    if list.tasks.is_empty() {
        return Err(eyre!("no tasks were found using the current filter"));
    }
    let items = list.tasks.flat_tree();
    let result = interactive::select(
        "Select task",
        &items.iter().map(|t| list.table_task(t)).collect::<Vec<_>>(),
    )?;
    Ok(result.map(|index| items[index]))
}

fn list_tasks<'a>(tasks: &'a [Tree<Task>], list: &'a List) {
    for task in tasks.iter() {
        println!("{}", list.table_task(task));
        list_tasks(&task.subitems, list);
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
    list: &'a List,
    gw: &'b Gateway,
) -> Result<()> {
    println!("{}", list.full_task(task));
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
    // Project, TODO: allow to edit project and section
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
