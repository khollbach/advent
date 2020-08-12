use std::cell::RefCell;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::rc::{Rc, Weak};

/// Derefs to the inner Rc. Exists only so we can implement Hash.
#[derive(Debug)]
pub struct Tree(pub Rc<RefCell<Node>>);

impl Tree {
    pub fn new(label: Rc<str>) -> Self {
        Self(Rc::new(RefCell::new(Node::new(label))))
    }

    pub fn clone(tree: &Tree) -> Self {
        Tree(Rc::clone(&tree))
    }
}

impl Deref for Tree {
    type Target = Rc<RefCell<Node>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Hash for Tree {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.borrow().label.hash(state);
    }
}

impl PartialEq for Tree {
    fn eq(&self, other: &Self) -> bool {
        self.borrow().label == other.borrow().label
    }
}

impl Eq for Tree {}

/// Node in a DAG. In our case, the DAG is a tree.
/// Graphs with cycles will cause a memory leak.
#[derive(Debug)]
pub struct Node {
    label: Rc<str>,
    pub parent: Weak<RefCell<Node>>,
    pub children: HashSet<Tree>,
}

impl Node {
    pub fn new(label: Rc<str>) -> Self {
        Self {
            label,
            parent: Weak::new(),
            children: HashSet::new(),
        }
    }

    pub fn parent(&self) -> Option<Tree> {
        self.parent.upgrade().map(|rc| Tree(Rc::clone(&rc)))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label
    }
}

impl Eq for Node {}
