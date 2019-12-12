use std::env;
use std::fs;

use std::fmt;
use std::cmp::{Ordering, PartialEq, Eq};


#[derive(Clone, Copy, PartialEq, Eq)]
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
}

impl fmt::Debug for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<x={}, y={}, z={}>", self.x, self.y, self.z)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
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


fn vel_diff(c1: i64, c2: i64) -> (i64, i64) {
    match c1.cmp(&c2) {
        Ordering::Less => (1, -1),
        Ordering::Equal => (0, 0),
        Ordering::Greater => (-1, 1)
    }
}

fn update_velocities(m1: &mut Moon, m2: &mut Moon) -> () {
    let (vx1_diff, vx2_diff) = vel_diff(m1.pos.x, m2.pos.x);
    let (vy1_diff, vy2_diff) = vel_diff(m1.pos.y, m2.pos.y);
    let (vz1_diff, vz2_diff) = vel_diff(m1.pos.z, m2.pos.z);

    m1.vel.x += vx1_diff;
    m1.vel.y += vy1_diff;
    m1.vel.z += vz1_diff;

    m2.vel.x += vx2_diff;
    m2.vel.y += vy2_diff;
    m2.vel.z += vz2_diff;
}

fn step(moons: &Vec<Moon>) -> Vec<Moon> {
    let mut new_moons: Vec<Moon> = moons.iter().map(|&m| m).collect();

    // apply gravity: change velocities
    for (i1, _) in moons.iter().enumerate() {
        for (i2, _) in moons.iter().skip(i1 + 1).enumerate() {
            let new_m1: &mut Moon = &mut new_moons[i1];
            let new_m2: &mut Moon = &mut new_moons[i2 + i1];
            // let mut new_m1 = new_moons.get_mut(i1).expect("No new_m1!");
            // let mut new_m2 = new_moons.get_mut(i2 + i1).expect("No new_m2!");
            update_velocities(new_m1, new_m2);
        };
    };

    // apply velocity: change positions
    for mut new_m in new_moons.iter_mut() {
        new_m.pos = new_m.pos.sum(&new_m.vel);
    };

    new_moons
}


fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let contents: String = fs::read_to_string(&args[1])?;

    let mut moons: Vec<Moon> = Vec::new();
    for line in contents.split("\n") {
        if line != "" {
            let pos = Vector::from_str(line);
            let moon = Moon::from_pos(pos);
            // println!("{:?}", moon);
            moons.push(moon);
        };
    };

    println!("{:#?}", moons);
    println!("{:#?}", step(&moons));

    Ok(())
}
