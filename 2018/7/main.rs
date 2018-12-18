use std::env;
use std::fs;
use std::collections::HashMap;
use std::collections::BTreeSet;
use std::collections::BinaryHeap;

type Graph = HashMap<char, BTreeSet<char>>;

fn time(c: &char) -> i64 {
    ((*c as u8) - ('A' as u8) + 61) as i64
}

fn is_full<T: std::cmp::Ord>(heap: &BinaryHeap<T>) -> bool {
    heap.len() == heap.capacity()
}

fn process(graph: &mut Graph,
           incoming: &mut Graph,
           origins: &mut BTreeSet<char>,
           num_workers: i64) -> (Vec<char>, i64) {
    let mut ordering: Vec<char> = Vec::new();
    let mut workers: BinaryHeap<(i64, char)> = BinaryHeap::with_capacity(
        num_workers as usize);
    let mut current_time = 0;

    while !origins.is_empty() || !is_full(&workers) {
        // schedule as many as we can
        while !origins.is_empty() && !is_full(&workers) {
            let n: char = *origins.iter().next().unwrap();
            let _ = origins.remove(&n);
            workers.push((0 - (current_time + time(&n)), n.clone()));
        };

        // process until origins is not empty and queue is not full
        while !workers.is_empty() && (origins.is_empty() || is_full(&workers)) {
            let (completion_time, task) = workers.pop().unwrap();
            current_time = 0 - completion_time;
            ordering.push(task.clone());

            for &m in graph.get(&task).unwrap().iter() {
                let incoming_edges = incoming.get_mut(&m).unwrap();
                let _ = incoming_edges.remove(&task);

                if incoming_edges.is_empty() {
                    origins.insert(m);
                };
            };
        };

        // if we're done, we're done
        if workers.is_empty() && origins.is_empty() {
            break;
        };
    };

    // if graph has edges
    if incoming.iter().any(|(_, edges)| !edges.is_empty()) {
        panic!("Cycle detected!");
    };

    (ordering, current_time)
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

    let (ordering_1, _) = process(&mut graph.clone(),
                                  &mut incoming.clone(),
                                  &mut origins.clone(),
                                  1);
    println!("Part 1: ordering = {}", ordering_1.iter().collect::<String>());

    let (ordering_2, time_2) = process(&mut graph.clone(),
                                  &mut incoming.clone(),
                                  &mut origins.clone(),
                                  5);
    println!("Part 2: ordering = {}, time = {}",
             ordering_2.iter().collect::<String>(),
             time_2);

    Ok(())
}
