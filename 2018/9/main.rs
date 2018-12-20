use std::env;
use std::fs;


fn place_marble(circle: &mut Vec<u64>, curr_marble_index: usize, marble: u64) -> (u64, usize) {
    if marble % 23 != 0 {
        let new_index = (curr_marble_index + 2) % circle.len();
        circle.insert(new_index, marble);
        (0, new_index)
    } else {
        let new_index = (curr_marble_index - 7) % circle.len();
        let removed = circle.remove(new_index);
        (marble + removed, new_index + 1)
    }
}

fn part_1(num_players: usize, num_marbles: usize) -> usize {
    let mut scores = vec![0; num_players as usize];
    let mut circle = vec![0, 1];
    let mut curr_marble_index = 1;

    for i in 2..num_marbles {
        let (curr_player_score, new_index) = place_marble(
            &mut circle, curr_marble_index, i as u64);
        curr_marble_index = new_index;
        println!("{:?}", circle);
    }

    0
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    let num_players = args[1].parse::<usize>().unwrap();
    let num_marbles = args[2].parse::<usize>().unwrap();

    println!("Part 1: highest score = {}", part_1(num_players, num_marbles));

    Ok(())
}
