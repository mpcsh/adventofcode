use std::env;
use std::fs;


struct Node {
    num_children: u64,
    num_metadata: u64,
    children: Vec<Node>,
    metadata: Vec<u64>
}

impl Node {
    fn init(raw: &[u64]) -> Node {
        let num_children = raw[0] as u64;
        let num_metadata = raw[1] as u64;

        Node {
            num_children,
            num_metadata,
            children: vec![],
            metadata: vec![]
        }

    }

    fn from_raw_helper(raw: &[u64]) -> (Option<Node>, &[u64]) {
        if raw.len() < 2 {
            return (None, raw);
        }

        let mut node = Node::init(raw);
        let mut remaining: &[u64] = &raw[2..];

        for _ in 0..(node.num_children) {
            let (maybe_child, new_remaining) = Node::from_raw_helper(remaining);
            match maybe_child {
                None => (),
                Some(child) => node.children.push(child)
            };
            remaining = new_remaining;
        };

        for i in 0..(node.num_metadata as usize) {
            node.metadata.push(remaining[i]);
        };

        remaining = &remaining[(node.num_metadata as usize)..];

        (Some(node), remaining)
    }

    fn from_raw(raw: Vec<u64>) -> Option<Node> {
        let (node, _) = Node::from_raw_helper(&raw[..]);
        node
    }
}

fn part_1(tree: &Node) -> u64 {
    let mut count: u64 = tree.metadata
        .iter()
        .fold(0, |acc, x| acc + x);

    count += tree.children
        .iter()
        .fold(0, |acc, child| acc + part_1(&child));

    count
}

fn part_2(tree: &Node) -> u64 {
    if tree.num_children == 0 {
        tree.metadata
            .iter()
            .fold(0, |acc, x| acc + x)
    }
    else {
        tree.metadata
            .iter()
            .fold(0, |acc, &i| {
                if i > tree.num_children {
                    acc
                } else {
                    acc + part_2(&tree.children[i as usize - 1])
                }
            })
    }
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let contents: String = fs::read_to_string(&args[1])?;

    let raw: Vec<u64> = contents
        .split_whitespace()
        .map(|s| {
            match s.parse::<u64>() {
                Ok(i) => i,
                Err(_) => panic!("Couldn't parse file")
            }
        })
        .collect();

    let tree = Node::from_raw(raw).unwrap();

    println!("Part 1: sum of all metadata entries = {}", part_1(&tree));
    println!("Part 2: value of root node = {}", part_2(&tree));

    Ok(())
}
