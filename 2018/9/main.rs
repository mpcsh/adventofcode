#![feature(euclidean_division)]

use std::env;

fn place_marble(circle: &mut Vec<u64>, curr_marble_index: usize, marble: u64) -> (u64, usize) {
    let circ_len: i64 = circle.len() as i64;
    if marble % 23 != 0 {
        let mut new_index = (curr_marble_index + 2).mod_euc(circ_len as usize);
        if new_index == 0 {
            new_index = circle.len();
        };
        circle.insert(new_index, marble);
        (0, new_index)
    } else {
        let new_index = (curr_marble_index as i64 - 7).mod_euc(circ_len) as usize;
        let removed = circle.remove(new_index);
        (marble + removed, new_index)
    }
}

fn part_1(num_players: usize, num_marbles: usize) -> u64 {
    let mut scores = vec![0; num_players as usize];
    let mut circle = vec![0, 1];
    let mut curr_marble_index = 1;

    for i in 2..(num_marbles + 1) {
        let (curr_player_score, mut new_index) = place_marble(
            &mut circle, curr_marble_index, i as u64);
        let curr_player = curr_marble_index % num_players;
        scores[curr_player] += curr_player_score;
        if new_index == circle.len() {
            new_index = 0;
        };
        curr_marble_index = new_index;
        // println!("{:?}", circle);
    };

    let mut max = 0;
    for i in 0..num_players {
        if scores[i] > max {
            max = scores[i];
        };
    };

    max


}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    let num_players = args[1].parse::<usize>().unwrap();
    let num_marbles = args[2].parse::<usize>().unwrap();

    println!("Part 1: highest score = {}", part_1(num_players, num_marbles));

    Ok(())
}
