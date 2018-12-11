use std::fs;
use std::collections::BTreeMap;

fn manhattan_distance((x1, y1): (i64, i64), (x2, y2): (i64, i64)) -> i64 {
    (x2 - x1).abs() + (y2 - y1).abs()
}

fn part_1(points: &mut BTreeMap<(i64, i64), i64>, x_max: i64, y_max: i64) -> i64 {
    for x in 0..x_max {
        for y in 0..y_max {
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

            if closest_point != None {
                *(points.get_mut(&closest_point.unwrap()).unwrap()) += 1;
            };
        };
    };
    0
}

fn main() -> Result<(), std::io::Error> {
    let contents: String = fs::read_to_string("input.txt")?;

    let mut points: BTreeMap<(i64, i64), i64> = BTreeMap::new();

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

            points.insert((x, y), 0);
        };
    };

    Ok(())
}
