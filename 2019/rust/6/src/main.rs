use by_address::ByAddress;
use std::io;
use std::rc::Rc;
use tree::{read_tree, Tree};

mod tree;

fn main() {
    let nodes = read_tree(io::stdin().lock());

    let root = Rc::clone(&nodes["COM"]);
    assert!(root.borrow().parent.upgrade().is_none());
    let you = Rc::clone(&nodes["YOU"]);
    let santa = Rc::clone(&nodes["SAN"]);

    println!("{}", sum_of_depths(root));
    println!("{}", distance(you, santa) - 2);
}

/// Compute the sum of the depths of each node in a tree.
fn sum_of_depths(root: Tree) -> u32 {
    let mut depth_sum = 0;

    let mut stack = vec![(root, 0)];
    while let Some((node, d)) = stack.pop() {
        depth_sum += d;

        for ByAddress(child) in &node.borrow().children {
            stack.push((Rc::clone(child), d + 1));
        }
    }

    depth_sum
}

/// Compute the distance between two nodes in a tree.
fn distance(node1: Tree, node2: Tree) -> u32 {
    let d1 = depth(Rc::clone(&node1));
    let d2 = depth(Rc::clone(&node2));

    // Sort so that node1's depth <= node2's depth.
    let (mut node1, d1, mut node2, mut d2) = if d1 <= d2 {
        (node1, d1, node2, d2)
    } else {
        (node2, d2, node1, d1)
    };

    let mut total_distance = 0;

    // Go up from node2 until they're at the same depth.
    while d1 < d2 {
        let tmp = node2.borrow().parent.upgrade().unwrap();
        node2 = tmp;

        total_distance += 1;
        d2 -= 1;
    }

    // Move both pointers up until they meet.
    while node1 != node2 {
        let tmp = node1.borrow().parent.upgrade().unwrap();
        node1 = tmp;

        let tmp = node2.borrow().parent.upgrade().unwrap();
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
        let parent = match node.borrow().parent.upgrade() {
            Some(p) => p,
            None => break,
        };
        node = parent;

        d += 1;
    }

    d
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::input;

    #[test]
    fn real() {
        let nodes = read_tree(input!("../tests/input"));

        let root = Rc::clone(&nodes["COM"]);
        assert!(root.borrow().parent.upgrade().is_none());
        let you = Rc::clone(&nodes["YOU"]);
        let santa = Rc::clone(&nodes["SAN"]);

        println!("{}", sum_of_depths(root));
        println!("{}", distance(you, santa) - 2);
    }
}
