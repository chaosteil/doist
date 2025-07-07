//! Describes everything related to configuration of the binary.
use std::{
    fs,
    path::{Path, PathBuf},
};

use chrono::{DateTime, Utc};
use color_eyre::{Result, eyre::eyre};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::api::rest::{Gateway, TODOIST_API_URL};

/// Stores configuration used by the application.
#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    /// The auth token that will be used to work with the Todoist API.
    /// The API Token can be found in the [Todoist settings](https://todoist.com/app/settings/integrations).
    #[serde(default)]
    pub token: Option<String>,
    /// Sets the different filter when using the filter without any options. Uses the value of
    /// `DEFAULT_FILTER` if none specifed.
    #[serde(default = "default_filter")]
    pub default_filter: String,
    /// Can override the API URL used by all commands. Mostly used for testing, but go crazy!
    #[serde(default = "default_url")]
    pub url: Option<url::Url>,
    /// Override the current time for various display options in the CLI.
    #[serde(default)]
    pub override_time: Option<DateTime<Utc>>,

    /// Sets a particular config location prefix. Mostly used for testing.
    #[serde(skip)]
    pub prefix: Option<PathBuf>,
}

/// Returns the default URL to be used for calling the Todoist API.
fn default_url() -> Option<url::Url> {
    Some(TODOIST_API_URL.clone())
}

/// Default filter when no config override is done.
const DEFAULT_FILTER: &str = "(today | overdue)";

fn default_filter() -> String {
    DEFAULT_FILTER.to_string()
}

/// Describes errors that occur when loading from configuration storage.
#[derive(Error, Debug)]
pub enum ConfigError {
    /// For errors that get returned when reading the config file.
    #[error("unable to work with config file {file}")]
    File {
        /// The path of the file that experienced the error.
        file: PathBuf,
        /// The error that prevented from working with the config file.
        #[source]
        io: Option<std::io::Error>,
    },
    /// For errors that happen during saving of the config file.
    #[error("unable to save config file")]
    SaveFormat(#[from] toml::ser::Error),
}

/// Defines the configuration filename inside the config directory.
const CONFIG_FILE: &str = "config.toml";

/// The name of the directories where configuration is stored.
const XDG_PREFIX: &str = "doist";

impl Config {
    /// Returns the name of the directories that are used for the configuration.
    fn config_dir(prefix: Option<&Path>) -> Result<PathBuf, ConfigError> {
        xdg::BaseDirectories::with_prefix(prefix.and_then(|p| p.to_str()).unwrap_or(XDG_PREFIX))
            .get_config_home()
            .ok_or_else(|| ConfigError::File {
                file: PathBuf::from(XDG_PREFIX),
                io: None,
            })
    }

    /// Returns the name of the config file that is used for configuration.
    fn config_file(prefix: Option<&Path>) -> Result<PathBuf, ConfigError> {
        let mut path = Self::config_dir(prefix)?;
        path.push(CONFIG_FILE);
        Ok(path)
    }

    /// Load configuration from storage, if it exists.
    ///
    /// Tries to load configuration from storage, but If configuration does not exist, it will
    /// initialize a default configuration.
    pub fn load() -> Result<Config, ConfigError> {
        let file = Self::config_file(None)?;
        Self::load_from(&file)
    }

    /// Load configuration from storage specified in another place, if it exists.
    ///
    /// Tries to load configuration from storage, but If configuration does not exist, it will
    /// initialize a default configuration.
    pub fn load_prefix(path: &Path) -> Result<Config, ConfigError> {
        let file = Self::config_file(Some(path))?;
        let mut cfg = Self::load_from(&file)?;
        cfg.prefix = Some(path.to_owned());
        Ok(cfg)
    }

    fn load_from(file: &PathBuf) -> Result<Config, ConfigError> {
        let data = match fs::read_to_string(file) {
            Ok(d) => d,
            Err(io) => match io.kind() {
                std::io::ErrorKind::NotFound => "".to_string(),
                _ => {
                    return Err(ConfigError::File {
                        file: file.clone(),
                        io: Some(io),
                    })?;
                }
            },
        };
        let config = toml::from_str(&data).unwrap();
        Ok(config)
    }

    /// Saves the current configuration to storage.
    pub fn save(&self) -> Result<(), ConfigError> {
        let file = Self::config_file(self.prefix.as_deref())?;
        file.parent()
            .map(fs::create_dir_all)
            .transpose()
            .map_err(|io| ConfigError::File {
                file: file.clone(),
                io: Some(io),
            })?;
        let data = toml::to_string(self)?;
        fs::write(&file, data).map_err(|io| ConfigError::File { file, io: Some(io) })?;
        Ok(())
    }

    /// Returns a fully initialized gateway if the config is valid, or otherwise informs about
    /// potential issues with the configuration.
    pub fn gateway(&self) -> Result<Gateway> {
        let token = self.token.as_deref().ok_or_else(|| {
            eyre!("No token in config specified. Use `doist auth` to register your token.")
        })?;
        Ok(Gateway::new(
            token,
            &self.url.clone().unwrap_or_else(|| default_url().unwrap()),
        ))
    }
}
