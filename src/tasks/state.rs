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
    config::Config,
    interactive,
};

/// State is a helper to fully construct a tasks state for display.
pub struct State<'a> {
    pub tasks: Vec<Tree<Task>>,
    pub projects: HashMap<ProjectID, Project>,
    pub sections: HashMap<SectionID, Section>,
    pub labels: HashMap<LabelID, Label>,
    pub config: &'a Config,
}

impl<'a> State<'a> {
    pub async fn fetch_tree(
        filter: Option<&'_ str>,
        gw: &'_ Gateway,
        cfg: &'a Config,
    ) -> Result<State<'a>> {
        let (filtered_tasks, projects, sections, labels) =
            tokio::try_join!(gw.tasks(filter), gw.projects(), gw.sections(), gw.labels())?;
        let projects = projects.into_iter().map(|p| (p.id, p)).collect();
        let sections = sections.into_iter().map(|p| (p.id, p)).collect();
        let labels = labels.into_iter().map(|p| (p.id, p)).collect();
        let tasks = Tree::from_items(filtered_tasks).wrap_err("tasks do not form clean tree")?;
        Ok(State {
            tasks,
            projects,
            sections,
            labels,
            config: cfg,
        })
    }
    pub async fn fetch_full_tree(
        filter: Option<&'_ str>,
        gw: &'_ Gateway,
        cfg: &'a Config,
    ) -> Result<State<'a>> {
        let (mut state, all_tasks) =
            tokio::try_join!(Self::fetch_tree(filter, gw, cfg), gw.tasks(Some("all")))?;
        let all_tasks = Tree::from_items(all_tasks).wrap_err("tasks do not form clean tree")?;
        let tasks = all_tasks.keep_trees(&state.tasks.iter().map(|t| t.id).collect::<Vec<_>>());

        state.tasks = tasks;
        Ok(state)
    }

    pub fn task(&self, id: TaskID) -> Option<&Tree<Task>> {
        self.tasks.find(id)
    }

    pub fn select_task(&self) -> Result<Option<&Tree<Task>>> {
        if self.tasks.is_empty() {
            return Err(eyre!("no tasks were found using the current filter"));
        }
        let items = self.tasks.flat_tree();
        let result = interactive::select(
            "Select task",
            &items.iter().map(|t| self.table_task(t)).collect::<Vec<_>>(),
        )?;
        Ok(result.map(|index| items[index]))
    }

    pub fn filter<F>(self, filter: F) -> State<'a>
    where
        F: Fn(&Tree<Task>) -> bool,
    {
        let tasks: Vec<_> = self.tasks.into_iter().filter(&filter).collect();
        State {
            tasks,
            projects: self.projects,
            sections: self.sections,
            labels: self.labels,
            config: self.config,
        }
    }

    fn project<'s>(&'s self, task: &'s Tree<Task>) -> Option<&'s Project> {
        self.projects.get(&task.project_id)
    }

    fn section<'s>(&'s self, task: &'s Tree<Task>) -> Option<&'s Section> {
        task.section_id
            .as_ref()
            .map(|s| self.sections.get(s).unwrap())
    }

    fn labels<'s>(&'s self, task: &'s Tree<Task>) -> Vec<&'s Label> {
        task.label_ids
            .iter()
            .map(|l| self.labels.get(l).unwrap())
            .collect()
    }

    pub fn table_task<'s>(&'s self, task: &'s Tree<Task>) -> TableTask {
        TableTask(
            task,
            self.project(task),
            self.section(task),
            self.labels(task),
            self.config,
        )
    }

    pub fn full_task<'s>(&'s self, task: &'s Tree<Task>) -> FullTask {
        FullTask(
            task,
            self.project(task),
            self.section(task),
            self.labels(task),
            self.config,
        )
    }
}
