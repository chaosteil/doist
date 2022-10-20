use crate::{
    config::Config,
    labels, projects, sections,
    tasks::{add, close, comment, create, edit, list, view},
};
use clap::{Args, Parser, Subcommand};
use color_eyre::Result;

/// Args are the main entry point struct of the CLI app.
#[derive(Parser, Debug)]
#[command(author, version, about)]
#[command(args_conflicts_with_subcommands = true)]
pub struct Arguments {
    #[command(subcommand)]
    command: Option<Commands>,
    #[command(flatten)]
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
    #[command(flatten)]
    Authenticated(AuthCommands),
}

#[derive(Subcommand, Debug)]
enum AuthCommands {
    /// Adds a task.
    #[command(visible_alias = "a")]
    Add(add::Params),
    /// Creates a task interactively.
    #[command(visible_alias = "A")]
    Create(create::Params),
    /// Lists tasks. This is the default if no subcommand is specified.
    #[command(visible_alias = "l")]
    List(list::Params),
    /// Edits a task.
    #[command(visible_alias = "e")]
    Edit(edit::Params),
    /// Closes a task.
    #[command(visible_alias = "c")]
    Close(close::Params),
    /// View details of a single task.
    #[command(visible_alias = "v")]
    View(view::Params),
    /// Add a comment on a task.
    #[command(visible_alias = "C")]
    Comment(comment::Params),

    /// Manages projects.
    #[command(visible_alias = "p")]
    Projects(ProjectArgs),
    /// Manages labels.
    #[command(visible_alias = "lbl")]
    Labels(LabelArgs),
}

#[derive(Args, Debug)]
#[command(args_conflicts_with_subcommands = true)]
struct ProjectArgs {
    #[command(subcommand)]
    command: Option<ProjectCommands>,
    #[command(flatten)]
    params: projects::list::Params,
}

#[derive(Subcommand, Debug)]
enum ProjectCommands {
    /// Lists all current projects. This is the default view.
    #[command(visible_alias = "l")]
    List(projects::list::Params),
    /// View details of a single project.
    #[command(visible_alias = "v")]
    View(projects::view::Params),
    /// Add a comment on a project.
    #[command(visible_alias = "C")]
    Comment(projects::comment::Params),
    /// Adds (creates) a new project.
    #[command(visible_alias = "a")]
    Add(projects::add::Params),
    /// Deletes a project
    #[command(visible_alias = "d")]
    Delete(projects::delete::Params),

    /// Manages sections.
    #[command(visible_alias = "s")]
    Sections(SectionArgs),
}

#[derive(Args, Debug)]
#[command(args_conflicts_with_subcommands = true)]
struct LabelArgs {
    #[command(subcommand)]
    command: Option<LabelCommands>,
    #[command(flatten)]
    params: labels::list::Params,
}

#[derive(Subcommand, Debug)]
enum LabelCommands {
    /// Lists all current labels.
    #[command(visible_alias = "l")]
    List(labels::list::Params),
    /// Adds (creates) a new label.
    #[command(visible_alias = "a")]
    Add(labels::add::Params),
    /// Deletes a label.
    #[command(visible_alias = "d")]
    Delete(labels::delete::Params),
}

#[derive(Args, Debug)]
#[command(args_conflicts_with_subcommands = true)]
struct SectionArgs {
    #[command(subcommand)]
    command: Option<SectionCommands>,
    #[command(flatten)]
    params: sections::list::Params,
}

#[derive(Subcommand, Debug)]
enum SectionCommands {
    /// Lists all current sections of the project.
    #[command(visible_alias = "l")]
    List(sections::list::Params),
    /// Adds (creates) a new section in a project.
    #[command(visible_alias = "a")]
    Add(sections::add::Params),
    /// Deletes a section in a project.
    #[command(visible_alias = "d")]
    Delete(sections::delete::Params),
}

impl Arguments {
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
                        AuthCommands::Create(p) => create::create(p, &gw, &cfg).await?,
                        AuthCommands::List(p) => list::list(p, &gw, &cfg).await?,
                        AuthCommands::Edit(p) => edit::edit(p, &gw, &cfg).await?,
                        AuthCommands::Close(p) => close::close(p, &gw, &cfg).await?,
                        AuthCommands::View(p) => view::view(p, &gw, &cfg).await?,
                        AuthCommands::Comment(p) => comment::comment(p, &gw, &cfg).await?,
                        AuthCommands::Projects(p) => match p.command {
                            Some(p) => match p {
                                ProjectCommands::List(p) => projects::list::list(p, &gw).await?,
                                ProjectCommands::View(p) => projects::view::view(p, &gw).await?,
                                ProjectCommands::Comment(p) => {
                                    projects::comment::comment(p, &gw).await?
                                }
                                ProjectCommands::Add(p) => projects::add::add(p, &gw).await?,
                                ProjectCommands::Delete(p) => {
                                    projects::delete::delete(p, &gw).await?
                                }
                                ProjectCommands::Sections(s) => match s.command {
                                    Some(s) => match s {
                                        SectionCommands::List(p) => {
                                            sections::list::list(p, &gw).await?
                                        }
                                        SectionCommands::Add(p) => {
                                            sections::add::add(p, &gw).await?
                                        }
                                        SectionCommands::Delete(p) => {
                                            sections::delete::delete(p, &gw).await?
                                        }
                                    },
                                    None => sections::list::list(s.params, &gw).await?,
                                },
                            },
                            None => projects::list::list(p.params, &gw).await?,
                        },
                        AuthCommands::Labels(p) => match p.command {
                            Some(p) => match p {
                                LabelCommands::List(p) => labels::list::list(p, &gw).await?,
                                LabelCommands::Add(p) => labels::add::add(p, &gw).await?,
                                LabelCommands::Delete(p) => labels::delete::delete(p, &gw).await?,
                            },
                            None => labels::list::list(p.params, &gw).await?,
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
#[cfg(test)]
mod test {
    use crate::Arguments;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Arguments::command().debug_assert()
    }
}
