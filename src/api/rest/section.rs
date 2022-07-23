use serde::{Deserialize, Serialize};

use super::ProjectID;

pub type SectionID = usize;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Section {
    pub id: SectionID,
    pub project_id: ProjectID,
    pub order: isize,
    pub name: String,
}

impl Ord for Section {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.order.cmp(&other.order) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for Section {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::fmt::Display for Section {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
