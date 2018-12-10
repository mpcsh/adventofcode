use std::fs;

fn flip_case(c: &char) -> char {
    ((*c as u8) ^ 0x20) as char
}

fn part_1(polymer: &mut Vec<char>) -> u64 {
    let mut i = 0;
    loop {
        if i == polymer.len() - 1 {
            return polymer.len() as u64;
        };

        if polymer[i] == flip_case(&polymer[i + 1]) {
            // removes the chars that start at i and i + 1
            polymer.remove(i);
            polymer.remove(i);
            if i != 0 {
                i -= 1;
            };
        } else {
            i += 1;
        };
    };
}

fn main() -> Result<(), std::io::Error> {
    let polymer: String = fs::read_to_string("input.txt")?.trim().to_string();

    println!("Part 1: reacted length = {}",
             part_1(&mut polymer.chars().collect()));

    Ok(())
}
