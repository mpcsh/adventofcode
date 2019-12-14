use std::env;
use std::fs;

use std::fmt;
use std::cmp::{PartialEq, Eq};
use std::hash::{Hash, Hasher};
use std::collections::{HashSet, HashMap};

use id_tree::{Tree, Node, NodeId};
use id_tree::InsertBehavior;
use log::debug;

#[derive(Clone)]
struct Chemical {
    qty: usize,
    el: String
}

impl Chemical {
    fn scale(&self, factor: usize) -> Self {
        Chemical {
            qty: self.qty * factor,
            el: self.el.to_string()
        }
    }

    fn from_str(parts: &str) -> Self {
        let mut toks = parts.split_whitespace();
        let qty = toks.next().expect("No quantity!").parse::<usize>().unwrap();
        let el = toks.next().expect("No element!").to_string();
        Chemical { qty, el }
    }
}

impl fmt::Debug for Chemical {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.qty, self.el)
    }
}

impl PartialEq for Chemical {
    fn eq(&self, other: &Self) -> bool {
        self.el == other.el
    }
}

impl Eq for Chemical { }

impl Hash for Chemical {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.el.hash(state);
    }
}

#[derive(Clone)]
struct Reaction {
    output: Chemical,
    inputs: Vec<Chemical>
}

impl Reaction {
    fn scale(&self, factor: usize) -> Self {
        Reaction {
            output: self.output.scale(factor),
            inputs: self.inputs
                .iter()
                .map(|chem| chem.scale(factor))
                .collect::<Vec<Chemical>>()
        }
    }

    fn from_str(parts: &str) -> Self {
        let mut toks = parts
            .split(|c| c == ',' || c == '=' || c == '>')
            .filter(|&s| s != "")
            .map(|s| s.trim())
            .collect::<Vec<_>>();
        let output = Chemical::from_str(toks.pop().expect("No output token!"));
        let inputs = toks.iter().map(|tok| Chemical::from_str(tok)).collect();

        Reaction { output, inputs }
    }

    fn from_output(output: Chemical) -> Self {
        Reaction {
            output,
            inputs: vec![]
        }
    }
}

impl fmt::Debug for Reaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} => {:?}", self.inputs, self.output)
    }
}

impl PartialEq for Reaction {
    fn eq(&self, other: &Self) -> bool {
        self.output.el == other.output.el
    }
}

impl Eq for Reaction { }

impl Hash for Reaction {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.output.el.hash(state);
    }
}

fn multiples_needed(orig_batch: usize, needed: usize) -> usize {
    let mut batch = orig_batch;
    let mut counter = 1;
    while batch < needed {
        batch += orig_batch;
        counter += 1;
    }
    
    counter
}

fn build_reaction_tree(reactions: &HashSet<Reaction>) -> Tree<Chemical> {
    let mut reaction_tree: Tree<Chemical> = Tree::new();

    let root_reaction = reactions.get(&Reaction::from_str("0 FUEL")).expect("No fuel producer!");
    let root_node_id = reaction_tree
        .insert(
            Node::new(root_reaction.output.clone()),
            InsertBehavior::AsRoot)
        .expect("Couldn't insert root!");
    let mut node_stack: Vec<(NodeId, Reaction, usize)> = vec![(root_node_id, root_reaction.clone(), root_reaction.output.qty)];

    while !node_stack.is_empty() {
        let (current_node_id, current_reaction, scaling_factor) = node_stack.pop().expect("Node stack was empty!");

        for input in current_reaction.inputs.iter() {
            let sub_node_id = reaction_tree
                .insert(
                    Node::new(input.scale(scaling_factor)),
                    InsertBehavior::UnderNode(&current_node_id))
                .expect("Couldn't insert ore!");

            if input.el != "ORE" {
                let sub_reaction = reactions
                    .get(&Reaction::from_output(input.clone()))
                    .expect("No input producer!")
                    .clone();

                let sub_scaling_factor = multiples_needed(sub_reaction.output.qty, input.qty * scaling_factor);

                node_stack.push((sub_node_id, sub_reaction, sub_scaling_factor));
            };
        };
    };

    let mut s = String::new();
    let _ = reaction_tree.write_formatted(&mut s).expect("Couldn't format tree!");
    debug!("{}", s);

    reaction_tree
}

fn compute_requirements(reaction_tree: &Tree<Chemical>) -> HashMap<Chemical, usize> {
	let mut requirements: HashMap<Chemical, usize> = HashMap::new();

	for node in reaction_tree.traverse_pre_order(&reaction_tree.root_node_id().expect("No reaction tree root!")).expect("Can't create traverser!") {
        if node.data().el == "ORE" {
            let requirement = reaction_tree.get(node.parent().expect("Orphan ore!")).expect("Couldn't get ore parent!").data();
            *requirements.entry(requirement.clone()).or_insert(0) += requirement.qty;
        };
    };

    debug!("{:?}", requirements);

	requirements
}

fn count_ore(reactions: &HashSet<Reaction>, requirements: &HashMap<Chemical, usize>) -> usize {
    let mut ore_count = 0;

    for (requirement, &needed) in requirements.iter() {
        let ore_reaction = reactions.get(&Reaction::from_output(requirement.clone())).expect("Couldn't find ore reaction!");
        let scaled_ore_reaction = ore_reaction.scale(multiples_needed(ore_reaction.output.qty, needed));
        debug!("{:?}", scaled_ore_reaction);
        ore_count += scaled_ore_reaction.inputs[0].qty;
    };

    ore_count
}


fn part1(reactions: &HashSet<Reaction>) -> () {
    let reaction_tree = build_reaction_tree(reactions);
    let requirements = compute_requirements(&reaction_tree);
    println!("Part 1: {}", count_ore(&reactions, &requirements));
}


fn main() -> Result<(), std::io::Error> {
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    let contents: String = fs::read_to_string(&args[1])?;

    let mut reactions: HashSet<Reaction> = HashSet::new();
    for line in contents.split("\n") {
        if line != "" {
            reactions.insert(Reaction::from_str(line));
        };
    };

    part1(&reactions);

    Ok(())
}
