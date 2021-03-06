use crate::{
    api::rest::Gateway,
    config::Config,
    projects,
    tasks::{add, close, comment, edit, list, view},
};
use clap::{Parser, Subcommand};
use color_eyre::{eyre::eyre, Result};

/// Args are the main entry point struct of the CLI app.
#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Args {
    #[clap(subcommand)]
    command: Commands,
}

/// All commands available for the CLI app.
#[derive(Subcommand, Debug)]
enum Commands {
    /// Authenticates with the Todoist API.
    Auth {
        /// The Todoist API token.
        /// This can be taken from the Todoist client by going into
        /// Settings -> Integrations -> API token
        token: String,
    },
    /// Authenticated commands are commands that require a token to be set up via the Auth command
    /// before executing.
    #[clap(flatten)]
    Authenticated(AuthCommands),
}

#[derive(Subcommand, Debug)]
enum AuthCommands {
    /// Adds a task.
    #[clap(alias = "a")]
    Add(add::Params),
    /// Lists tasks.
    #[clap(alias = "l")]
    List(list::Params),
    /// Edits a task.
    #[clap(alias = "e")]
    Edit(edit::Params),
    /// Closes a task.
    #[clap(alias = "c")]
    Close(close::Params),
    /// View details of a single task.
    #[clap(alias = "v")]
    View(view::Params),
    /// Add a comment on a task.
    #[clap(alias = "C")]
    Comment(comment::Params),

    /// Manages projects.
    #[clap(subcommand, alias = "p")]
    Projects(ProjectCommands),
}

#[derive(Subcommand, Debug)]
enum ProjectCommands {
    /// Lists all current projects
    #[clap(alias = "l")]
    List(projects::list::Params),
    /// View details of a single project.
    #[clap(alias = "v")]
    View(projects::view::Params),
    /// Add a comment on a project.
    #[clap(alias = "C")]
    Comment(projects::comment::Params),
}

impl Args {
    /// Runs the CLI app.
    pub async fn exec(self) -> Result<()> {
        let mut cfg = Config::load()?;
        match self.command {
            Commands::Auth { token } => {
                cfg.token = Some(token);
                cfg.save()?;
                println!("Token successfully saved")
            }
            Commands::Authenticated(command) => {
                let token = cfg.token.ok_or_else(|| {
                    eyre!("No token in config specified. Use `doist auth` to register your token.")
                })?;
                let gw = Gateway::new(&token, cfg.url);
                match command {
                    AuthCommands::Add(p) => add::add(p, &gw).await?,
                    AuthCommands::List(p) => list::list(p, &gw).await?,
                    AuthCommands::Edit(p) => edit::edit(p, &gw).await?,
                    AuthCommands::Close(p) => close::close(p, &gw).await?,
                    AuthCommands::View(p) => view::view(p, &gw).await?,
                    AuthCommands::Comment(p) => comment::comment(p, &gw).await?,
                    AuthCommands::Projects(p) => match p {
                        ProjectCommands::List(p) => projects::list::list(p, &gw).await?,
                        ProjectCommands::View(p) => projects::view::view(p, &gw).await?,
                        ProjectCommands::Comment(p) => projects::comment::comment(p, &gw).await?,
                    },
                }
            }
        }
        Ok(())
    }
}
