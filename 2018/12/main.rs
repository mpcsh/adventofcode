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

fn print_state(state: &State) {
    println!("{}", state_to_string(&state[..]))
}

fn cut_dots(mut state: State) -> State {
    let mut curr_plant = state[0].0;
    while curr_plant != '#' && state.len() > 1 {
        state.remove(0);
        curr_plant = state[0].0;
    };
    let mut length = state.len();
    curr_plant = state[length - 1].0;
    while curr_plant != '#' && length != 1 {
        length = state.len();
        state.pop();
        curr_plant = state[length - 2].0;
    };
    state
}

fn should_sprout(slice: &[(char, i64)], rules: &HashSet<String>) -> bool {
    rules.contains(&state_to_string(slice))
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

fn find_rep(mut state: State, rules: &HashSet<String>) -> (State, i64) {
    let mut next_state = evolve(&state, rules);
    let mut next_next_state = evolve(&evolve(&state, rules), rules);
    let mut first_difference = pot_sum(&next_state) - pot_sum(&state);
    let mut second_difference = pot_sum(&next_next_state) - pot_sum(&next_state);
    let mut prev_difference = 0;
    let mut generation = 0;

    while first_difference != second_difference || prev_difference != first_difference {
        prev_difference = first_difference;
        let next_gen = evolve(&next_next_state, rules);
        state = next_state;
        next_state = next_next_state;
        next_next_state = next_gen;
        first_difference = second_difference;
        second_difference = pot_sum(&next_next_state) - pot_sum(&next_state);
        generation += 1;
    };

    (state, generation)

}

fn part_2(state: State, rules: &HashSet<String>) -> i64 {
    let (first_cycle_state, generation) = find_rep(state.clone(), rules);
    let sum_at_cycle_start = pot_sum(&first_cycle_state);
    let next_sum = pot_sum(&evolve(&first_cycle_state, rules));
    let difference = next_sum - sum_at_cycle_start;
    let remaining_gen = 50000000000 - generation;
    sum_at_cycle_start + (difference * remaining_gen)
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
