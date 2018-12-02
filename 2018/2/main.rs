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

fn strcmp(s1: &String, s2: &String) -> u64 {
    let mut diff = 0;
    for (c1, c2) in s1.chars().zip(s2.chars()) {
        if c1 != c2 {
            diff += 1;
        };
    };

    diff
}

fn common_letters(s1: &String, s2: &String) -> String {
    let mut i = 0;
    for (c1, c2) in s1.chars().zip(s2.chars()) {
        if c1 != c2 {
            return [&s1[..i], &s2[i + 1..]].join("");
        };
        i += 1
    };

    s1.clone()
}

fn part_2(tags: &Vec<String>) -> String {
    for tag1 in tags {
        for tag2 in tags {
            if strcmp(tag1, tag2) == 1 {
                return common_letters(tag1, tag2);
            };
        };
    };

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
    println!("Part 2: common tag letters = {}", part_2(&tags));

    Ok(())
}
