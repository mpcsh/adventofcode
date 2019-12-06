use std::env;
use std::fs;

use std::collections::HashMap;


type Galaxy<'a> = HashMap<&'a str, &'a str>;

fn path_to<'a>(galaxy: &'a Galaxy, planet: &'a str, target: &'a str) -> Vec<&'a str> {
    if planet == target {
        vec!()
    } else if planet == "COM" {
        panic!("Does not orbit {}", target)
    } else {
        let mut path = path_to(galaxy, galaxy.get(planet).unwrap(), target);
        path.insert(0, planet);
        path
    }
}

fn part1(galaxy: &Galaxy) -> () {
    let orbit_count = galaxy.keys().fold(0, |sum, planet| sum + path_to(galaxy, planet, "COM").len());
    println!("Part 1: {}", orbit_count);
}

fn part2(galaxy: &Galaxy) -> () {
    let you_path = path_to(galaxy, "YOU", "COM");
    let san_path = path_to(galaxy, "SAN", "COM");

    let tri_point = you_path
        .iter()
        .find_map(|p1| san_path.iter().find(|&p2| p1 == p2))
        .unwrap();

    let xfer_count = path_to(galaxy, "YOU", tri_point).len()
                     + path_to(galaxy, "SAN", tri_point).len()
                     - 2;

    println!("Part 2: {}", xfer_count);
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let contents: String = fs::read_to_string(&args[1])?;

    let mut galaxy: Galaxy = HashMap::new();
    for line in contents.split("\n") {
        if line != "" {
            let orbit = line.split(")").collect::<Vec<_>>();
            let moon = orbit[1];
            let planet = orbit[0];
            galaxy.insert(moon, planet);
        };
    };

    part1(&galaxy);
    part2(&galaxy);

    Ok(())
}
