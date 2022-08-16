use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

use crate::api::rest::{Label, LabelID, Project, ProjectID, Section, SectionID, Task, TaskID};
use color_eyre::{eyre::eyre, eyre::WrapErr, Result};

// TODO:
// Optional:
//   * Has ID? Check if valid, return Ok(Some(item))
//   * Has name? Run through fuzzer, return Ok(Some(item))
//   * Nothing? Return Ok(None)
// Must Choose:
//   * Go through optional, if Ok(None) ->
//   * Nothing? Spin up interactive mode, return Ok(item)

pub fn select<T: ToString>(prompt: &str, items: &[T]) -> Result<Option<usize>> {
    let result = dialoguer::FuzzySelect::with_theme(&dialoguer::theme::ColorfulTheme {
        fuzzy_match_highlight_style: dialoguer::console::Style::new()
            .for_stderr()
            .yellow()
            .bold(),
        active_item_style: dialoguer::console::Style::new().for_stderr(),
        ..Default::default()
    })
    .items(items)
    .with_prompt(prompt)
    .default(0)
    .interact_opt()
    .wrap_err("Unable to make a selection")?;
    Ok(result)
}

pub fn fuzz_select<'a, T: FuzzSelect>(items: &'a [T], input: &'_ str) -> Result<&'a T> {
    if items.is_empty() {
        return Err(eyre!("no items available for selection, aborting"));
    }
    let matcher = SkimMatcherV2::default();
    items
        .iter()
        .filter_map(|i| matcher.fuzzy_match(i.name(), input).map(|s| (s, i)))
        .max_by(|left, right| left.0.cmp(&right.0))
        .and_then(|v| Some(v.1))
        .ok_or_else(|| eyre!("no suitable item found, aborting"))
}

pub trait FuzzSelect {
    type ID;

    fn id(&self) -> Self::ID;
    fn name(&self) -> &str;
}

impl FuzzSelect for Project {
    type ID = ProjectID;

    fn id(&self) -> ProjectID {
        self.id
    }
    fn name(&self) -> &str {
        &self.name
    }
}

impl FuzzSelect for Section {
    type ID = SectionID;

    fn id(&self) -> SectionID {
        self.id
    }
    fn name(&self) -> &str {
        &self.name
    }
}

impl FuzzSelect for Label {
    type ID = LabelID;

    fn id(&self) -> LabelID {
        self.id
    }
    fn name(&self) -> &str {
        &self.name
    }
}

impl FuzzSelect for Task {
    type ID = TaskID;

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

    impl<'a> FuzzSelect for Selectable<'a> {
        type ID = i32;

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
        assert_eq!(fuzz_select(&select, "one").unwrap().0, 1);
        assert_eq!(fuzz_select(&select, "w").unwrap().0, 2);
        assert!(fuzz_select(&select, "what").is_err());
    }
}
