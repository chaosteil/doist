use clap::{Arg, ArgAction, Args, FromArgMatches};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

use crate::api::rest::{Label, LabelID, Project, ProjectID, Section, SectionID, Task, TaskID};
use color_eyre::{eyre::eyre, eyre::WrapErr, Result};

#[derive(Debug)]
pub struct Selection<T: FuzzSelect> {
    name: Option<String>,
    id: Option<T::ID>,
}

macro_rules! selection {
    ($select_type:ty, $select_name:literal, $long:literal, $short:literal, $select_id:literal, $select_help:literal, $select_id_help:literal) => {
        impl FromArgMatches for Selection<$select_type> {
            fn from_arg_matches(matches: &clap::ArgMatches) -> Result<Self, clap::Error> {
                let id = match matches.get_one::<String>($select_id).map(|i| i.to_owned()) {
                    Some(id) => Some(id.parse().map_err(|_| {
                        clap::Error::raw(clap::ErrorKind::ValueValidation, "must be valid ID")
                    })?),
                    None => None,
                };
                Ok(Self {
                    name: matches
                        .get_one::<String>($select_name)
                        .map(|i| i.to_owned()),
                    id,
                })
            }

            fn update_from_arg_matches(
                &mut self,
                matches: &clap::ArgMatches,
            ) -> Result<(), clap::Error> {
                if let Some(name) = matches
                    .get_one::<String>($select_name)
                    .map(|i| i.to_owned())
                {
                    self.name = Some(name)
                }
                if let Some(id) = matches.get_one::<String>($select_id).map(|i| i.to_owned()) {
                    self.id = Some(id.parse().map_err(|_| {
                        clap::Error::raw(clap::ErrorKind::ValueValidation, "must be valid ID")
                    })?)
                }
                Ok(())
            }
        }

        impl Args for Selection<$select_type> {
            fn augment_args(cmd: clap::Command<'_>) -> clap::Command<'_> {
                cmd.arg(
                    Arg::new($select_name)
                        .short($short)
                        .long($long)
                        .help($select_help)
                        .action(ArgAction::Set),
                )
                .arg(
                    Arg::new($select_id)
                        .long($select_id)
                        .help($select_id_help)
                        .action(ArgAction::Set),
                )
            }

            fn augment_args_for_update(cmd: clap::Command<'_>) -> clap::Command<'_> {
                Self::augment_args(cmd)
            }
        }
    };
}

selection!(
    Project,
    "project",
    "project",
    'P',
    "project_id",
    "Uses the project name with the closest name, if possible. Does fuzzy matching for the name.",
    "ID of the project to use. Does nothing if -P is specified."
);

// TODO: filter down selection based on selected project if any
selection!(
    Section,
    "section",
    "section",
    'S',
    "section_id",
    "Uses the section name with the closest name, if possible. Does fuzzy matching for the name.",
    "ID of the section to use. Does nothing if -S is specified."
);

impl<T: FuzzSelect + std::fmt::Display> Selection<T> {
    pub fn optional<'a>(&self, items: &'a [T]) -> Result<Option<&'a T>> {
        let name = match &self.name {
            Some(name) => name,
            None => {
                return Ok(self
                    .id
                    .as_ref()
                    .and_then(|id| items.iter().find(|item| item.id() == *id)))
            }
        };
        Ok(Some(fuzz_select(items, name)?))
    }
    pub fn mandatory<'a>(&self, items: &'a [T]) -> Result<&'a T> {
        let selection = Self::optional(self, items)?;
        match selection {
            Some(s) => Ok(s),
            None => Ok(select("select item", items)?
                .map(|i| &items[i])
                .ok_or_else(|| eyre!("no selection made"))?),
        }
    }
}

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
        .map(|v| v.1)
        .ok_or_else(|| eyre!("no suitable item found, aborting"))
}

pub trait FuzzSelect {
    type ID: std::cmp::PartialEq + std::clone::Clone;

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
