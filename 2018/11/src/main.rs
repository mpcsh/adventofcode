use std::env;

fn power_level(serial: i64, x: i64, y: i64) -> i64 {
    let rack_id = x + 10;
    let mut power_level = rack_id * y;
    power_level += serial;
    power_level *= rack_id;
    power_level /= 100;
    power_level %= 10;
    power_level -= 5;

    power_level
}

fn construct_grid(serial: i64) -> Vec<Vec<i64>> {
    let mut grid: Vec<Vec<i64>> = Vec::with_capacity(300);
    for y in 1..301 {
        grid.push(Vec::with_capacity(300));
        for x in 1..301 {
            grid[y - 1].push(power_level(serial, x as i64, y as i64));
        };
    };

    grid
}

fn find_maximum_power(grid: &Vec<Vec<i64>>) -> (usize, usize, usize) {
    let mut max = None;
    let mut max_x = None;
    let mut max_y = None;
    let mut max_d = None;

    // greedy stays ahead
    for d in 1..301 {
        for y in 1..(301 - d) {
            for x in 1..(301 - d) {
                let power: i64 = grid
                    .iter()
                    .skip(y - 1)
                    .take(d)
                    .flat_map(|row| row.iter().skip(x - 1).take(d))
                    .sum();

                if max.is_none() || power > max.unwrap() {
                    max = Some(power);
                    max_x = Some(x);
                    max_y = Some(y);
                    max_d = Some(d);
                };
            };
        };
    };

    (max_x.unwrap(), max_y.unwrap(), max_d.unwrap())
}

fn main() -> Result<(), std::io::Error> {
    let serial: i64 = env::args()
        .nth(1)
        .unwrap()
        .parse::<i64>()
        .unwrap();

    let grid = construct_grid(serial);
    let (max_x, max_y, max_d) = find_maximum_power(&grid);
    println!("{},{},{}", max_x, max_y, max_d);

    Ok(())
}
