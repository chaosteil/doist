use crate::{
    config::Config,
    labels, projects, sections,
    tasks::{add, close, comment, edit, list, view},
};
use clap::{AppSettings, Parser, Subcommand};
use color_eyre::Result;

/// Args are the main entry point struct of the CLI app.
#[derive(Parser, Debug)]
#[clap(author, version, about)]
#[clap(global_setting(AppSettings::ArgsNegateSubcommands))]
pub struct Args {
    #[clap(subcommand)]
    command: Option<Commands>,
    #[clap(flatten)]
    params: list::Params,
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
    /// Lists tasks. This is the default if no subcommand is specified.
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
    /// Manages labels.
    #[clap(subcommand, alias = "l")]
    Labels(LabelCommands),
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
    /// Adds (creates) a new project.
    #[clap(alias = "a")]
    Add(projects::add::Params),
    /// Deletes a project
    #[clap(alias = "d")]
    Delete(projects::delete::Params),

    /// Manages sections.
    #[clap(subcommand, alias = "s")]
    Sections(SectionCommands),
}

#[derive(Subcommand, Debug)]
enum LabelCommands {
    /// Lists all current labels.
    #[clap(alias = "l")]
    List(labels::list::Params),
    /// Adds (creates) a new label.
    #[clap(alias = "a")]
    Add(labels::add::Params),
    /// Deletes a label.
    #[clap(alias = "d")]
    Delete(labels::delete::Params),
}

#[derive(Subcommand, Debug)]
enum SectionCommands {
    /// Lists all current sections of the project.
    #[clap(alias = "l")]
    List(sections::list::Params),
    /// Adds (creates) a new section in a project.
    #[clap(alias = "a")]
    Add(sections::add::Params),
    /// Deletes a section in a project.
    #[clap(alias = "d")]
    Delete(sections::delete::Params),
}

impl Args {
    /// Runs the CLI app.
    pub async fn exec(self) -> Result<()> {
        let mut cfg = Config::load()?;
        match self.command {
            Some(command) => match command {
                Commands::Auth { token } => {
                    cfg.token = Some(token);
                    cfg.save()?;
                    println!("Token successfully saved")
                }
                Commands::Authenticated(command) => {
                    let gw = cfg.gateway()?;
                    match command {
                        AuthCommands::Add(p) => add::add(p, &gw, &cfg).await?,
                        AuthCommands::List(p) => list::list(p, &gw, &cfg).await?,
                        AuthCommands::Edit(p) => edit::edit(p, &gw, &cfg).await?,
                        AuthCommands::Close(p) => close::close(p, &gw, &cfg).await?,
                        AuthCommands::View(p) => view::view(p, &gw, &cfg).await?,
                        AuthCommands::Comment(p) => comment::comment(p, &gw, &cfg).await?,
                        AuthCommands::Projects(p) => match p {
                            ProjectCommands::List(p) => projects::list::list(p, &gw).await?,
                            ProjectCommands::View(p) => projects::view::view(p, &gw).await?,
                            ProjectCommands::Comment(p) => {
                                projects::comment::comment(p, &gw).await?
                            }
                            ProjectCommands::Add(p) => projects::add::add(p, &gw).await?,
                            ProjectCommands::Delete(p) => projects::delete::delete(p, &gw).await?,
                            ProjectCommands::Sections(s) => match s {
                                SectionCommands::List(p) => sections::list::list(p, &gw).await?,
                                SectionCommands::Add(p) => sections::add::add(p, &gw).await?,
                                SectionCommands::Delete(p) => {
                                    sections::delete::delete(p, &gw).await?
                                }
                            },
                        },
                        AuthCommands::Labels(p) => match p {
                            LabelCommands::List(p) => labels::list::list(p, &gw).await?,
                            LabelCommands::Add(p) => labels::add::add(p, &gw).await?,
                            LabelCommands::Delete(p) => labels::delete::delete(p, &gw).await?,
                        },
                    }
                }
            },
            None => {
                list::list(self.params, &cfg.gateway()?, &cfg).await?;
            }
        }
        Ok(())
    }
}
