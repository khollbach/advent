use by_address::ByAddress;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::prelude::*;
use std::rc::{Rc, Weak};

pub type Tree = Rc<RefCell<Node>>;

/// Node in a tree.
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

/// Read the input into a map from labels to nodes.
///
/// The returned graph is guaranteed to be a tree.
pub fn read_tree<R: BufRead>(input: R) -> HashMap<String, Tree> {
    let mut nodes = HashMap::new();

    // Lookup a node, or create one if it doesn't exist.
    let mut get_node = |label: &str| {
        if !nodes.contains_key(label) {
            nodes.insert(String::from(label), Node::new());
        }

        Rc::clone(&nodes[label])
    };

    for line in input.lines() {
        let line = line.unwrap();
        let mut words = line.split(')');

        let parent = get_node(words.next().unwrap());
        let child = get_node(words.next().unwrap());
        assert!(words.next().is_none());

        // Each child should have only one parent.
        assert!(child.borrow().parent.upgrade().is_none());

        // Insert the edge `(parent, child)` into the graph.
        parent
            .borrow_mut()
            .children
            .insert(ByAddress(Rc::clone(&child)));
        child.borrow_mut().parent = Rc::downgrade(&parent);
    }

    assert!(verify_tree(&nodes));

    nodes
}

/// Ensure that the collection of nodes is a tree.
///
/// It must be empty or have exactly one root, and everything should be reachable from the root.
#[must_use]
fn verify_tree(nodes: &HashMap<String, Tree>) -> bool {
    if nodes.is_empty() {
        return true;
    }

    // Get the root.
    let mut roots = nodes
        .values()
        .filter(|t| t.borrow().parent.upgrade().is_none());
    let root = match roots.next() {
        Some(r) => r,
        // No root; the graph is a collection of cycles.
        None => return false,
    };
    if roots.next().is_some() {
        // Multiple roots; the graph is disjoint.
        return false;
    }

    // All nodes should be reachable from the root
    num_nodes(Rc::clone(root)) == Some(nodes.len())
}

/// How many nodes reachable from this one?
///
/// Return None if there are cycles.
fn num_nodes(root: Tree) -> Option<usize> {
    // Keep track of which nodes have been seen, to detect cycles.
    fn helper(root: Tree, seen: &mut HashSet<ByAddress<Tree>>) -> Option<usize> {
        // Detect cycles.
        if seen.contains(&ByAddress(Rc::clone(&root))) {
            return None;
        }

        let mut n = 1;

        for ByAddress(t) in &root.borrow().children {
            n += helper(Rc::clone(t), seen)?;
        }

        Some(n)
    }

    let mut seen = HashSet::new();
    helper(root, &mut seen)
}
