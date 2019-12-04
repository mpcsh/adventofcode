use std::env;

trait DigitsWrapper {
    fn digits(self) -> Vec<u64>;
}

impl DigitsWrapper for u64 {
    fn digits(mut self) -> Vec<u64> {
        let mut digits = Vec::new();

        while self > 0 {
            digits.insert(0, self % 10);
            self /= 10;
        }

        digits
    }
}

fn get_runs(pw: &u64) -> Vec<Vec<u64>> {
    let digits = pw.digits();

    let mut runs = Vec::new();
    let mut current_digit = digits[0];
    let mut current_run = Vec::new();

    for i in digits {
        if i == current_digit {
            current_run.push(i);
        } else {
            if current_run.len() > 1 {
                runs.push(current_run);
            }
            current_digit = i;
            current_run = vec!(current_digit);
        };
    };

    if current_run.len() > 1 {
        runs.push(current_run);
    };

    runs
}

fn contains_adjacent(pw: &u64) -> bool {
    get_runs(pw).len() > 0
}

fn is_nondecreasing(pw: &u64) -> bool {
    let digits = pw.digits();

    digits.iter().zip(digits.iter().skip(1)).all(|(d1, d2)| d1 <= d2)
}

fn contains_pair(pw: &u64) -> bool {
    get_runs(pw)
        .iter()
        .filter(|run| run.len() == 2)
        .count() > 0
}


fn part1(lower_bound: u64, upper_bound: u64) -> usize {
    (lower_bound..upper_bound)
        .filter(|pw| contains_adjacent(pw) && is_nondecreasing(pw))
        .count()
}

fn part2(lower_bound: u64, upper_bound: u64) -> usize {
    (lower_bound..upper_bound)
        .filter(|pw| contains_pair(pw) && is_nondecreasing(pw))
        .count()
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let contents: String = args[1].to_string();

    let range = contents
        .split("-")
        .map(|i| i.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let lower_bound = range[0];
    let upper_bound = range[1];

    println!("Part 1: {}", part1(lower_bound, upper_bound));
    println!("Part 2: {}", part2(lower_bound, upper_bound));

    Ok(())
}
