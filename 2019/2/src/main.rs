use std::env;

use std::iter::successors;

type Program = Vec<usize>;

fn step(program: Program, index: usize) -> Option<Program> {
    let mut new_program = program.to_vec();
    match program[index] {
        99 => None,
        1 => {
            let val1 = program[program[index + 1]];
            let val2 = program[program[index + 2]];
            let store_idx = program[index + 3];
            new_program[store_idx] = val1 + val2;
            Some(new_program)
        },
        2 => {
            let val1 = program[program[index + 1]];
            let val2 = program[program[index + 2]];
            let store_idx = program[index + 3];
            new_program[store_idx] = val1 * val2;
            Some(new_program)
        },
        opcode => panic!("Unknown opcode {}", opcode)
    }
}

fn eval(initial: Program) -> Vec<Program> {
    let mut index = 0;
    successors(Some(initial), |prev| {
        let old_index = index;
        index += 4;
        step(prev.to_vec(), old_index)
    }).collect()
}

fn initialize(program: &Program, noun: usize, verb: usize) -> Program {
    let mut new_program = program.to_vec();
    new_program[1] = noun;
    new_program[2] = verb;

    new_program
}

fn get_result(steps: Vec<Program>) -> usize {
    steps.last().unwrap()[0]
}

fn part1(program: &Program) -> () {
    let initial = initialize(program, 12, 2);
    let steps = eval(initial);
    println!("Part 1: {}", get_result(steps));
}

fn part2(program: &Program) -> () {
    for noun in 0..100 {
        for verb in 0..100 {
            let initial = initialize(program, noun, verb);
            let steps = eval(initial);
            if get_result(steps) == 19690720 {
                println!("Part 2: 100 * {} + {} = {}", noun, verb, 100 * noun + verb);
                return;
            };
        };
    };
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let program = &args[1];

    let toks: Vec<usize> = program.split(",").map(|s| s.parse::<usize>().unwrap()).collect();

    part1(&toks);
    part2(&toks);

    Ok(())
}
