use crate::{
    api::rest::{Gateway, Project},
    comments, interactive,
    projects::state::State,
};
use color_eyre::{Result, eyre::eyre};

#[derive(clap::Parser, Debug)]
pub struct Params {
    #[clap(flatten)]
    project: interactive::Selection<Project>,
}

pub async fn view(params: Params, gw: &Gateway) -> Result<()> {
    let projects = gw.projects().await?;
    let project = params.project.mandatory(&projects)?;
    // TODO: no refetch here
    let state = State::fetch_tree(gw).await?;
    let tree = state
        .project(&project.id)
        .ok_or_else(|| eyre!("full project list contained invalid data"))?;
    println!("Project: {}", &tree.item);
    if !tree.subitems.is_empty() {
        println!("Subprojects:");
        for project in &tree.subitems {
            println!("{}", project.item)
        }
    }
    let sections = state.sections(&project.id);
    if !sections.is_empty() {
        println!("Sections:");
        for section in sections {
            println!("{section}")
        }
    }
    let comments = gw.project_comments(&project.id).await?;
    if !comments.is_empty() {
        comments::list(&comments)
    }
    Ok(())
}
