use std::fs;
use std::vec::Vec;

fn min_max_cksum(row : &Vec<u32>) -> u32 {
    let (mut min, mut max) : (u32, u32) = (std::u32::MAX, 0);
    for i in row {
        if *i < min {
            min = *i;
        }
        if *i > max {
            max = *i;
        }
    }
    max - min
}

fn div_cksum(row : &Vec<u32>) -> u32 {
    for tortoise in row {
        for hare in row {
            if tortoise != hare {
                if tortoise % hare == 0 {
                    return tortoise / hare;
                }
                else if hare % tortoise == 0 {
                    return hare / tortoise;
                }
            }
        }
    }
    panic!("Couldn't find evenly dividing elements!");
}

fn cksum<F>(spreadsheet : &Vec<Vec<u32>>, row_cksum : F) -> u32
        where F: Fn(&Vec<u32>) -> u32 {
    let mut cksum : u32 = 0;
    for row in spreadsheet {
        cksum += row_cksum(row);
    }
    cksum
}

fn main() -> Result<(), Box<std::error::Error>> {
    let mut input = fs::read_to_string("input.tsv")?;
    let _ = input.pop();

    let mut spreadsheet : Vec<Vec<u32>> = Vec::new();

    for line in input.split("\n") {
        let mut row : Vec<u32> = Vec::new();
        for cell in line.split("\t") {
            row.push(cell.parse::<u32>()?);
        }
        spreadsheet.push(row);
    }

    println!("Part 1: {}", cksum(&spreadsheet, min_max_cksum));
    println!("Part 2: {}", cksum(&spreadsheet, div_cksum));

    Ok(())
}
