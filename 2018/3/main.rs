use std::fs;
use std::fmt;
use std::vec::Vec;

struct Point {
    x: i64,
    y: i64
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

struct Claim {
    id: i64,
    origin: Point,
    width: i64,
    height: i64
}

impl fmt::Display for Claim {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Claim #{}: origin {}, width {}, height {}",
               self.id, self.origin, self.width, self.height)
    }
}

fn claim_init(line: &String) -> Claim {
    let fields: Vec<i64> = line
        .split(|c| vec![' ', '#', '@', ',', ':', 'x'].contains(&c))
        .filter(|f| *f != "")
        .map(|f| f.parse::<i64>().unwrap())
        .collect();

    Claim {
        id: fields[0],
        origin: Point { x: fields[1], y: fields[2] },
        width: fields[3],
        height: fields[4]
    }
}

fn part_1(claims: &Vec<Claim>) -> i64 {

    0
}

fn main() -> Result<(), std::io::Error> {
    let contents: String = fs::read_to_string("input.txt")?;

    let mut lines: Vec<String> = Vec::new();
    for line in contents.split("\n") {
        if line != "" {
            lines.push(line.to_string());
        };
    };

    let claims = lines.into_iter().map(|l| claim_init(&l)).collect();

    println!("Part 1: in^2 of fabric overlapped = {}", part_1(&claims));

    Ok(())
}
