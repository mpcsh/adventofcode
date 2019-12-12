use std::env;
use std::fs;

use std::fmt;
use std::cmp::{Ordering, PartialEq, Eq};
use std::hash::Hash;
use std::collections::HashSet;


#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Vector {
    x: i64,
    y: i64,
    z: i64
}

impl Vector {
    fn from_str(s: &str) -> Self {
        let mut toks = s
            .split(|c| "<x=,yz>"
                .chars()
                .collect::<Vec<_>>()
                .contains(&c))
            .filter(|&tok| tok != "" && tok != " ")
            .map(|tok| tok.parse::<i64>().expect("Couldn't parse i64!"));

        let x = toks.next().expect("No x component!");
        let y = toks.next().expect("No y component!");
        let z = toks.next().expect("No z component!");

        assert!(toks.next().is_none(), "Extraneous components!");

        Vector { x, y, z }
    }

    fn sum(&self, other: &Self) -> Self {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }

    fn energy(&self) -> u64 {
        (self.x.abs() + self.y.abs() + self.z.abs()) as u64
    }
}

impl fmt::Debug for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<x={}, y={}, z={}>", self.x, self.y, self.z)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Moon {
    pos: Vector,
    vel: Vector
}

impl Moon {
    fn from_pos(pos: Vector) -> Self {
        Moon {
            pos,
            vel: Vector { x: 0, y: 0, z: 0 }
        }
    }
}

impl fmt::Debug for Moon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "pos={:?}, vel={:?}", self.pos, self.vel)
    }
}


fn component_diff(c1: i64, c2: i64) -> (i64, i64) {
    match c1.cmp(&c2) {
        Ordering::Less => (1, -1),
        Ordering::Equal => (0, 0),
        Ordering::Greater => (-1, 1)
    }
}

fn vel_diff(v1: &Vector, v2: &Vector) -> (Vector, Vector) {
    let (x1_diff, x2_diff) = component_diff(v1.x, v2.x);
    let (y1_diff, y2_diff) = component_diff(v1.y, v2.y);
    let (z1_diff, z2_diff) = component_diff(v1.z, v2.z);

    let v1_diff = Vector { x: x1_diff, y: y1_diff, z: z1_diff };
    let v2_diff = Vector { x: x2_diff, y: y2_diff, z: z2_diff };

    (v1_diff, v2_diff)
}

fn step(moons: &mut Vec<Moon>) -> () {
    let mut velocities: Vec<Vector> = moons.iter().map(|m| m.vel).collect();
    // apply gravity: compute velocities
    for (i1, m1) in moons.iter().enumerate() {
        for (i2, m2) in moons.iter().skip(i1 + 1).enumerate() {
            let (v1_diff, v2_diff) = vel_diff(&m1.pos, &m2.pos);
            velocities[i1] = velocities[i1].sum(&v1_diff);
            velocities[i2 + i1 + 1] = velocities[i2 + i1 + 1].sum(&v2_diff);
        };
    };

    // apply velocity: change positions
    for (mut m, v_diff) in moons.iter_mut().zip(velocities.iter()) {
        m.vel = *v_diff;
        m.pos = m.pos.sum(&v_diff);
    };
}

fn compute_energy(moons: &Vec<Moon>) -> u64 {
    moons
        .iter()
        .map(|m| m.pos.energy() * m.vel.energy())
        .sum()
}

fn gcd(mut x: usize, mut y: usize) -> usize {
    while x != 0 {
        let old_x = x;
        x = y % x;
        y = old_x;
    };

    y
}

fn lcm(x: usize, y: usize) -> usize {
    (x * y) / gcd(x, y)
}


fn part1(moons: &Vec<Moon>) -> () {
    let mut moons = moons.to_vec();
    (0..1000).for_each(|_| step(&mut moons));
    println!("Part 1: {:?}", compute_energy(&moons));
}

fn part2(moons: &Vec<Moon>) -> () {
    let selectors: Vec<fn(&Moon) -> (i64, i64)> = vec![
        |m| (m.pos.x, m.vel.x),
        |m| (m.pos.y, m.vel.y),
        |m| (m.pos.z, m.vel.z),
    ];

    let mut cycle_frequencies: Vec<usize> = Vec::new();

    for selector in selectors {
        let mut past_generations: HashSet<Vec<(i64, i64)>> = HashSet::new();
        let mut current_gen = moons.to_vec();
        let mut cycle_frequency = 0;
        while !past_generations.contains(&current_gen.iter().map(selector).collect::<Vec<(i64, i64)>>()) {
            past_generations.insert(current_gen.iter().map(selector).collect::<Vec<(i64, i64)>>());
            step(&mut current_gen);
            cycle_frequency += 1;
        };
        cycle_frequencies.push(cycle_frequency);
    };

    let cycle_point = cycle_frequencies.iter().fold(1, |acc, &freq| lcm(acc, freq));

    println!("Part 2: {}", cycle_point);
}


fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let contents: String = fs::read_to_string(&args[1])?;

    let mut moons: Vec<Moon> = Vec::new();
    for line in contents.split("\n") {
        if line != "" {
            let pos = Vector::from_str(line);
            let moon = Moon::from_pos(pos);
            moons.push(moon);
        };
    };

    part1(&moons);
    part2(&moons);

    Ok(())
}
