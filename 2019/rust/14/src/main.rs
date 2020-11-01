#![feature(const_int_pow)]

use graph::{read_graph, Graph};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::io;
use std::rc::Rc;

mod graph;

const TRILLION: u64 = 10u64.pow(12);

fn main() {
    let graph = read_graph(io::stdin().lock());

    println!("{}", ore_needed(&graph, 1));

    println!("{}", fuel_produced(&graph, TRILLION));
}

/// What is the maximum amount of fuel that can be produced by `ore_available` units of ore.
///
/// We use the `ore_needed` function and binary search on all different possible values of fuel,
/// until we find the maximum value of fuel such that `ore_needed(fuel) <= ore_available`.
fn fuel_produced(graph: &Graph, ore_available: u64) -> u64 {
    // I'm not sure what a better choice of upper bound is. Technically, depending on the graph,
    // the fuel produced could be arbitrarily much, possibly even more than the amount of ore
    // available.
    //
    // Also, if we make `upper` too large, we get overflows in `ore_needed`, so this is actually
    // quite fragile.
    let fuel_upper_bound = 2u64.pow(40);

    let target = ore_available;
    let key = |fuel| ore_needed(&graph, fuel);

    match binary_search(0, fuel_upper_bound, target, key) {
        Ok(fuel) => fuel,
        Err(fuel) => fuel - 1,
    }
}

/// Binary search in the range `lower..upper` to find `x` such that `key(x) == target`.
///
/// `key` must be a non-decreasing function on the range `lower..upper`.
///
/// If no such `x` is found, return an `Err` with the least upper bound. I.e., return some `y` such
/// that `key(y-1) < target < key(y)`.
fn binary_search<F>(lower: u64, upper: u64, target: u64, key: F) -> Result<u64, u64>
where
    F: Fn(u64) -> u64,
{
    if lower >= upper {
        return Err(upper);
    }

    let mid = (lower + upper) / 2;

    use Ordering::*;
    match target.cmp(&key(mid)) {
        Less => binary_search(lower, mid, target, key),
        Greater => binary_search(mid + 1, upper, target, key),
        Equal => Ok(mid),
    }
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
fn indegrees(graph: &Graph) -> HashMap<Rc<str>, usize> {
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

/// How much ore to produce this much fuel?
fn ore_needed(graph: &Graph, fuel_demand: u64) -> u64 {
    let n = graph.len();

    let idx_to_label = top_sort(&graph);
    let label_to_idx = reverse_mapping(&idx_to_label);

    let fuel = label_to_idx["FUEL"];
    let ore = label_to_idx["ORE"];

    let mut demands = vec![0; n];
    demands[fuel] = fuel_demand;

    for i in fuel..ore {
        let chem = &graph[&idx_to_label[i]];

        // How many times to perform this reaction.
        let num_reps = ceiling_divide(demands[i], chem.amount as u64);

        // Propagate demand across edges.
        for (edge, &weight) in &chem.deps {
            let j = label_to_idx[edge];
            debug_assert!(j > i);
            demands[j] += num_reps * weight as u64;
        }
    }

    // This checks for chemicals that are needed to make fuel, but don't reduce to ore.
    for &d in &demands[ore + 1..] {
        assert_eq!(d, 0);
    }

    demands[ore]
}

fn ceiling_divide(x: u64, y: u64) -> u64 {
    assert_ne!(y, 0);
    (x as f64 / y as f64).ceil() as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::input;

    #[test]
    fn part1() {
        let graph = read_graph(input!("../tests/input"));
        assert_eq!(143173, ore_needed(&graph, 1));
    }

    #[test]
    fn part2() {
        let graph = read_graph(input!("../tests/input"));
        assert_eq!(8845261, fuel_produced(&graph, TRILLION));
    }

    #[test]
    fn example1() {
        let graph = read_graph(input!("../tests/1"));
        assert_eq!(31, ore_needed(&graph, 1));
    }

    #[test]
    fn example2() {
        let graph = read_graph(input!("../tests/2"));
        assert_eq!(165, ore_needed(&graph, 1));
    }

    #[test]
    fn example3() {
        let graph = read_graph(input!("../tests/3"));
        assert_eq!(13312, ore_needed(&graph, 1));

        let fuel = 82892753;
        assert!(ore_needed(&graph, fuel) <= TRILLION);
        assert!(ore_needed(&graph, fuel + 1) > TRILLION);
        assert_eq!(fuel, fuel_produced(&graph, TRILLION));
    }

    #[test]
    fn example4() {
        let graph = read_graph(input!("../tests/4"));
        assert_eq!(180697, ore_needed(&graph, 1));

        let fuel = 5586022;
        assert!(ore_needed(&graph, fuel) <= TRILLION);
        assert!(ore_needed(&graph, fuel + 1) > TRILLION);
        assert_eq!(fuel, fuel_produced(&graph, TRILLION));
    }

    #[test]
    fn example5() {
        let graph = read_graph(input!("../tests/5"));
        assert_eq!(2210736, ore_needed(&graph, 1));

        let fuel = 460664;
        assert!(ore_needed(&graph, fuel) <= TRILLION);
        assert!(ore_needed(&graph, fuel + 1) > TRILLION);
        assert_eq!(fuel, fuel_produced(&graph, TRILLION));
    }
}
