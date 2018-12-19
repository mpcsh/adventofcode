use std::env;
use std::fs;

fn part_1(num_players: u64, num_marbles: u64) -> u64{
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    let num_players = args[1].parse::<u64>().unwrap();
    let num_marbles = args[2].parse::<u64>().unwrap();

    println!("Part 1: highest score = {}", part_1(num_players, num_marbles));

    Ok(())
}
