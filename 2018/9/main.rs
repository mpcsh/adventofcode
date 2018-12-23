use std::{env, fmt};
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

    fn insert(&mut self, elem: T) {
        let _ = self.current
            .take()
            .map(|e| self.right.push_front(e));
        self.current = Some(elem);
    }

    fn remove(&mut self) -> Option<T> {
        // take out the current value
        let ret = self.current.take();

        // take from right
        self.right.pop_front().map(|e| self.current = Some(e));

        ret
    }

    fn rotate_clockwise(&mut self) {
        // push onto left
        self.current.take().map(|e| self.left.push_back(e));

        // rotate the lists
        if self.left.len() > self.right.len() {
            self.left.pop_front().map(|e| self.right.push_back(e));
        }

        // take from right
        self.right.pop_front().map(|e| self.current = Some(e));
    }

    fn rotate_counterclockwise(&mut self) {
        // push onto right
        self.current.take().map(|e| self.right.push_front(e));

        // rotate the lists
        if self.right.len() > self.left.len() {
            self.right.pop_back().map(|e| self.left.push_front(e));
        };

        // take from left
        self.left.pop_back().map(|e| self.current = Some(e));
    }

    // positive num_rot goes clockwise, negative num_rot goes counterclockwise
    fn rotate(&mut self, num_rot: i64) {
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

    // fn size(&self) -> usize {
    //     self.left.len() + self.right.len() + 1
    // }
}

impl<T: fmt::Debug> fmt::Debug for Circle<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for elem in self.left.iter() {
            write!(f, "{:?} ", elem)?;
        };

        match &self.current {
            Some(elem) => write!(f, "({:?}) ", elem)?,
            None => write!(f, "")?
        };

        for elem in self.right.iter() {
            write!(f, "{:?} ", elem)?;
        };

        Ok(())
    }
}

fn play(num_players: usize, num_marbles: u64) -> u64 {
    let mut circle: Circle<u64> = Circle::new();
    let mut scores: Vec<u64> = vec![0; num_players];
    let mut remaining_marbles: Vec<u64> = (0..(num_marbles + 1)).rev().collect();
    let mut player = 0;
    while let Some(next) = remaining_marbles.pop() {
        if next == 0 || next % 23 != 0 {
            circle.rotate(2);
            circle.insert(next);
        }
        else {
            scores[player] += next;
            circle.rotate(-7);
            scores[player] += circle.remove().unwrap();
        };

        player += 1;
        player %= num_players;
    };

    scores.iter().max().cloned().unwrap()
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    let num_players = args[1].parse::<usize>().unwrap();
    let num_marbles = args[2].parse::<u64>().unwrap();

    println!("Part 1: highest score = {}", play(num_players, num_marbles));
    println!("Part 2: highest score = {}", play(num_players, num_marbles * 100));

    Ok(())
}

#[test]
fn creation() {
    let mut circle: Circle<String> = Circle::new();
    assert!(circle.current.is_none());
    assert!(circle.left.is_empty());
    assert!(circle.right.is_empty());
}

#[test]
fn insertion() {
    let mut circle: Circle<String> = Circle::new();

    // insert a single element
    circle.insert("Hello!".to_string());
    assert!(circle.current.is_some());
    assert_eq!(circle.current.as_ref().unwrap(), &"Hello!".to_string());
    assert!(circle.left.is_empty());
    assert!(circle.right.is_empty());

    // insert another and check that the current element gets pushed to the right
    circle.insert("World!".to_string());
    assert!(circle.current.is_some());
    assert_eq!(circle.current.as_ref().unwrap(), &"World!".to_string());
    assert!(circle.left.is_empty());
    assert!(!circle.right.is_empty());
    assert_eq!(circle.right.front().unwrap(), &"Hello!".to_string());
}

#[test]
fn rotation() {
    let mut circle: Circle<String> = Circle::new();

    // insert three elements
    circle.insert("world!".to_string());
    circle.insert("there,".to_string());
    circle.insert("Hi".to_string());

    // rotate zero places
    circle.rotate(0);
    assert_eq!(circle.current.as_ref().unwrap(), &"Hi".to_string());

    // rotate all the way clockwise
    circle.rotate(1);
    assert_eq!(circle.current.as_ref().unwrap(), &"there,".to_string());
    circle.rotate(1);
    assert_eq!(circle.current.as_ref().unwrap(), &"world!".to_string());
    circle.rotate(1);
    assert_eq!(circle.current.as_ref().unwrap(), &"Hi".to_string());

    // rotate all the way counterclockwise
    circle.rotate(-1);
    assert_eq!(circle.current.as_ref().unwrap(), &"world!".to_string());
    circle.rotate(-1);
    assert_eq!(circle.current.as_ref().unwrap(), &"there,".to_string());
    circle.rotate(-1);
    assert_eq!(circle.current.as_ref().unwrap(), &"Hi".to_string());

    // jump clockwise
    circle.rotate(7);
    assert_eq!(circle.current.as_ref().unwrap(), &"there,".to_string());

    // jump counterclockwise
    circle.rotate(-7);
    assert_eq!(circle.current.as_ref().unwrap(), &"Hi".to_string());
}
