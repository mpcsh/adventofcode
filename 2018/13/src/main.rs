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

struct GridPoint {
    cart: Option<Cart>,
    track: Option<Track>,
    position: (usize, usize)
}

impl GridPoint {
    fn from_char(c: char, position: (usize, usize)) -> Self {
        let cart = match c {
            '^' => Some(Cart::Up),
            'v' => Some(Cart::Down),
            '<' => Some(Cart::Left),
            '>' => Some(Cart::Right),
            _ => None
        };

        let track = match c {
            '^' | 'v' | '|' => Some(Track::Vertical),
            '<' | '>' | '-' => Some(Track::Horizontal),
            '+' => Some(Track::Intersection),
            '\\' => Some(Track::LeftCurve),
            '/' => Some(Track::RightCurve),
            _ => None
        };

        GridPoint {
            cart,
            track,
            position
        }
    }
}

impl fmt::Display for GridPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display = self.cart.as_ref().map_or(self.track.as_ref().map_or(' ', |t| t.value()), |c| c.value());
        write!(f, "{}", display)
    }
}


fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let contents: String = fs::read_to_string(&args[1])?;

    let mut carts: Vec<(usize, usize)> = Vec::new();
    let mut grid: Vec<Vec<GridPoint>> = Vec::new();

    for (y, line) in contents.split("\n").enumerate() {
        if line != "" {
            let mut row = Vec::new();
            for (x, c) in line.chars().enumerate() {
                let gp = GridPoint::from_char(c, (x, y));
                if gp.cart.is_some() {
                    carts.push((x, y));
                };
                row.push(gp);
            };

            grid.push(row);
        };
    };

    grid
        .iter()
        .for_each(|r| {
            r.iter()
             .for_each(|gp| print!("{}", gp));
            println!();
        });

    Ok(())
}
