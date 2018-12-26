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

fn should_sprout(slice: &[(char, i64)], rules: &HashSet<String>) -> bool {
    rules.contains(&state_to_string(slice))
}

fn evolve(mut initial_state: State, rules: &HashSet<String>, num_generations: u64) -> State {
    if num_generations == 0 {
        return initial_state;
    };

    // pad either side with empty pots if necessary
    if state_to_string(&initial_state[0..5]).contains('#') {
        let starting_index = initial_state[0].1;
        for i in 0..5 {
            initial_state.insert(0, ('.', starting_index - i - 1));
        };
    };
    if state_to_string(&initial_state[(initial_state.len() - 5)..]).contains('#') {
        let ending_index = initial_state[initial_state.len() - 1].1;
        let mut pad: State = Vec::new();
        for i in 0..5 {
            pad.push(('.', ending_index + i + 1));
        };
        initial_state.append(&mut pad);
    };

    // start the evolution process!
    let mut new_state = Vec::new();
    for i in 2..(initial_state.len() - 2) {
        if should_sprout(&initial_state[(i - 2)..(i + 3)], rules) {
            new_state.push(('#', initial_state[i].1));
        } else {
            new_state.push(('.', initial_state[i].1));
        };
    };

    evolve(new_state, &rules, num_generations - 1)
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

    let final_state = evolve(initial_state, &rules, 20);
    let pot_sum = final_state.iter().fold(0, |sum, x| if x.0 == '#' {
                                                        sum + x.1
                                                    } else {
                                                        sum
                                                    });
    println!("part 1: the sum of the pot numbers is {}", pot_sum);
    print_state(&final_state);

    Ok(())
}
