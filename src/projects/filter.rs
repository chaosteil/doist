use std::collections::HashMap;

use crate::{
    api::{
        rest::{Gateway, Project, ProjectID, Section, SectionID},
        tree::{Tree, TreeFlattenExt},
    },
    interactive,
};
use color_eyre::{
    eyre::{eyre, WrapErr},
    Result,
};

#[derive(clap::Parser, Debug)]
pub struct ProjectOrInteractive {
    id: Option<ProjectID>,
}

impl ProjectOrInteractive {
    pub fn with_id(id: ProjectID) -> Self {
        Self { id: Some(id) }
    }

    pub async fn project(&self, gw: &Gateway) -> Result<(ProjectID, List)> {
        let projects = List::fetch_tree(gw).await?;
        let id = match self.id {
            Some(id) => id,
            None => match projects.select_project()? {
                Some(project) => project.id,
                None => return Err(eyre!("no selection was made")),
            },
        };
        Ok((id, projects))
    }
}

pub struct List {
    projects: Vec<Tree<Project>>,
    sections: HashMap<SectionID, Section>,
}

impl List {
    pub async fn fetch_tree(gw: &Gateway) -> Result<List> {
        let (projects, sections) = tokio::try_join!(gw.projects(), gw.sections())?;
        let projects = Tree::from_items(projects).wrap_err("projects do not form a clean tree")?;
        let sections = sections.into_iter().map(|s| (s.id, s)).collect();
        Ok(List { projects, sections })
    }

    pub fn select_project(&self) -> Result<Option<&Tree<Project>>> {
        if self.projects.is_empty() {
            return Err(eyre!("no projects were found"));
        }
        let items = self.projects.flat_tree();
        let result = interactive::select(
            "Select project",
            &items.iter().map(|i| &i.item).collect::<Vec<_>>(),
        )?;
        Ok(result.map(|index| items[index]))
    }

    pub fn project(&self, id: ProjectID) -> Option<&Tree<Project>> {
        self.projects.find(id)
    }

    pub fn sections(&self, id: ProjectID) -> Vec<&Section> {
        self.sections
            .iter()
            .filter_map(|s| {
                if s.1.project_id == id {
                    Some(s.1)
                } else {
                    None
                }
            })
            .collect()
    }
}

impl From<ProjectID> for ProjectOrInteractive {
    fn from(id: ProjectID) -> Self {
        Self::with_id(id)
    }
}
