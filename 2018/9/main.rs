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

    println!("Part 1: highest score = {}", part_1(num_players, num_marbles));

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
