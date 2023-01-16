use std::collections::HashMap;

use crate::api::rest::{Gateway, Project, Task};
use color_eyre::{eyre::eyre, Result};

#[derive(clap::Parser, Debug)]
pub struct Params {
    /// If specified, will only show projects whose tasks are passing this filter.
    #[arg(short = 'f', long = "filter")]
    pub filter: Option<String>,
}

/// Lists available projects.
pub async fn list(params: Params, gw: &Gateway) -> Result<()> {
    let projects = gw.projects().await?;
    if let Some(filter) = params.filter {
        let tasks = gw.tasks(Some(&filter)).await?;
        if tasks.is_empty() {
            return Err(eyre!("no tasks match the given filter"))?;
        }
        let projects = filtered_projects(&projects, &tasks)?;
        for (project, tasks) in projects.iter() {
            println!("{} (Tasks: {})", &project, tasks);
        }
        return Ok(());
    }
    for project in projects.iter() {
        println!("{}", &project);
    }
    Ok(())
}

fn filtered_projects<'a>(
    projects: &'a [Project],
    tasks: &'_ [Task],
) -> Result<Vec<(&'a Project, usize)>> {
    let hm = tasks.iter().fold(HashMap::<_, usize>::new(), |mut hm, t| {
        *hm.entry(&t.project_id).or_default() += 1;
        hm
    });
    let projects = projects
        .iter()
        .filter_map(|p| hm.get(&p.id).map(|tasks| (p, *tasks)))
        .collect();
    Ok(projects)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::api::rest::{Project, ProjectID, Task, TaskID};

    #[tokio::test]
    async fn filter_projects() {
        let ps = vec![
            Project::new("1", "one"),
            Project::new("2", "two"),
            Project::new("3", "three"),
        ];
        let ts = vec![
            create_task("1", "1", "one"),
            create_task("2", "1", "two"),
            create_task("3", "2", "three"),
        ];
        let projects = filtered_projects(&ps, &ts).unwrap();
        assert_eq!(projects.len(), 2);
        assert_eq!(projects[0].0.id, "1");
        assert_eq!(projects[0].1, 2);
        assert_eq!(projects[1].0.id, "2");
        assert_eq!(projects[1].1, 1);
    }

    fn create_task(id: &str, project_id: &str, content: &str) -> Task {
        let mut task = crate::api::rest::Task::new(id, content);
        task.project_id = project_id.to_string();
        task
    }
}
