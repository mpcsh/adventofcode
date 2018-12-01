use std::fs;
use std::collections::HashSet;
use std::collections::Vec;

fn part_1(deltas: &Vec<i32>) -> i32 {
    let mut frequency: i32 = 0;

    for delta in deltas {
        frequency += delta
    };

    frequency
}


fn part_2(deltas: &Vec<i32>) -> i32 {
    let mut frequency: i32 = 0;
    let mut seen_frequencies = HashSet::new();

    loop {
        for delta in deltas {
        }
    }
}


fn main() {
    let contents: String = match fs::read_to_string("input.txt") {
        Ok(buffer) => buffer,
        Err(_) => String::new()
    };

    let mut deltas: Vec<i32> = Vec::new();
    for line in contents.split("\n") {
        let delta: i32 = match line.parse() {
            Ok(i) => i,
            Err(_) => 0
        };
        deltas.push(delta);
    };

    println!("Part 1: frequency is {}", part_1(deltas));
    println!("Part 2: first frequency reached twice is {}", part_2(deltas));
}
