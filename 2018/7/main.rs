use std::env;
use std::fs;
use std::collections::HashMap;
use std::collections::BTreeSet;

type Graph = HashMap<char, BTreeSet<char>>;

fn part_1(graph: &mut Graph, incoming: &mut Graph, origins: &mut BTreeSet<char>) -> Vec<char> {
    // L <- empty list that will contain the sorted elements
    let mut ordering: Vec<char> = Vec::new();

    while !origins.is_empty() {
        // remove a node n from S
        let n: char = *origins.iter().next().unwrap();
        let _ = origins.remove(&n);

        // add n to tail of L
        ordering.push(n.clone());

        // for each node m with an edge e from n to m
        for &m in graph.get(&n).unwrap().iter() {
            // remove edge e from the graph
            let incoming_edges = incoming.get_mut(&m).unwrap();
            let _ = incoming_edges.remove(&n);

            // if m has no other incoming edges
            if incoming_edges.is_empty() {
                origins.insert(m);
            };
        };
    };

    // if graph has edges
    if incoming.iter().any(|(_, edges)| !edges.is_empty()) {
        panic!("Cycle detected!");
    };

    ordering
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let contents: String = fs::read_to_string(&args[1])?;

    let mut lines: Vec<String> = Vec::new();
    for line in contents.split("\n") {
        if line != "" {
            lines.push(line.to_string());
        };
    };

    let mut graph: Graph = HashMap::new();
    let mut incoming: Graph = HashMap::new();
    let mut origins: BTreeSet<char> = BTreeSet::new();
    for line in lines {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        let source: char = tokens[1].chars().next().unwrap();
        let destination: char = tokens[7].chars().next().unwrap();

        // remove destination from S
        let _ = origins.remove(&destination);

        // add source to S
        if !graph.contains_key(&source) {
            let _ = origins.insert(source);
        };

        // insert edge into G
        let _ = graph
            .entry(source)
            .or_insert(BTreeSet::new())
            .insert(destination);

        // insert edge into incoming
        let _ = incoming
            .entry(destination)
            .or_insert(BTreeSet::new())
            .insert(source);

        // also insert the destination with an empty neighbors list so that the
        // guard around adding the source to S is correct
        let _ = graph
            .entry(destination)
            .or_insert(BTreeSet::new());
    };

    println!("Part 1: ordering = {}",
             part_1(&mut graph, &mut incoming, &mut origins)
                 .iter().collect::<String>());

    Ok(())
}
