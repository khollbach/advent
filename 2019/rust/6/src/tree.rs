use by_address::ByAddress;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::{Rc, Weak};

pub type Tree = Rc<RefCell<Node>>;

/// Node in a DAG. In our case, the DAG is a tree.
/// Graphs with cycles will cause a memory leak.
#[derive(Debug)]
pub struct Node {
    pub parent: Weak<RefCell<Node>>,
    pub children: HashSet<ByAddress<Tree>>,
}

impl Node {
    pub fn new() -> Tree {
        Rc::new(RefCell::new(Self {
            parent: Weak::new(),
            children: HashSet::new(),
        }))
    }
}

/// Allows us to compare Trees by address.
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        ByAddress(self) == ByAddress(other)
    }
}

impl Eq for Node {}
