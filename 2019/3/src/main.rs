use std::env;
use std::fs;

use std::fmt;
use std::cmp;
use std::hash;
use std::collections::HashMap;


#[derive(cmp::PartialEq, cmp::Eq, hash::Hash)]
struct Coord {
    x: i64,
    y: i64
}

impl fmt::Debug for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn manhattan_distance(c1: &Coord, c2: &Coord) -> u64 {
    ((c1.x - c2.x).abs() + (c1.y - c2.y).abs()) as u64
}


#[derive(fmt::Debug)]
enum Direction {
    U,
    R,
    D,
    L
}

struct Instruction {
    direction: Direction, 
    paces: u64
}

impl Instruction {
    fn from_str(s: &str) -> Self {
        let direction: Direction = match s.chars().nth(0).unwrap() {
            'U' => Direction::U,
            'R' => Direction::R,
            'D' => Direction::D,
            'L' => Direction::L,
            d => panic!("Unknown direction {}", d)
        };

        let paces: u64 = s[1..].parse().unwrap();

        Instruction {
            direction,
            paces
        }
    }
}

type Path = Vec<Instruction>;

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}{}", self.direction, self.paces)
    }
}


type Wire = u64;
type Grid = HashMap<Coord, HashMap<Wire, u64>>;

fn travel(grid: &mut Grid, wire: Wire, path: &Path) -> () {
    let mut x = 0;
    let mut y = 0;
    let mut step_count = 0;

    for instruction in path {
        for _ in 0..instruction.paces {
            match instruction.direction {
                Direction::U => y += 1,
                Direction::R => x += 1,
                Direction::D => y -= 1,
                Direction::L => x -= 1
            };
            step_count += 1;

            let pos = Coord { x, y };
            match grid.get_mut(&pos) {
                None => {
                    let mut wires = HashMap::new();
                    wires.insert(wire, step_count);
                    grid.insert(pos, wires);
                },
                Some(wires) => {
                    match wires.get_mut(&wire) {
                        Some(_) => (),
                        None => { wires.insert(wire, step_count); }
                    };
                }
            };
        };
    };
}


fn part1(intersections: &Vec<(&Coord, &HashMap<Wire, u64>)>) -> () {
    let origin = Coord { x: 0, y: 0 };
    let (closest_to_origin, _) = intersections
        .iter()
        .min_by(|(i1, _), (i2, _)|
            manhattan_distance(i1, &origin)
                .cmp(&manhattan_distance(i2, &origin)))
        .unwrap();

    println!("Part 1: {}", manhattan_distance(closest_to_origin, &origin));
}

fn part2(intersections: &Vec<(&Coord, &HashMap<Wire, u64>)>) -> () {
    let minimum_delay = intersections
        .iter()
        .min_by(|(_, ws1), (_, ws2)|
                ws1.values().sum::<u64>().cmp(&ws2.values().sum::<u64>()))
        .unwrap()
        .1
        .values()
        .sum::<u64>();

    println!("Part 2: {}", minimum_delay);
}


fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let contents: String = fs::read_to_string(&args[1])?;

    let mut lines: Vec<String> = Vec::new();
    for line in contents.split("\n") {
        if line != "" {
            lines.push(line.to_string());
        };
    };

    let paths: Vec<Path> = lines
        .iter()
        .map(|line| line
            .split(",")
            .map(Instruction::from_str)
            .collect())
        .collect();

    let mut grid: Grid = HashMap::new();

    for (wire, path) in (1..).zip(paths) {
        travel(&mut grid, wire, &path);
    };

    let intersections: Vec<(&Coord, &HashMap<Wire, u64>)> = grid
        .iter()
        .filter(|(_, wires)| wires.len() >= 2)
        .collect();

    part1(&intersections);
    part2(&intersections);

    Ok(())
}
