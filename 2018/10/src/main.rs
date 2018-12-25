use std::{env, fmt, fs};
use std::cmp::PartialEq;

struct Point {
    x: i64,
    y: i64,
    v_x: i64,
    v_y: i64
}

impl Point {
    fn new(line: String) -> Self {
        let tokens = line
            .split(|c: char| !c.is_ascii_digit() && c != '-')
            .filter(|&s| s != "")
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        Point {
            x: tokens[0],
            y: tokens[1],
            v_x: tokens[2],
            v_y: tokens[3]
        }
    }

    fn travel(&mut self) {
        self.x += self.v_x;
        self.y += self.v_y;
    }

    fn neighbors(&self) -> Vec<Point> {
        let mut ret = Vec::new();
        for x in -1..2 {
            for y in -1..2 {
                if x != 0 || y != 0 {
                    ret.push(Point {
                        x: self.x + x,
                        y: self.y + y,
                        v_x: 0, v_y: 0
                    });
                };
            };
        };

        ret
    }

    fn has_neighbor(&self, points: &Vec<Point>) -> bool {
        self.neighbors().iter().any(|n| points.contains(n))
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}

fn print_message(points: &Vec<Point>) {
    let mut x_min = points[0].x;
    let mut y_min = points[0].y;
    let mut x_max = points[0].x;
    let mut y_max = points[0].y;
    for point in points.iter() {
        if point.x > x_max {
            x_max = point.x;
        };
        if point.y > y_max {
            y_max = point.y;
        };
        if point.x < x_min {
            x_min = point.x;
        };
        if point.y < y_min {
            y_min = point.y;
        };
    };
    for y in y_min..(y_max + 1) {
        print!("\n");
        for x in x_min..(x_max + 1) {
            if points.contains(&Point {x: x, y: y, v_x: 0, v_y: 0}) {
                print!("#");
            } else {
                print!(".")
            };
        };
    };
    println!("\n");
}

fn simulate(mut points: Vec<Point>) -> (Vec<Point>, u64) {
    let mut num_seconds = 0;
    loop {
        // move each point
        points.iter_mut().for_each(|p| p.travel());
        num_seconds += 1;

        // check if all have neighbors
        if points.iter().all(|p| p.has_neighbor(&points)) {
            return (points, num_seconds);
        };
    };
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let contents: String = fs::read_to_string(&args[1])?;

    let mut points: Vec<Point> = Vec::new();
    for line in contents.split("\n") {
        if line != "" {
            points.push(Point::new(line.to_string()));
        };
    };

    let (message, num_seconds) = simulate(points);
    println!("Number of seconds to resolve = {}", num_seconds);
    print_message(&message);

    Ok(())
}
