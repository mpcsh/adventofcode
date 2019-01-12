use std::env;
use std::fs;
use std::fmt;
use std::collections::BTreeSet;

#[derive(PartialEq, Eq, Clone, Copy)]
enum CartDirection {
    Up,
    Down,
    Left,
    Right,
}

impl CartDirection {
    fn value(&self) -> char {
        match *self {
            CartDirection::Up => '^',
            CartDirection::Down => 'v',
            CartDirection::Left => '<',
            CartDirection::Right => '>',
        }
    }
}

#[derive(Clone)]
struct Cart {
    direction: CartDirection,
    num_intersections: u64
}

impl Cart {
    fn increment(&self) -> (isize, isize) {
        match self.direction {
            CartDirection::Up => (-1, 0),
            CartDirection::Down => (1, 0),
            CartDirection::Left => (0, -1),
            CartDirection::Right => (0, 1)
        }
    }

    fn intersection(&mut self) -> CartDirection {
        let new_direction = match self.num_intersections % 3 {
            0 => match self.direction { // turn left
                CartDirection::Up => CartDirection::Left,
                CartDirection::Left => CartDirection::Down,
                CartDirection::Down => CartDirection::Right,
                CartDirection::Right => CartDirection::Up,
            },
            1 => self.direction, // move straight
            2 => match self.direction { // turn right
                CartDirection::Up => CartDirection::Right,
                CartDirection::Right => CartDirection::Down,
                CartDirection::Down => CartDirection::Left,
                CartDirection::Left => CartDirection::Up,
            },
            _ => panic!("Math didn't work???")
        };

        self.num_intersections += 1;

        new_direction
    }
}

impl fmt::Display for Cart {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.direction.value())
    }
}


#[derive(PartialEq, Eq, Clone)]
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

#[derive(Clone)]
struct GridPoint {
    cart: Option<Cart>,
    track: Option<Track>
}

impl GridPoint {
    fn from_char(c: char) -> Self {
        let cart = match c {
            '^' => Some(Cart {direction: CartDirection::Up, num_intersections: 0}),
            'v' => Some(Cart {direction: CartDirection::Down, num_intersections: 0}),
            '<' => Some(Cart {direction: CartDirection::Left, num_intersections: 0}),
            '>' => Some(Cart {direction: CartDirection::Right, num_intersections: 0}),
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
            track
        }
    }
}

impl fmt::Display for GridPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display = self.cart.as_ref()
            .map_or(
                self.track.as_ref().map_or(' ',
                                           |track| track.value()),
               |cart| cart.direction.value());
        write!(f, "{}", display)
    }
}

type Grid = Vec<Vec<GridPoint>>;
type Position = (usize, usize);
type Carts = BTreeSet<Position>;

fn print_grid(grid: &Grid) {
    grid
        .iter()
        .for_each(|r| {
            r.iter()
             .for_each(|gp| print!("{}", gp));
            println!();
        });
}


fn tick(grid: &mut Grid, carts: &Carts) -> (Carts, Carts) {
    let mut new_carts = BTreeSet::new();
    let mut collisions = BTreeSet::new();

    for &(r, c) in carts.iter() {
        if collisions.contains(&(r, c)) {
            continue;
        };
        let gp = &mut grid[r][c];
        let mut cart = gp.cart.take().unwrap();

        // turn the cart
        cart.direction = match gp.track.as_ref().unwrap() {
            Track::Horizontal | Track::Vertical => cart.direction,
            Track::LeftCurve => match cart.direction {
                CartDirection::Up => CartDirection::Left,
                CartDirection::Down => CartDirection::Right,
                CartDirection::Left => CartDirection::Up,
                CartDirection::Right => CartDirection::Down,
            },
            Track::RightCurve => match cart.direction {
                CartDirection::Up => CartDirection::Right,
                CartDirection::Down => CartDirection::Left,
                CartDirection::Left => CartDirection::Down,
                CartDirection::Right => CartDirection::Up,
            },
            Track::Intersection => cart.intersection()
        };

        // move it to its new position
        let (inc_r, inc_c) = cart.increment();
        let (new_r, new_c) = ((r as isize + inc_r) as usize, (c as isize + inc_c) as usize);

        let new_gp = &mut grid[new_r][new_c];
        if new_gp.cart.is_some() {
            let _ = new_gp.cart.take();
            collisions.insert((new_r, new_c));
        } else {
            new_gp.cart = Some(cart);
            new_carts.insert((new_r, new_c));
        };
    };

    (new_carts, collisions)
}

fn part_1(mut grid: Grid, mut carts: Carts) -> Position {
    loop {
       let (new_carts, collisions) = tick(&mut grid, &carts);
       if !collisions.is_empty() {
           return *collisions.iter().next().unwrap();
       };
       carts = new_carts;
    };
}

fn part_2(mut grid: Grid, mut carts: Carts) -> Position {
    loop {
        let (new_carts, collisions) = tick(&mut grid, &carts);
        carts = new_carts.difference(&collisions).cloned().collect();
        if carts.len() == 1 {
            return *carts.iter().next().unwrap();
        };
    };
}


fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let contents: String = fs::read_to_string(&args[1])?;

    let mut carts: Carts = BTreeSet::new();
    let mut grid: Grid = Vec::new();

    for (r, line) in contents.split("\n").enumerate() {
        if line != "" {
            let mut row = Vec::new();
            for (c, symb) in line.chars().enumerate() {
                let gp = GridPoint::from_char(symb);
                if gp.cart.is_some() {
                    carts.insert((r, c));
                };
                row.push(gp);
            };

            grid.push(row);
        };
    };

    let (r, c) = part_1(grid.to_vec(), carts.clone());
    println!("Part 1: first collision occurs at {},{}", c, r);
    let (r, c) = part_2(grid.to_vec(), carts.clone());
    println!("Part 2: last cart standing is at {},{}", c, r);

    Ok(())
}
