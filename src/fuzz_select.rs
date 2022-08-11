use color_eyre::{eyre::eyre, Result};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

use crate::api::rest::{Label, LabelID, Project, ProjectID, Section, SectionID, Task, TaskID};

pub fn fuzz_select<U, T: FuzzSelect<U>>(items: &[T], input: &str) -> Result<U> {
    if items.is_empty() {
        return Err(eyre!("no items available for selection, aborting"));
    }
    let matcher = SkimMatcherV2::default();
    let item = items
        .iter()
        .filter_map(|i| matcher.fuzzy_match(i.name(), input).map(|s| (s, i.id())))
        .max_by(|left, right| left.0.cmp(&right.0));
    match item {
        Some((_, id)) => Ok(id),
        None => Err(eyre!("no suitable item found, aborting")),
    }
}

pub trait FuzzSelect<T> {
    fn id(&self) -> T;
    fn name(&self) -> &str;
}

impl FuzzSelect<ProjectID> for Project {
    fn id(&self) -> ProjectID {
        self.id
    }
    fn name(&self) -> &str {
        &self.name
    }
}

impl FuzzSelect<SectionID> for Section {
    fn id(&self) -> SectionID {
        self.id
    }
    fn name(&self) -> &str {
        &self.name
    }
}

impl FuzzSelect<LabelID> for Label {
    fn id(&self) -> LabelID {
        self.id
    }
    fn name(&self) -> &str {
        &self.name
    }
}

impl FuzzSelect<TaskID> for Task {
    fn id(&self) -> TaskID {
        self.id
    }
    fn name(&self) -> &str {
        &self.content
    }
}

#[cfg(test)]
mod test {
    use super::*;

    type Selectable<'a> = (i32, &'a str);

    impl<'a> FuzzSelect<i32> for Selectable<'a> {
        fn id(&self) -> i32 {
            self.0
        }
        fn name(&self) -> &str {
            self.1
        }
    }

    #[test]
    fn select_best() {
        let select: Vec<Selectable> = vec![(0, "zero"), (1, "one"), (2, "two"), (3, "three")];
        assert_eq!(fuzz_select(&select, "one").unwrap(), 1);
        assert_eq!(fuzz_select(&select, "w").unwrap(), 2);
        assert!(fuzz_select(&select, "what").is_err());
    }
}
