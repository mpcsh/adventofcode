use std::env;
use std::fs;

use std::fmt;
use std::cmp::{PartialEq, Eq};
use std::hash::Hash;
use std::collections::HashMap;
use std::f64::consts::{PI, FRAC_PI_2};


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

    fn rotate_90(&self) -> Self {
        let (x, y) = (self.x as f64, self.y as f64);
        let theta = FRAC_PI_2;
        let x2 = x * theta.cos() + y * theta.sin();
        let y2 = -x * theta.sin() + y * theta.cos();

        Point { x: x2.round() as i64, y: y2.round() as i64 }
    }

    fn atan(&self) -> f64 {
        let x = self.x as f64;
        let y = self.y as f64;
        y.atan2(x)
    }

    fn scan_angle(&self) -> f64 {
        (self.rotate_90().atan() / PI * 4.0 + 4.0) % 8.0
    }

    fn distance(&self, other: &Point) -> f64 {
        let (x1, y1) = (self.x as f64, self.y as f64);
        let (x2, y2) = (other.x as f64, other.y as f64);
        ((x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1)).sqrt()
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

fn sort_by_scan_angle(grouped: HashMap<Point, Vec<Point>>) -> Vec<Vec<Point>> {
    let mut grouped_as_vec = grouped
        .iter()
        .collect::<Vec<_>>();

    grouped_as_vec.sort_by(|(d1, _), (d2, _)| d1.scan_angle().partial_cmp(&d2.scan_angle()).expect("Couldn't find an ordering!"));

    grouped_as_vec
        .iter()
        .map(|(_, ps)| ps.to_vec())
        .collect()
}

fn optimal_station(space: &Space) -> (Point, usize) {
    space
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
        .expect("No maximum point!")
}

fn scan(space: &Space) -> Vec<Point> {
    let (origin, _) = optimal_station(space);
    let mut sorted_by_angle = sort_by_scan_angle(group_by_angle(space, &origin));
    sorted_by_angle
        .iter_mut()
        .for_each(|points| points
                  .sort_by(|point1, point2|
                           origin.distance(&point1).partial_cmp(
                               &origin.distance(&point2))
                           .expect("No ordering on distance found!")
                  )
        );

    let mut scan_order = Vec::new();
    while !sorted_by_angle.iter().all(|points| points.is_empty()) {
        sorted_by_angle
            .iter_mut()
            .for_each(|points| {
                if !points.is_empty() {
                    let point = points.remove(0);
                    if point != origin {
                        scan_order.push(point);
                    };
                };
            });
    };

    scan_order
}


fn part1(space: &Space) -> () {
    let (max_point, max_count) = optimal_station(space);

    println!("Part 1: {:?} can see {} asteroids", max_point, max_count);
}

fn part2(space: &Space) -> () {
    let lucky = scan(space)[199];
    println!("{}", lucky.x * 100 + lucky.y);
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

    let unit_vecs: Vec<Point> = vec![
		Point { x: 0, y: -1 }, Point { x: 1, y: -1 },
		Point { x: 1, y: 0 }, Point { x: 1, y: 1 },
        Point { x: 0, y: 1 }, Point { x: -1, y: 1 },
	    Point { x: -1, y: 0 }, Point { x: -1, y: -1000 }
	];
    for (p, dir) in unit_vecs.iter().zip(vec!["U", "UR", "R", "RD", "D", "DL", "L", "LU"]) {
        println!("{}: {:?}, {:?}", dir, p, p.scan_angle());
    };

    part2(&space);

    Ok(())
}
