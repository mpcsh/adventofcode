use std::fs;
use std::vec::Vec;
use std::collections::HashMap;

fn part_1(tags: &Vec<String>) -> u16 {
    let (mut exactly_two, mut exactly_three) = (0, 0);

    for tag in tags {
        let mut letter_counts: HashMap<char, u16> = HashMap::new();
        for letter in tag.chars() {
            let new_count = match letter_counts.get(&letter) {
                Some(count) => *count,
                None => 0
            };
            let _ = letter_counts.insert(letter, new_count + 1);
        };

        let (mut exactly_two_seen, mut exactly_three_seen) = (false, false);
        for letter_count in letter_counts.values() {
            exactly_two_seen |= *letter_count == 2;
            exactly_three_seen |= *letter_count == 3;
        }

        exactly_two += if exactly_two_seen {1} else {0};
        exactly_three += if exactly_three_seen {1} else {0};
    };

    exactly_two * exactly_three
}

fn part_2(tags: &Vec<String>) -> String {
    String::from("")
}

fn main() -> Result<(), std::io::Error> {
    let contents: String = fs::read_to_string("input.txt")?;

    let mut tags: Vec<String> = Vec::new();
    for tag in contents.split("\n") {
        if tag != "" {
            tags.push(tag.to_string());
        };
    };

    println!("Part 1: checksum = {}", part_1(&tags));

    Ok(())
}
