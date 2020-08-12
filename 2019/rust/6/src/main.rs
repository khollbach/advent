use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{self, prelude::*};
use std::rc::{Rc, Weak};

fn main() {
    let tree = read_input();

    println!("{}", sum_of_depths(tree));
}

/// Compute the sum of the depths of each node in a tree.
fn sum_of_depths(root: Rc<RefCell<Node>>) -> u32 {
    let mut depth_sum = 0;

    let mut stack = vec![(root, 0)];
    while let Some((node, d)) = stack.pop() {
        depth_sum += d;

        for child in &node.borrow().children {
            stack.push((Rc::clone(child), d + 1));
        }
    }

    depth_sum
}

/// Node in a DAG. In our case, the DAG is a tree.
/// Graphs with cycles will cause a memory leak.
#[derive(Debug)]
struct Node {
    parent: Weak<RefCell<Node>>,
    children: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    fn new() -> Self {
        Self {
            parent: Weak::new(),
            children: vec![],
        }
    }
}

/// Read the input graph, and return the root node, "COM". If the input graph is disconnected and
/// there are any undirected cycles, their memory will be leaked.
///
/// The returned graph will be a tree (or else this will panic).
fn read_input() -> Rc<RefCell<Node>> {
    // Map from each label to its corresponding node.
    let mut nodes = HashMap::<String, Rc<RefCell<Node>>>::new();

    // Lookup a node, or create one if it doesn't exist.
    let mut get_node = |label: &str| {
        if !nodes.contains_key(label) {
            nodes.insert(label.to_string(), Rc::new(RefCell::new(Node::new())));
        }

        Rc::clone(&nodes[label])
    };

    for line in io::stdin().lock().lines().map(Result::unwrap) {
        let mut words = line.split(')');

        let parent = get_node(words.next().unwrap());
        let child = get_node(words.next().unwrap());

        // Insert the edge `(parent, child)` into the graph.
        parent.borrow_mut().children.push(Rc::clone(&child));
        assert!(child.borrow().parent.upgrade().is_none());
        child.borrow_mut().parent = Rc::downgrade(&parent);
    }

    let root = get_node("COM");
    assert!(root.borrow().parent.upgrade().is_none());
    root
}
