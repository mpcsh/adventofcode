use std::env;



fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    let num_players = args[1].parse::<usize>().unwrap();
    let num_marbles = args[2].parse::<usize>().unwrap();

    println!("Part 1: highest score = {}", part_1(num_players, num_marbles));

    Ok(())
}
