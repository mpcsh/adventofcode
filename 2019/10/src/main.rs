use std::env;
use std::fs;

use std::fmt;
use std::cmp::{PartialEq, Eq};
use std::hash::Hash;
use std::collections::HashMap;


#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Point {
    fn point_diff(&self, p2: &Point) -> Self {
        let p1 = self;
        let dx = p2.x - p1.x;
        let dy = p2.y - p1.y;

        Point { x: dx, y: dy }
    }

    fn normalize(&self) -> Self {
        if self.x == 0 && self.y == 0 { return *self; };
        let mut m = self.x;
        let mut n = self.y;
        while m != 0 {
            let old_m = m;
            m = n % m;
            n = old_m;
        };
        let gcd = n.abs();
        Point { x: self.x / gcd, y: self.y / gcd }
    }
}


type Space = Vec<Vec<bool>>;

fn print_space(space: &Space) -> () {
    space.iter().for_each(|row| {
            row.iter().for_each(|&point| {
                let pretty = if point { '#' } else { '.' };
                print!("{}", pretty);
            });
            println!("");
        });
}


fn group_by_angle(space: &Space, p1: &Point) -> HashMap<Point, Vec<Point>> {
    let mut grouped = HashMap::new();

    space
        .iter()
        .enumerate()
        .for_each(|(y, row)| row
            .iter()
            .enumerate()
            .filter(|(_, &loc)| loc)
            .for_each(|(x, _)| {
                let p2 = Point { x: x as i64, y: y as i64 };
                let diff = p1.point_diff(&p2).normalize();
                grouped.entry(diff).or_insert(Vec::new()).push(p2);
            })
        );

    grouped
}

fn count_visible(space: &Space, p1: &Point) -> usize {
    group_by_angle(space, p1).len() - 1
}


fn part1(space: &Space) -> () {
    let (max_point, max_count) = space
        .iter()
        .enumerate()
        .map(|(y, row)| row
             .iter()
             .enumerate()
             .filter(|(_, &loc)| loc)
             .map(move |(x, _)| {
                 let p1 = Point { x: x as i64, y: y as i64 };
                 (p1, count_visible(space, &p1))
             }))
        .flatten()
        .max_by(|(_, count1), (_, count2)| count1.cmp(&count2))
        .expect("No maximum point!");

    println!("Part 1: {:?} can see {} asteroids", max_point, max_count);
}


fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let contents: String = fs::read_to_string(&args[1])?;

    let mut space: Space = Vec::new();
    for line in contents.split("\n") {
        if line != "" {
            space.push(line.chars().map(|c| match c {
                '.' => false,
                '#' => true,
                _ => panic!("Unexpected input {}", c)
            }).collect());
        };
    };

    // let p1 = Point { x: 6, y: 2 };
    // let p2 = Point { x: 9, y: 3 };
    // let line = Line::from_points(&p1, &p2);
    // println!("{:?}", line);
    // println!("{}", line.plug_chug(3));
    // println!("{}", can_see_each_other(&space, &p1, &p2));

    // let p3 = Point { x: 1, y: 3 };
    // let p4 = Point { x: 2, y: 6 };
    // println!("{}", can_see_each_other(&space, &p3, &p4));

    // let p1 = Point { x: 0, y: 0 };
    // let p2 = Point { x: 1, y: 3 };
    // let p3 = Point { x: 1, y: 6 };
    // let p4 = Point { x: 1, y: 9 };
    // println!("{:?}", p4.point_diff(&p2).normalize());

    print_space(&space);
    part1(&space);

    Ok(())
}
