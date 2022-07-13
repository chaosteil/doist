use crate::api::rest::Priority as RESTPriority;
use color_eyre::eyre::eyre;
use serde::{Deserialize, Serialize};

#[derive(clap::ArgEnum, Debug, Copy, Clone, Deserialize, Serialize)]
pub enum Priority {
    #[clap(name = "1")]
    Urgent,
    #[clap(name = "2")]
    VeryHigh,
    #[clap(name = "3")]
    High,
    #[clap(name = "4")]
    Normal,
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
        // API has urgent as p4, but UI uses p1 as top priority.
        match value {
            1 => Ok(Priority::Urgent),
            2 => Ok(Priority::VeryHigh),
            3 => Ok(Priority::High),
            4 => Ok(Priority::Normal),
            _ => Err(eyre!("invalid value for priority")),
        }
    }
}
