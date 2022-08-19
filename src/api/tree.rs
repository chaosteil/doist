//! Maps Todoist API elements to the Tree that they represent.
//!
//! Todoist API items have their own ID and sometimes a parent ID. Using this information we can
//! construct a tree of items and their subitems. This is just a dirty implementation to get a tree
//! data structure out of that.
use color_eyre::{eyre::eyre, Result};
use std::{
    cell::RefCell,
    collections::{hash_map::Entry, HashMap, HashSet, VecDeque},
    ops::Deref,
    rc::Rc,
};

/// Treeable allows to make trees out of an ID and parent IDs.
pub trait Treeable: std::fmt::Debug + std::cmp::Ord {
    /// This is the ID type that will be used to generate the tree.
    type ID: std::cmp::Eq + std::hash::Hash;

    /// The ID of the current item.
    fn id(&self) -> Self::ID;
    /// The optional parent ID of the current item.
    fn parent_id(&self) -> Option<Self::ID>;
    /// To help finish trees that are perhaps not complete, reset_parent is called on items that
    /// could not find a parent.
    fn reset_parent(&mut self);
}

/// Tree is a representation of Items as a tree.
#[derive(Debug, PartialEq, Eq)]
pub struct Tree<T: Treeable> {
    /// The item of this Tree leaf.
    pub item: T,
    /// Additional leaves under this item.
    pub subitems: Vec<Tree<T>>,
    /// How deep we are in this tree, useful for representation.
    pub depth: usize,
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
    fn finalize(self, depth: usize) -> Tree<T> {
        let subitems: Vec<Tree<T>> = self
            .subitems
            .into_iter()
            .map(|c| {
                Rc::try_unwrap(c)
                    .expect("should consume single Rc")
                    .into_inner()
                    .finalize(depth + 1)
            })
            .collect();
        Tree {
            item: self.item,
            subitems,
            depth,
        }
    }
}

impl<T: Treeable + std::cmp::Eq> Tree<T> {
    /// Creates a new Tree leaf from the given item.
    pub fn new(item: T) -> Self {
        Self {
            item,
            subitems: vec![],
            depth: 0,
        }
    }
    /// Synthesizes a Tree out of a list of [`Treeable`] items.
    ///
    /// The main caveat here is that each item in the list of items must:
    /// 1. Have a unique ID
    /// 2. Not contain circular references and
    /// 3. If a parent ID exists, the actual parent must also exist.
    ///
    /// There is a case where a filtered todoist API will return only the subtasks and not
    /// its parents. Currently solved it by resetting parents of tasks that are not in the initial vector.
    ///
    /// The output from a whole tree can be used with the [`Tree::keep_trees`] method to get a clean tree.
    pub fn from_items(items: Vec<T>) -> Result<Vec<Tree<T>>> {
        let ids = items.iter().map(|t| t.id()).collect::<HashSet<_>>();
        // Split into things without parents and things with parents
        let (top_level_items, mut subitems): (VecDeque<_>, VecDeque<_>) = items
            .into_iter()
            .map(|mut item| {
                if let Some(parent) = item.parent_id() {
                    if !ids.contains(&parent) {
                        item.reset_parent();
                    }
                }
                Rc::new(RefCell::new(TreeBuilder {
                    item,
                    parent: None,
                    subitems: vec![],
                }))
            })
            .partition(|item| item.borrow().item.parent_id().is_none());

        // Create tree builder out of parents, this is where we'll slowly attach new items to
        let mut items: HashMap<_, Rc<RefCell<TreeBuilder<T>>>> = top_level_items
            .into_iter()
            .map(|item| (item.borrow().item.id(), item.clone()))
            .collect();

        while !subitems.is_empty() {
            let subitem = subitems.pop_front().unwrap();
            let parent = items.entry(
                subitem
                    .borrow()
                    .item
                    .parent_id()
                    .ok_or_else(|| eyre!("Subitem has bad parent assigned"))?,
            );
            if let Entry::Vacant(_) = parent {
                subitems.push_back(subitem);
                continue;
            }
            // Get the parent, and add our entry
            parent.and_modify(|entry| {
                subitem.borrow_mut().parent = Some(());
                entry.borrow_mut().subitems.push(subitem.clone())
            });
            // This item is now something that can be assigned as a parent
            items.insert(subitem.borrow().item.id(), subitem.clone());
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
                    .finalize(0))
            })
            .collect();
        items.map(|mut f| {
            f.sort();
            f
        })
    }

    /// Converts a Tree to a Vector of all items and their subitems (and so on) for easier handling.
    pub fn flatten(&self) -> Vec<&Tree<T>> {
        let mut items = vec![self];
        for item in &self.subitems {
            items.extend(item.flatten())
        }
        items
    }

    /// Tries to find the item with the given ID in this tree.
    pub fn find(&self, id: &<T as Treeable>::ID) -> Option<&Tree<T>> {
        if self.item.id() == *id {
            return Some(self);
        }
        for item in &self.subitems {
            if let Some(tree) = item.find(id) {
                return Some(tree);
            }
        }
        None
    }

    /// Tries to find the item with the given ID in this tree, mutably.
    pub fn find_mut(&mut self, id: &<T as Treeable>::ID) -> Option<&mut Tree<T>> {
        if self.item.id() == *id {
            return Some(self);
        }
        for item in &mut self.subitems {
            if let Some(tree) = item.find_mut(id) {
                return Some(tree);
            }
        }
        None
    }
}

/// Extension Trait to provide some additional common functionality for vectors of [Tree]s.
pub trait TreeFlattenExt<T: Treeable> {
    /// Takes the whole tree of tasks and flattens it out to a single vector with each tree being
    /// its own indexable item. Useful for user selection lists.
    fn flat_tree(&self) -> Vec<&Tree<T>>;
    /// Finds a particular Tree item within the whole vector of Trees.
    fn find(&self, id: T::ID) -> Option<&Tree<T>>;
    /// Finds a particular Tree item within the whole vector of Trees, mutably.
    fn find_mut(&mut self, id: T::ID) -> Option<&mut Tree<T>>;
    /// Uses the filter to keep only a subset of tasks of the current tree, but respects to keep
    /// parents.
    fn keep_trees(self, filter_items: &Vec<T>) -> Result<Self>
    where
        Self: Sized;
}

impl<T: Treeable> TreeFlattenExt<T> for Vec<Tree<T>> {
    fn flat_tree(&self) -> Vec<&Tree<T>> {
        self.iter().flat_map(Tree::flatten).collect()
    }

    fn find(&self, id: T::ID) -> Option<&Tree<T>> {
        for item in self {
            if let Some(item) = item.find(&id) {
                return Some(item);
            }
        }
        None
    }

    fn find_mut(&mut self, id: T::ID) -> Option<&mut Tree<T>> {
        for item in self {
            if let Some(item) = item.find_mut(&id) {
                return Some(item);
            }
        }
        None
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
        assert_eq!(trees[0].depth, 0);
        assert_eq!(trees[0].subitems[0].item.id, 2);
        assert_eq!(trees[0].subitems[0].depth, 1);
        assert_eq!(trees[0].subitems[0].subitems[0].item.id, 3);
        assert_eq!(trees[0].subitems[0].subitems[0].depth, 2);
        assert_eq!(trees[0].subitems[0].subitems[0].subitems[0].item.id, 4);
        assert_eq!(trees[0].subitems[0].subitems[0].subitems[0].depth, 3);
    }

    #[test]
    fn task_tree_no_parent() {
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
        let trees = Tree::from_items(tasks).unwrap();
        assert_eq!(trees.len(), 1);
        assert_eq!(trees[0].item.parent_id, None);
        assert_eq!(trees[0].subitems[0].item.id, 3);
    }
}
