use std::env;
use std::fs;

use std::fmt;


#[derive(Clone, Copy)]
struct Point {
    x: usize,
    y: usize
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Clone, Copy)]
struct Line {
    m: f64,
    b: f64
}

impl fmt::Debug for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "y = {}x + {}", self.m, self.b)
    }
}

impl Line {
    fn from_points(p1: &Point, p2: &Point) -> Self {
        let (x1, y1) = (p1.x as f64, p1.y as f64);
        let (x2, y2) = (p2.x as f64, p2.y as f64);
        let m: f64 = (y2 - y1) / (x2 - x1);
        let b: f64 = y1 - m*x1;

        Line { m, b }
    }

    fn plug_chug(&self, x: usize) -> f64 {
        self.m * (x as f64) + self.b
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

fn can_see_each_other(space: &Space, p1: &Point, p2: &Point) -> bool {
    let line = Line::from_points(p1, p2);

    for x in p1.x..=p2.x {
        for y in p1.y..=p2.y {
            if (x == p1.x && y == p1.y) || (x == p2.x && y == p2.y) {
                continue;
            };

            println!("p1: {:?}, p2: {:?}, line: {:?}, p3: {:?}", p1, p2, line, Point { x, y });

            if line.plug_chug(x) == y as f64 && space[x][y] {
                return false;
            };
        }
    }

    true
}

fn get_visible_from(space: &Space, p1: &Point) -> Vec<Point> {
    space
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row
                .iter()
                .enumerate()
                .map(move |(x, loc)| (loc, Point { x, y }))
                .filter(|(&loc, p2)| can_see_each_other(space, p1, &p2))
        })
        .flatten()
        .map(|(_, p2)| p2)
        .collect()
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

    let p5 = Point { x: 1, y: 0 };
    let p6 = Point { x: 1, y: 3 };
    println!("{}", can_see_each_other(&space, &p5, &p6));

    print_space(&space);

    Ok(())
}
