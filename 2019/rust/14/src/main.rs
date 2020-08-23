use graph::{read_graph, Graph};
use std::collections::HashMap;
use std::io;
use std::rc::Rc;

mod graph;

fn main() {
    let graph = read_graph(io::stdin().lock());

    println!("{}", ore_needed(&graph));
}

/// Compute a topological ordering of the nodes in `graph.` Doesn't do anything special if there
/// are multiple connected components; the returned ordering is arbitrary among the valid ones.
///
/// Panics if there are cycles.
fn top_sort(graph: &Graph) -> Vec<Rc<str>> {
    let mut graph = graph.clone();
    let n = graph.len();

    let mut sorted = Vec::with_capacity(n);

    let mut indegrees = indegrees(&graph);

    // Nodes with indegree 0.
    let mut candidates: Vec<Rc<str>> = indegrees
        .iter()
        .filter(|&(_label, &deg)| deg == 0)
        .map(|(label, _deg)| Rc::clone(label))
        .collect();

    // Greedily delete nodes with indegree 0, until you can't.
    while let Some(label) = candidates.pop() {
        // Update indegrees and candidates.
        for other in graph[&label].deps.keys() {
            let deg = indegrees.get_mut(other).unwrap();
            *deg -= 1;
            if *deg == 0 {
                candidates.push(Rc::clone(other));
            }
        }

        // Delete the node; add it to the sorted list.
        graph.remove(&label);
        sorted.push(label);
    }

    // If the graph is a DAG, we'll have deleted all nodes.
    assert!(graph.is_empty());
    debug_assert_eq!(sorted.len(), n);

    sorted
}

/// Compute the indegrees of each node in `graph`.
fn indegrees(graph: &Graph) -> HashMap<Rc<str>, u32> {
    let mut degs: HashMap<_, _> = graph.keys().map(|label| (Rc::clone(label), 0)).collect();

    for node in graph.values() {
        for label in node.deps.keys() {
            *degs.get_mut(label).unwrap() += 1;
        }
    }

    degs
}

/// Compute the reverse mapping for the given ordering of nodes. That is, for each node, what is
/// it's position in the ordering?
fn reverse_mapping(idx_to_label: &[Rc<str>]) -> HashMap<Rc<str>, usize> {
    let mut label_to_idx = HashMap::new();

    for (i, label) in idx_to_label.iter().enumerate() {
        label_to_idx.insert(Rc::clone(label), i);
    }

    label_to_idx
}

/// How much ore to produce one fuel?
fn ore_needed(graph: &Graph) -> u32 {
    let n = graph.len();

    let idx_to_label = top_sort(&graph);
    let label_to_idx = reverse_mapping(&idx_to_label);

    let fuel = label_to_idx["FUEL"];
    let ore = label_to_idx["ORE"];

    let mut demands = vec![0; n];
    demands[fuel] = 1;

    for i in fuel..ore {
        let chem = &graph[&idx_to_label[i]];

        // How many times to perform this reaction.
        let num_reps = ceiling_divide(demands[i], chem.amount);

        // Propagate demand across edges.
        for (edge, weight) in &chem.deps {
            let j = label_to_idx[edge];
            debug_assert!(j > i);
            demands[j] += num_reps * weight;
        }
    }

    // This checks for chemicals that are needed to make fuel, but don't reduce to ore.
    for &d in &demands[ore + 1..] {
        assert_eq!(d, 0);
    }

    demands[ore]
}

fn ceiling_divide(x: u32, y: u32) -> u32 {
    assert_ne!(y, 0);
    (x as f64 / y as f64).ceil() as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::input;

    #[test]
    fn part1() {
        let graph = read_graph(input!("../tests/input"));
        assert_eq!(143173, ore_needed(&graph));
    }
}
