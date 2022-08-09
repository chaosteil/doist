use std::collections::HashMap;

use crate::api::rest::Gateway;
use color_eyre::{eyre::eyre, Result};

#[derive(clap::Parser, Debug)]
pub struct Params {
    /// If specified, will only show projects whose tasks are passing this filter.
    #[clap(short = 'f', long = "filter")]
    pub filter: Option<String>,
}

/// Lists available projects.
pub async fn list(params: Params, gw: &Gateway) -> Result<()> {
    if let Some(filter) = params.filter {
        return show_filtered_projects(&filter, gw).await;
    }
    show_projects(gw).await
}

async fn show_projects(gw: &Gateway) -> Result<()> {
    let projects = gw.projects().await?;
    for project in projects.iter() {
        println!("{}", &project);
    }
    Ok(())
}

async fn show_filtered_projects(filter: &str, gw: &Gateway) -> Result<()> {
    let tasks = gw.tasks(Some(filter)).await?;
    if tasks.is_empty() {
        return Err(eyre!("no tasks match the given filter"));
    }
    let hm = tasks
        .into_iter()
        .fold(HashMap::<_, usize>::new(), |mut hm, t| {
            *hm.entry(t.project_id).or_default() += 1;
            hm
        });
    let projects = gw
        .projects()
        .await?
        .into_iter()
        .filter(|p| hm.contains_key(&p.id))
        .collect::<Vec<_>>();
    for project in projects.iter() {
        println!("{} (Tasks: {})", &project, hm[&project.id]);
    }
    Ok(())
}
