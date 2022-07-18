//! Maps Todoist API elements to the Tree that they represent.
//!
//! Todoist API items have their own ID and sometimes a parent ID. Using this information we can
//! construct a tree of items and their subitems. This is just a dirty implementation to get a tree
//! data structure out of that.
use color_eyre::{eyre::eyre, Result};
use std::{
    cell::RefCell,
    collections::{hash_map::Entry, HashMap, VecDeque},
    ops::Deref,
    rc::Rc,
};

/// Treeable allows to make trees out of an ID and parent IDs.
pub trait Treeable: std::fmt::Debug + std::cmp::Ord {
    fn id(&self) -> usize;
    fn parent_id(&self) -> Option<usize>;
}

/// Tree is a representation of Items as a tree.
#[derive(Debug, PartialEq, Eq)]
pub struct Tree<T: Treeable> {
    pub item: T,
    pub subitems: Vec<Tree<T>>,
}

impl<T: Treeable> Deref for Tree<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.item
    }
}

impl<T: Treeable> Ord for Tree<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.item.cmp(&other.item)
    }
}

impl<T: Treeable + std::cmp::PartialEq> PartialOrd for Tree<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.item.cmp(&other.item))
    }
}

/// TreeBuilder is a helper struct helping to create a [`Tree`].
#[derive(Debug)]
struct TreeBuilder<Treeable> {
    item: Treeable,
    parent: Option<()>,
    subitems: Vec<Rc<RefCell<TreeBuilder<Treeable>>>>,
}

impl<T: Treeable> TreeBuilder<T> {
    fn finalize(self) -> Tree<T> {
        let subitems: Vec<Tree<T>> = self
            .subitems
            .into_iter()
            .map(|c| {
                Rc::try_unwrap(c)
                    .expect("should consume single Rc")
                    .into_inner()
                    .finalize()
            })
            .collect();
        Tree {
            item: self.item,
            subitems,
        }
    }
}

impl<T: Treeable + std::cmp::Eq> Tree<T> {
    /// Synthesizes a Tree out of a list of [`Treeable`] items.
    ///
    /// The main caveat here is that each item in the list of items must:
    /// 1. Have a unique ID
    /// 2. Not contain circular references and
    /// 3. If a parent ID exists, the actual parent must also exist.
    pub fn from_items(items: Vec<T>) -> Result<Vec<Tree<T>>> {
        let (top_level_items, mut subitems): (VecDeque<_>, VecDeque<_>) = items
            .into_iter()
            .map(|item| {
                Rc::new(RefCell::new(TreeBuilder {
                    item,
                    parent: None,
                    subitems: vec![],
                }))
            })
            .partition(|item| item.borrow().item.parent_id().is_none());

        let mut items: HashMap<_, Rc<RefCell<TreeBuilder<T>>>> = top_level_items
            .into_iter()
            .map(|item| (item.borrow().item.id(), item.clone()))
            .collect();

        let mut fails = 0; // Tracks for infinite loop on subitems
        while !subitems.is_empty() && fails <= subitems.len() {
            let subitem = subitems.pop_front().unwrap();
            let parent = items.entry(
                subitem
                    .borrow()
                    .item
                    .parent_id()
                    .ok_or_else(|| eyre!("Subitem has bad parent assigned"))?,
            );
            if let Entry::Vacant(_) = parent {
                fails += 1;
                subitems.push_back(subitem);
                continue;
            }
            fails = 0;
            parent.and_modify(|entry| {
                subitem.borrow_mut().parent = Some(());
                entry.borrow_mut().subitems.push(subitem.clone())
            });
            items.insert(subitem.borrow().item.id(), subitem.clone());
        }

        if !subitems.is_empty() {
            return Err(eyre!("missing parent nodes in {} subitems", subitems.len()));
        }
        let items: Result<Vec<_>> = items
            .into_iter()
            .filter(|(_, c)| c.borrow().parent.is_none())
            .collect::<Vec<_>>()
            .into_iter()
            .map(|(_, c)| {
                Ok(Rc::try_unwrap(c)
                    .map_err(|_| eyre!("Expected single item reference"))?
                    .into_inner()
                    .finalize())
            })
            .collect();
        items.map(|mut f| {
            f.sort();
            f
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::api::rest::Task;

    use super::*;

    #[test]
    fn test_tree_no_subitems() {
        let tasks = vec![
            Task::new(1, "one"),
            Task::new(2, "two"),
            Task::new(3, "three"),
        ];
        let trees = Tree::from_items(tasks).unwrap();
        assert_eq!(trees.len(), 3);
    }

    #[test]
    fn test_tree_some_subtasks() {
        let tasks = vec![
            Task::new(1, "one"),
            Task::new(2, "two"),
            Task::new(3, "three"),
            Task {
                parent_id: Some(1),
                ..Task::new(4, "four")
            },
        ];
        let trees = Tree::from_items(tasks).unwrap();
        assert_eq!(trees.len(), 3);
        let task = trees.iter().filter(|t| t.item.id == 1).collect::<Vec<_>>();
        assert_eq!(task.len(), 1);
        let task = task[0];
        assert_eq!(task.subitems.len(), 1);
        assert_eq!(task.subitems[0].item.id, 4);
        for task in trees.into_iter().filter(|t| t.item.id != 1) {
            assert_eq!(task.subitems.len(), 0);
        }
    }

    #[test]
    fn task_tree_complex_subtasks() {
        let tasks = vec![
            Task::new(1, "one"),
            Task {
                parent_id: Some(1),
                ..Task::new(2, "two")
            },
            Task {
                parent_id: Some(2),
                ..Task::new(3, "three")
            },
            Task {
                parent_id: Some(3),
                ..Task::new(4, "four")
            },
        ];
        let trees = Tree::from_items(tasks).unwrap();
        assert_eq!(trees.len(), 1);
        assert_eq!(trees[0].item.id, 1);
        assert_eq!(trees[0].subitems[0].item.id, 2);
        assert_eq!(trees[0].subitems[0].subitems[0].item.id, 3);
        assert_eq!(trees[0].subitems[0].subitems[0].subitems[0].item.id, 4);
    }

    #[test]
    fn task_tree_bad_input() {
        let tasks = vec![
            Task {
                parent_id: Some(1),
                ..Task::new(2, "two")
            },
            Task {
                parent_id: Some(2),
                ..Task::new(3, "three")
            },
        ];
        assert!(Tree::from_items(tasks).is_err());
    }
}
