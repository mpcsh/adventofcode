use std::env;
use std::fs;
use std::fmt;

#[derive(PartialEq, Eq)]
enum Cart {
    Up,
    Down,
    Left,
    Right,
}

impl Cart {
    fn value(&self) -> char {
        match *self {
            Cart::Up => '^',
            Cart::Down => 'v',
            Cart::Left => '<',
            Cart::Right => '>',
        }
    }
}

impl fmt::Display for Cart {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

#[derive(PartialEq, Eq)]
enum Track {
    Horizontal,
    Vertical,
    Intersection,
    LeftCurve,
    RightCurve,
}

impl Track {
    fn value(&self) -> char {
        match *self {
            Track::Horizontal => '-',
            Track::Vertical => '|',
            Track::Intersection => '+',
            Track::LeftCurve => '\\',
            Track::RightCurve => '/'
        }
    }
}

impl fmt::Display for Track {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}


// struct GridPoint {



fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let contents: String = fs::read_to_string(&args[1])?;

    let mut lines: Vec<String> = Vec::new();
    for line in contents.split("\n") {
        if line != "" {
            lines.push(line.to_string());
        };
    };

    Ok(())
}
