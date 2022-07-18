use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::api::rest::TODOIST_API_URL;

/// Stores configuration used by the application.
#[derive(Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub token: Option<String>,
    #[serde(default = "default_url")]
    pub url: url::Url,
}

/// Returns the default URL to be used for calling the Todoist API.
fn default_url() -> url::Url {
    TODOIST_API_URL.clone()
}

///! Describes errors that occur when loading from configuration storage.
#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("failed to place config into its directory")]
    Location(#[from] xdg::BaseDirectoriesError),
    #[error("unable to work with config file {file}")]
    File {
        file: PathBuf,
        #[source]
        io: std::io::Error,
    },
    #[error("unable to save config file")]
    SaveFormat(#[from] toml::ser::Error),
}

/// Defines the configuration filename inside the config directory.
const CONFIG_FILE: &str = "config.toml";

/// The name of the directories where configuration is stored.
const XDG_PREFIX: &str = "doist";

impl Config {
    /// Returns the name of the directories that are used for the configuration.
    fn config_dir() -> Result<xdg::BaseDirectories, xdg::BaseDirectoriesError> {
        xdg::BaseDirectories::with_prefix(XDG_PREFIX)
    }

    /// Load configuration from storage, if it exists.
    ///
    /// Tries to load configuration from storage, but If configuration does not exist, it will
    /// initialize a default configuration.
    pub fn load() -> Result<Config, ConfigError> {
        let file = Self::config_dir()?.get_config_file(CONFIG_FILE);
        let data = match fs::read_to_string(&file) {
            Ok(d) => d,
            Err(io) => match io.kind() {
                std::io::ErrorKind::NotFound => "".to_string(),
                _ => return Err(ConfigError::File { file, io })?,
            },
        };
        let config = toml::from_str(&data).unwrap();
        Ok(config)
    }

    /// Saves the current configuration to storage.
    pub fn save(&self) -> Result<(), ConfigError> {
        let dir = Self::config_dir()?;
        let file = dir
            .place_config_file(CONFIG_FILE)
            .map_err(|io| ConfigError::File {
                file: dir.get_config_file(CONFIG_FILE),
                io,
            })?;
        let data = toml::to_string(self)?;
        fs::write(&file, &data).map_err(|io| ConfigError::File { file, io })?;
        Ok(())
    }
}
