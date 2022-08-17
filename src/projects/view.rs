use crate::{
    api::rest::{Gateway, Project},
    comments, interactive,
    projects::filter::List,
};
use color_eyre::{eyre::eyre, Result};

#[derive(clap::Parser, Debug)]
pub struct Params {
    #[clap(flatten)]
    project: interactive::Selection<Project>,
}

pub async fn view(params: Params, gw: &Gateway) -> Result<()> {
    let projects = gw.projects().await?;
    let project = params.project.mandatory(&projects)?;
    // TODO: no refetch here
    let list = List::fetch_tree(gw).await?;
    let tree = list
        .project(project.id)
        .ok_or_else(|| eyre!("full project list contained invalid data"))?;
    println!("Project: {}", &tree.item);
    if !tree.subitems.is_empty() {
        println!("Subprojects:");
        for project in &tree.subitems {
            println!("{}", project.item)
        }
    }
    let sections = list.sections(project.id);
    if !sections.is_empty() {
        println!("Sections:");
        for section in sections {
            println!("{}", section)
        }
    }
    if project.comment_count > 0 {
        let comments = gw.project_comments(project.id).await?;
        comments::list(&comments)
    }
    Ok(())
}
