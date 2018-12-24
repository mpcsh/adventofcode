use std::{env, fmt, fs};

struct Point {
    x: i64,
    y: i64,
    v_x: i64,
    v_y: i64
}

impl Point {
    fn new(line: String) -> Self {
        let tokens = line
            .split(|c: char| !c.is_ascii_digit())
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
        for neighbor in self.neighbors().iter() {
            if points.contains(neighbor) {
                return true
            };

        };
        false
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
    let mut x_min = 0;
    let mut y_min = 0;
    let mut x_max = 0;
    let mut y_max = 0;
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
    for x in x_min..(x_max + 1) {
        print!("\n");
        for y in y_min..(y_max + 1) {
            if points.contains(&Point {x: x, y: y, v_x: 0, v_y: 0}) {
                print!("#");
            } else {
                print!(".")
            };
        };
    };
    print!("\n");
}

fn part_1(mut points: Vec<Point>) -> Vec<Point> {
    let mut all_have_neighbors = false;
    let mut x_max = 0;
    let mut x_min = 0;
    let mut y_max = 0;
    let mut y_min = 0;
    while !all_have_neighbors {
        print_message(&points);
        // travel
        for point in points.iter_mut() {
            point.travel();
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
        // check if all have neighbors
        all_have_neighbors = true;
        for x in x_min..(x_max + 1) {
            for y in y_min..(y_max + 1) {
                let curr_point = Point {x: x, y: y, v_x: 0, v_y: 0};
                if !curr_point.has_neighbor(&points) {
                    all_have_neighbors = false;
                };
            };
        };
    };

    points
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

    let message = part_1(points);
    println!("{:?}", message);
    Ok(())
}
