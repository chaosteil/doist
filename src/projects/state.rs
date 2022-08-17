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

pub struct State {
    projects: Vec<Tree<Project>>,
    sections: HashMap<SectionID, Section>,
}

impl State {
    pub async fn fetch_tree(gw: &Gateway) -> Result<State> {
        let (projects, sections) = tokio::try_join!(gw.projects(), gw.sections())?;
        let projects = Tree::from_items(projects).wrap_err("projects do not form a clean tree")?;
        let sections = sections.into_iter().map(|s| (s.id, s)).collect();
        Ok(State { projects, sections })
    }

    pub fn _select_project(&self) -> Result<Option<&Tree<Project>>> {
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
