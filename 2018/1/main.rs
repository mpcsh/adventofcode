use std::fs;

fn part_1(contents: String) -> i32 {
    let mut frequency: i32 = 0;

    for line in contents.split("\n") {
        let delta: i32 = match line.parse() {
            Ok(i) => i,
            Err(_) => 0
        };
        frequency += delta
    };

    frequency
}


fn main() {
    let contents: String = match fs::read_to_string("input.txt") {
        Ok(buffer) => buffer,
        Err(_) => String::new()
    };

    println!("Part 1: frequency is {}", part_1(contents));
}
