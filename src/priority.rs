use crate::api::rest::Priority as RESTPriority;
use color_eyre::eyre::eyre;
use serde::{Deserialize, Serialize};

#[derive(clap::ArgEnum, Debug, Copy, Clone, Deserialize, Serialize)]
pub enum Priority {
    #[clap(name = "1")]
    Normal,
    #[clap(name = "2")]
    High,
    #[clap(name = "3")]
    VeryHigh,
    #[clap(name = "4")]
    Urgent,
}

impl From<Priority> for RESTPriority {
    fn from(p: Priority) -> Self {
        match p {
            Priority::Normal => RESTPriority::Normal,
            Priority::High => RESTPriority::High,
            Priority::VeryHigh => RESTPriority::VeryHigh,
            Priority::Urgent => RESTPriority::Urgent,
        }
    }
}

impl TryFrom<usize> for Priority {
    type Error = color_eyre::eyre::Error;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Priority::Normal),
            2 => Ok(Priority::High),
            3 => Ok(Priority::VeryHigh),
            4 => Ok(Priority::Urgent),
            _ => Err(eyre!("invalid value for priority")),
        }
    }
}
