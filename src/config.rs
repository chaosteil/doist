use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};
use thiserror::Error;
use url::Url;

#[derive(Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub token: Option<String>,
    #[serde(default = "default_url")]
    pub url: url::Url,
}

fn default_url() -> Url {
    Url::parse("https://api.todoist.com/").unwrap()
}

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

// Defines the configuration filename inside the config directory.
const CONFIG_FILE: &str = "config.toml";

impl Config {
    fn config_dir() -> Result<xdg::BaseDirectories, xdg::BaseDirectoriesError> {
        xdg::BaseDirectories::with_prefix("todoist")
    }

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
