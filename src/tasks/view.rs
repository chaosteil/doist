use color_eyre::{Result, eyre::eyre};

use crate::{api::rest::Gateway, comments, config::Config, tasks::state::State};

use super::filter::TaskOrInteractive;

#[derive(clap::Parser, Debug)]
pub struct Params {
    #[clap(flatten)]
    task: TaskOrInteractive,
}

/// Displays full information about a task.
pub async fn view(params: Params, gw: &Gateway, cfg: &Config) -> Result<()> {
    // When we have a specific task ID, use "all" filter to ensure we can find it
    let filter = if params.task.id().is_some() {
        Some("all")
    } else {
        None // Use the default filter for interactive selection
    };
    
    // Fetch state with appropriate filter
    let state = if let Some(f) = filter {
        State::fetch_tree(Some(f), gw, cfg).await?
    } else {
        // Interactive - use the task's filter
        let (id, state) = params.task.task(gw, cfg).await?;
        let task = state.full_task(state.task(&id).ok_or_else(|| eyre!("no valid task"))?);
        println!("{task}");
        if task.0.comment_count > 0 {
            let comments = gw.task_comments(&id).await?;
            comments::list(&comments)
        }
        return Ok(());
    };
    
    // Get the task ID (either provided or selected)
    let id = params.task.id().cloned().ok_or_else(|| eyre!("no task ID"))?;
    
    // Find and display the task
    let task = state.task(&id).ok_or_else(|| eyre!("task not found"))?;
    let full_task = state.full_task(task);
    println!("{full_task}");
    if task.comment_count > 0 {
        let comments = gw.task_comments(&id).await?;
        comments::list(&comments)
    }
    Ok(())
}
