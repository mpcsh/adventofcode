use std::env;
use std::fs;
use std::collections::BTreeMap;

fn manhattan_distance((x1, y1): (i64, i64), (x2, y2): (i64, i64)) -> i64 {
    (x2 - x1).abs() + (y2 - y1).abs()
}

fn part_1(points: &mut BTreeMap<(i64, i64), Option<i64>>, x_max: i64, y_max: i64) -> i64 {
    for x in 0..(x_max + 1) {
        for y in 0..(y_max + 1) {
            let mut min_distance = None;
            let mut closest_point = None;
            for (&point, _) in points.iter() {
                let distance = manhattan_distance((x, y), point);
                if min_distance == None || distance < min_distance.unwrap() {
                    min_distance = Some(distance);
                    closest_point = Some(point);
                } else if distance == min_distance.unwrap() {
                    closest_point = None;
                };
            };

            if ([0, x_max].contains(&x) || [0, y_max].contains(&y))
                && closest_point != None {
                let _ = points.insert(closest_point.unwrap(), None);
            };

            if closest_point != None {
                let area_size: &mut Option<i64> = points
                    .get_mut(&closest_point.unwrap())
                    .unwrap();
                match area_size {
                    None => (),
                    Some(s) => *s += 1
                };
            };
        };
    };

    let mut max_area: Option<i64> = None;
    for area in points.values() {
        match area {
            None => (),
            Some(a) => {
                if max_area == None || *a > max_area.unwrap() {
                    max_area = Some(*a)
                };
            }
        };
    };

    max_area.unwrap()
}

fn part_2(points: &BTreeMap<(i64, i64), Option<i64>>, x_max: i64, y_max: i64) -> i64 {
    let mut region_size = 0;
    for x in 0..(x_max + 1) {
        for y in 0..(y_max + 1) {
            let mut sum_dist = 0;
            for (&point, _) in points.iter() {
                sum_dist += manhattan_distance(point, (x, y));
            };
            if sum_dist < 10000 {
                region_size += 1;
            };
        };
    };

    region_size
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let contents: String = fs::read_to_string(&args[1])?;

    let mut points: BTreeMap<(i64, i64), Option<i64>> = BTreeMap::new();

    let mut x_max = 0;
    let mut y_max = 0;
    for line in contents.split("\n") {
        if line != "" {
            let coords: Vec<i64> = line
                .split(", ")
                .map(|x| x.parse::<i64>().unwrap())
                .collect();
            let (x, y) = (coords[0], coords[1]);

            if x > x_max {
                x_max = x;
            };
            if y > y_max {
                y_max = y;
            }

            points.insert((x, y), Some(0));
        };
    };

    println!("Part 1: largest non-infinite area = {}", part_1(&mut points, x_max, y_max));
    println!("Part 2: region containing locations with distance < 10000 = {}",
             part_2(&points, x_max, y_max));

    Ok(())
}
