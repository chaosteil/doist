use serde::{Deserialize, Serialize};

use crate::api::Color;

pub type LabelID = usize;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Label {
    pub id: LabelID,
    pub name: String,
    pub color: Color,
    pub order: isize,
    pub favorite: bool,
}

impl Ord for Label {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.order.cmp(&other.order) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for Label {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::fmt::Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "@{}", self.name)
    }
}
