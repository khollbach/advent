use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::rc::{Rc, Weak};

pub type Tree = Rc<RefCell<Node>>;

/// Node in a DAG. In our case, the DAG is a tree.
/// Graphs with cycles will cause a memory leak.
#[derive(Debug)]
pub struct Node {
    label: Rc<str>,
    pub parent: Weak<RefCell<Node>>,
    pub children: BTreeSet<Tree>,
}

impl Node {
    pub fn new(label: Rc<str>) -> Tree {
        Rc::new(RefCell::new(Self {
            label,
            parent: Weak::new(),
            children: BTreeSet::new(),
        }))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.label.cmp(&other.label)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label
    }
}

impl Eq for Node {}
