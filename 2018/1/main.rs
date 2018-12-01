use std::fs;
use std::collections::HashSet;
use std::vec::Vec;

fn part_1(deltas: &Vec<i32>) -> i32 {
    let mut frequency: i32 = 0;

    for delta in deltas {
        frequency += delta
    };

    frequency
}


fn part_2(deltas: &Vec<i32>) -> i32 {
    let mut frequency: i32 = 0;
    let mut seen_frequencies: HashSet<i32> = HashSet::new();
    seen_frequencies.insert(0);

    loop {
        for delta in deltas {
            frequency += delta;
            if seen_frequencies.contains(&frequency) {
                return frequency;
            };
            seen_frequencies.insert(frequency);
        };
    }
}


fn main() {
    let contents: String = match fs::read_to_string("input.txt") {
        Ok(buffer) => buffer,
        Err(_) => String::new()
    };

    let mut deltas: Vec<i32> = Vec::new();
    for line in contents.split("\n") {
        let _ = match line.parse() {
            Ok(i) => deltas.push(i),
            Err(_) => ()
        };
    };

    println!("Part 1: frequency is {}", part_1(&deltas));
    println!("Part 2: first frequency reached twice is {}", part_2(&deltas));
}
