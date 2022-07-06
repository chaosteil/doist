use crate::{add, api::rest::Gateway, close, config::Config, list};
use clap::{Parser, Subcommand};
use color_eyre::{eyre::eyre, Result};
use owo_colors::OwoColorize;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Args {
    #[clap(subcommand)]
    command: Commands,
}

///
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
    Add(add::Params),
    /// Lists tasks.
    List(list::Params),
    /// Closes a task.
    Close(close::Params),
}

impl Args {
    pub async fn exec(self) -> Result<()> {
        let mut cfg = Config::load()?;
        match self.command {
            Commands::Auth { token } => {
                println!("Given token was {}", token.green());
                cfg.token = Some(token);
                cfg.save()?;
            }
            Commands::Authenticated(command) => {
                let token = cfg.token.ok_or_else(|| {
                    eyre!(
                        "No token in config specified. Use `todoist auth` to register your token."
                    )
                })?;
                let gw = Gateway::new(&token, cfg.url);
                match command {
                    AuthCommands::Add(p) => add::add(p, &gw).await?,
                    AuthCommands::List(p) => list::list(p, &gw).await?,
                    AuthCommands::Close(p) => close::close(p, &gw).await?,
                }
            }
        }
        Ok(())
    }
}
