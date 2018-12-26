use std::env;

type Grid<T> = Vec<Vec<T>>;

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

fn construct_grid(serial: i64) -> Grid<i64> {
    let mut grid: Grid<i64> = Vec::with_capacity(300);
    for y in 1..301 {
        grid.push(Vec::with_capacity(300));
        for x in 1..301 {
            grid[y - 1].push(power_level(serial, x as i64, y as i64));
        };
    };

    grid
}

fn construct_summed_area_table(grid: &Grid<i64>) -> Grid<i64> {
    let mut summed_area_table: Grid<i64> = Vec::with_capacity(300);
    for y in 1..301 {
        summed_area_table.push(Vec::with_capacity(300));
        for x in 1..301 {
            let top_sum = if y > 1 {
                summed_area_table[y - 2][x - 1]
            } else {
                0
            };

            let left_sum = if x > 1 {
                summed_area_table[y - 1][x - 2]
            } else {
                0
            };

            let corner_sum = if y > 1 && x > 1 {
                summed_area_table[y - 2][x - 2]
            } else {
                0
            };

            let power = grid[x - 1][y - 1] + left_sum + top_sum - corner_sum;

            summed_area_table[y - 1].push(power);
        };
    };

    summed_area_table
}

fn find_maximum_power(summed_area_table: &Grid<i64>) -> (i64, usize, usize, usize) {
    let (mut max, mut max_x, mut max_y, mut max_d) = (None, None, None, None);
    for d in 1..301 {
        for y in 1..(301 - d) {
            for x in 1..(301 - d) {
                let power = summed_area_table[(x - 1) + d][(y - 1) + d] +
                    summed_area_table[x - 1][y - 1] -
                    summed_area_table[(x - 1) + d][y - 1] -
                    summed_area_table[x - 1][(y - 1) + d];

                if max.is_none() || power > max.unwrap() {
                    max = Some(power);
                    max_x = Some(x + 1);
                    max_y = Some(y + 1);
                    max_d = Some(d);
                };
            };
        };
    };

    (max.unwrap(), max_x.unwrap(), max_y.unwrap(), max_d.unwrap())
}

fn main() -> Result<(), std::io::Error> {
    let serial: i64 = env::args()
        .nth(1)
        .unwrap()
        .parse::<i64>()
        .unwrap();

    let grid = construct_grid(serial);
    let summed_area_table = construct_summed_area_table(&grid);
    let (max, max_x, max_y, max_d) = find_maximum_power(&summed_area_table);
    println!("Power {} found at {},{},{}", max, max_x, max_y, max_d);

    Ok(())
}
