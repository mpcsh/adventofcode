use std::fs;
use std::fmt;
use std::vec::Vec;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

type ClaimId = i64;
type Canvas = HashMap<Point, Vec<ClaimId>>;

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

impl Claim {
    fn new(line: &String) -> Claim {
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

    fn place_point(&self, canvas: &mut Canvas, point: Point) -> () {
        canvas.entry(point).or_insert(Vec::new()).push(self.id)
    }

    fn place(&self, canvas: &mut Canvas) -> () {
        let Point {x, y} = self.origin;
        let (width, height) = (self.width, self.height);

        for r in y..y + height {
            for c in x..x + width {
                self.place_point(canvas, Point {x: c, y: r});
            };
        };
    }
}

fn part_1(claims: &Vec<Claim>) -> i64 {
    let mut canvas: Canvas = HashMap::new();

    for claim in claims {
        claim.place(&mut canvas);
    };

    let mut num_overlaps = 0;
    for (_, claimed_by) in canvas.iter() {
        if claimed_by.len() >= 2 {
            num_overlaps += 1;
        };
    };

    num_overlaps
}

fn main() -> Result<(), std::io::Error> {
    let contents: String = fs::read_to_string("input.txt")?;

    let mut lines: Vec<String> = Vec::new();
    for line in contents.split("\n") {
        if line != "" {
            lines.push(line.to_string());
        };
    };

    let claims: Vec<Claim> = lines.into_iter().map(|l| Claim::new(&l)).collect();
    println!("Part 1: in^2 of fabric overlapped = {}", part_1(&claims));

    Ok(())
}
