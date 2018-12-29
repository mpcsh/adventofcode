use std::env;
use std::fs;
use std::collections::HashSet;

type State = Vec<(char, i64)>;

fn state_to_string(slice: &[(char, i64)]) -> String {
    slice
        .iter()
        .map(|(c, _)| c)
        .collect()
}

// fn print_state(state: &State) {
//     println!("{}", state_to_string(&state[..]))
// }

fn should_sprout(slice: &[(char, i64)], rules: &HashSet<String>) -> bool {
    rules.contains(&state_to_string(slice))
}

fn find_cycle(init_state: &State, rules: &HashSet<String>) -> (State, u64, u64) {
    // find meeting point
    let &mut tortoise = &evolve(init_state, rules);
    let &mut hare = &evolve(&evolve(init_state, rules), rules);
    while tortoise != hare {
        tortoise = &evolve(tortoise, rules);
        hare = &evolve(&evolve(hare, rules), rules);
    };

    // find position of first repition
    let mut first_rep = 0;
    tortoise = &init_state;
    while tortoise != hare {
        tortoise = &evolve(tortoise, rules);
        hare = &evolve(hare, rules);
        first_rep += 1;
    };

    // find length of shortest cycle
    let mut cycle_length = 1;
    hare = &evolve(tortoise, rules);
    while tortoise != hare {
        hare = &evolve(hare, rules);
        cycle_length += 1;
    };

    (tortoise, first_rep, cycle_length)
}

fn evolve(state: &State, rules: &HashSet<String>) -> State {
    let mut new_state = Vec::new();
    let mut padded_state = state.clone();
    for _ in 0..4 {
        let front_idx = padded_state[0].1 - 1;
        padded_state.insert(0, ('.', front_idx));
        let back_idx = padded_state[padded_state.len() - 1].1 + 1;
        padded_state.push(('.', back_idx));
    };
    for i in 2..(padded_state.len() - 2) {
        if should_sprout(&padded_state[(i - 2)..(i + 3)], rules) {
            new_state.push(('#', padded_state[i].1));
        } else {
            new_state.push(('.', padded_state[i].1));
        };
    };

    new_state
}

fn pot_sum(state: &State) -> i64 {
    state.iter().fold(0, |sum, x| {
        if x.0 == '#' {
            sum + x.1
        } else {
            sum
        }
    })
}

fn part_1(state: &State, rules: &HashSet<String>) -> i64 {
    let mut final_state = state.clone();
    for _ in 0..20 {
        final_state = evolve(&final_state, rules);
    };
    pot_sum(&final_state)
}

fn part_2(state: State, rules: &HashSet<String>) -> i64 {
    let (cycle_start_state, cycle_start_generation, cycle_len) = find_cycle(&state, rules);

    // pot_sum(final_state)
    0
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

    let initial_state: State = lines[0]
        .split(": ")
        .skip(1)
        .take(1)
        .collect::<String>()
        .chars()
        .zip(0..)
        .collect();

    let rules = lines
        .iter()
        .skip(1)
        .map(|l| l
            .split(|c| c == '=' || c == '>' || c == ' ')
            .filter(|s| s != &"")
            .collect::<Vec<&str>>())
        .filter_map(|ts|
            if ts[1] == "#" {
                Some(ts[0].to_string())
            } else {
                None
            }
        )
        .collect::<HashSet<String>>();

    println!("Part 1: the sum of the pot numbers is {}",
             part_1(&initial_state, &rules));

    println!("Part 2: the sum of the pot numbers is {}",
             part_2(initial_state.clone(), &rules));

    Ok(())
}
