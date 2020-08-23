use regex::Regex;
use std::collections::HashMap;
use std::io::prelude::*;
use std::ops::Deref;
use std::rc::Rc;

/// Graph of possible chemical reactions.
pub type Graph = HashMap<Rc<str>, Chemical>;

/// Represents a chemical reaction in the graph.
#[derive(Debug, Clone)]
pub struct Chemical {
    /// The amount of this chemical produced by the reaction.
    pub amount: u32,

    /// Required chemicals for the reaction producing this chemical.
    /// The weight of an edge is the amount of that chemical needed.
    pub deps: HashMap<Rc<str>, u32>,
}

/// Read a list of reactions into a graph.
///
/// There must be exactly one reaction that produces each chemical (except for "ORE").
pub fn read_graph<R: BufRead>(input: R) -> Graph {
    let mut graph = HashMap::new();

    let chem = r"(\d+) (\w+)";
    let line_re = Regex::new(&format!("^{}(, {})* => {}$", chem, chem, chem)).unwrap();
    let chem_re = Regex::new(chem).unwrap();

    for line in input.lines() {
        let line = line.unwrap();
        assert!(
            line_re.is_match(&line),
            "Line doesn't match expected format"
        );

        let mut chems = read_chemicals(&line, &chem_re);
        let (target, amount) = chems.pop().unwrap();
        assert!(!chems.is_empty());

        add_reaction(&mut graph, target, amount, &chems);
    }

    // Check all chemicals have a reaction producing them (except ore).
    assert!(graph
        .iter()
        .all(|(label, chem)| label.deref() == "ORE" || chem.amount != 0));

    graph
}

/// Helper for `read_graph`.
///
/// Read the chemicals and their amounts from a line of input.
fn read_chemicals<'a>(line: &'a str, chem_re: &Regex) -> Vec<(&'a str, u32)> {
    chem_re
        .captures_iter(line)
        .map(|caps| {
            let amount: u32 = caps[1].parse().unwrap();

            // This weirdness is because `caps` gets dropped, but we can borrow from `line`.
            let cap2 = caps.get(2).unwrap();
            let label = &line[cap2.start()..cap2.end()];

            (label, amount)
        })
        .collect()
}

/// Helper for `read_graph`.
///
/// Panics if there is already a reaction producing `target`.
///
/// Note: we use an amount of 0 as a placeholder for chemicals that don't yet have a reaction
/// producing them.
fn add_reaction(graph: &mut Graph, target: &str, amount: u32, deps: &[(&str, u32)]) {
    assert_ne!(amount, 0);

    // Insert edges.
    let mut deps_map = HashMap::new();
    for &(label, weight) in deps {
        assert_ne!(weight, 0);

        // Insert a dummy node with amount 0, to later check that all nodes appeared in a reaction.
        if !graph.contains_key(label) {
            graph.insert(
                Rc::from(label),
                Chemical {
                    amount: 0,
                    deps: HashMap::new(),
                },
            );
        }

        // Insert the edge. Re-use the existing label instead of allocating a new one.
        let (l, _) = graph.get_key_value(label).unwrap();
        deps_map.insert(Rc::clone(l), weight);
    }

    // Ensure a chemical doesn't appear twice as the result of a reaction.
    assert_eq!(graph.get(target).map(|chem| chem.amount).unwrap_or(0), 0);

    // Insert the target node. Re-use existing label, if any.
    let label = graph
        .get_key_value(target)
        .map(|(l, _)| Rc::clone(l))
        .unwrap_or_else(|| Rc::from(target));
    graph.insert(
        label,
        Chemical {
            amount,
            deps: deps_map,
        },
    );
}
