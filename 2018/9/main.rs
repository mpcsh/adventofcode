use std::env;
use std::collections::VecDeque;

struct Circle<T> {
    current: Option<T>,
    left: VecDeque<T>,
    right: VecDeque<T>
}

impl<T: std::fmt::Debug> Circle<T> {
    fn new() -> Self {
        Circle {
            current: None,
            left: VecDeque::new(),
            right: VecDeque::new()
        }
    }

    fn insert(self: &mut Circle<T>, elem: T) {
        let _ = self.current
            .take()
            .map(|e| self.right.push_front(e));
        self.current = Some(elem);
    }

    fn rotate_clockwise(&mut self) {
        // push onto left
        self.current.take().map(|e| self.left.push_back(e));

        // rotate the lists
        self.left.pop_front().map(|e| self.right.push_back(e));

        // take from right
        self.right.pop_front().map(|e| self.current = Some(e));
    }

    fn rotate_counterclockwise(&mut self) {
        // push onto right
        self.current.take().map(|e| self.right.push_front(e));

        // rotate the lists
        self.right.pop_back().map(|e| self.left.push_front(e));

        // take from left
        self.left.pop_back().map(|e| self.current = Some(e));
    }

    // positive num_rot goes clockwise, negative num_rot goes counterclockwise
    fn rotate(self: &mut Circle<T>, num_rot: i64) {
            if num_rot >= 0 {
                for _ in 0..num_rot.abs() {
                    self.rotate_clockwise();
                };
            } else {
                for _ in 0..num_rot.abs() {
                    self.rotate_counterclockwise();
                };
            };
    }


    fn print(&mut self) -> () {
        for elem in self.left.iter() {
            print!("{:?} ", elem);
        };

        match &self.current {
            Some(elem) => print!("({:?}) ", elem),
            None => print!("")
        };

        for elem in self.right.iter() {
            print!("{:?} ", elem);

        };
        print!("\n");
    }
}

fn part_1(num_players: usize, num_marbles: usize) -> u64 {

    0
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    let num_players = args[1].parse::<usize>().unwrap();
    let num_marbles = args[2].parse::<usize>().unwrap();

    let mut game = Circle::new();
    game.insert(1);
    game.insert(2);
    game.insert(3);
    game.insert(4);
    game.print();
    game.rotate(1);
    game.print();
    game.rotate(3);
    game.print();
    game.rotate(-8);
    println!("Part 1: highest score = {}", part_1(num_players, num_marbles));

    Ok(())
}
