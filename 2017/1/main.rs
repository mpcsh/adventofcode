use std::fs;
use std::vec::Vec;

fn main() -> Result<(), Box<std::error::Error>> {
    let mut input = fs::read_to_string("input.txt")?;
    let _ = input.pop();

    let mut nums : Vec<u32> = Vec::new();
    for c in input.chars() {
        nums.push(c.to_digit(10).unwrap());
    };

    let mut sum = 0;

    for (i, n) in nums.iter().enumerate() {
        if *n == nums[(i + 1) % nums.len()] {
            sum += n;
        };
    };

    println!("Sum: {}", sum);

    Ok(())
}
