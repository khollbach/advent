use std::collections::HashMap;
use std::io::{self, prelude::*};
use std::rc::Rc;
use tree::Tree;

mod tree;

fn main() {
    let nodes = read_input();

    let root = Tree::clone(&nodes["COM"]);
    let you = Tree::clone(&nodes["YOU"]);
    let santa = Tree::clone(&nodes["SAN"]);

    // Part 1.
    println!("{}", sum_of_depths(root));

    // Part 2.
    println!("{}", distance(you, santa) - 2);
}

/// Compute the sum of the depths of each node in a tree.
fn sum_of_depths(root: Tree) -> u32 {
    let mut depth_sum = 0;

    let mut stack = vec![(root, 0)];
    while let Some((node, d)) = stack.pop() {
        depth_sum += d;

        for child in &node.borrow().children {
            stack.push((Tree::clone(child), d + 1));
        }
    }

    depth_sum
}

/// Compute the distance between two nodes in a tree.
fn distance(node1: Tree, node2: Tree) -> u32 {
    let d1 = depth(Tree::clone(&node1));
    let d2 = depth(Tree::clone(&node2));

    // Sort so that node1's depth <= node2's depth.
    let (mut node1, d1, mut node2, mut d2) = if d1 <= d2 {
        (node1, d1, node2, d2)
    } else {
        (node2, d2, node1, d1)
    };

    let mut total_distance = 0;

    // Go up from node2 until they're at the same depth.
    while d1 < d2 {
        let tmp = node2.borrow().parent().unwrap();
        node2 = tmp;

        total_distance += 1;
        d2 -= 1;
    }

    // Move both pointers up until they meet.
    while node1 != node2 {
        let tmp = node1.borrow().parent().unwrap();
        node1 = tmp;

        let tmp = node2.borrow().parent().unwrap();
        node2 = tmp;

        total_distance += 2;
    }

    total_distance
}

/// Depth of a node in a tree.
fn depth(mut node: Tree) -> u32 {
    let mut d = 0;

    #[allow(clippy::while_let_loop)]
    loop {
        let parent = match node.borrow().parent() {
            Some(p) => p,
            None => break,
        };
        node = parent;

        d += 1;
    }

    d
}

/// Read the input graph into a map from labels to nodes.
///
/// The returned graph is guaranteed to be a forest, plus any number of disjoint cycles.
///
/// If the graph is disconnected and there are cycles, their memory will be leaked.
fn read_input() -> HashMap<Rc<str>, Tree> {
    // Map from each label to its corresponding node.
    let mut nodes = HashMap::new();

    // Lookup a node, or create one if it doesn't exist.
    let mut get_node = |label: &str| {
        if !nodes.contains_key(label) {
            let rc = Rc::from(label);
            nodes.insert(Rc::clone(&rc), Tree::new(rc));
        }

        Tree::clone(&nodes[label])
    };

    for line in io::stdin().lock().lines().map(Result::unwrap) {
        let mut words = line.split(')');

        let parent = get_node(words.next().unwrap());
        let child = get_node(words.next().unwrap());

        assert!(child.borrow().parent().is_none());

        // Insert the edge `(parent, child)` into the graph.
        parent.borrow_mut().children.insert(Tree::clone(&child));
        child.borrow_mut().parent = Rc::downgrade(&parent);
    }

    nodes
}
