#![feature(option_expect_none)]

use std::env;
use std::fs;

use std::fmt;
use std::cmp::{PartialEq, Eq};

use itertools::Itertools;


#[derive(Clone, Copy, PartialEq, Eq)]
enum Mode {
    Pos = 0,
    Imm = 1
}

impl Mode {
    fn from_i64(i: i64) -> Self {
        match i {
            0 => Mode::Pos,
            1 => Mode::Imm,
            _ => panic!("Unknown mode {}", i)
        }
    }
}

impl fmt::Debug for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", *self as u64)
    }
}


#[derive(Clone, Copy, PartialEq, Eq)]
enum Op {
    Add = 1,
    Mul = 2,
    Inp = 3,
    Out = 4,
    Jnz = 5,
    Jez = 6,
    Ltn = 7,
    Eql = 8,
    Hlt = 99
}

impl Op {
    fn from_i64(i: i64) -> Self {
        match i {
            1 => Op::Add,
            2 => Op::Mul,
            3 => Op::Inp,
            4 => Op::Out,
            5 => Op::Jnz,
            6 => Op::Jez,
            7 => Op::Ltn,
            8 => Op::Eql,
            99 => Op::Hlt,
            _ => panic!("Unknown mode {}", i)
        }
    }
}

impl fmt::Debug for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", *self as u64)
    }
}


struct Instr {
    op: Op,
    modes: Vec<Mode>,
    num_params: usize
}

impl Instr {
    fn from_i64(i: i64) -> Self {
        let op = Op::from_i64(i % 100);

        let mode1 = Mode::from_i64(i / 100 % 10);
        let mode2 = Mode::from_i64(i / 1000 % 10);
        let mode3 = Mode::from_i64(i / 10000 % 10);
        let modes = vec!(mode1, mode2, mode3);

        let num_params = match op {
            Op::Add | Op::Mul => 3,
            Op::Inp | Op::Out => 1,
            Op::Jnz | Op::Jez => 2,
            Op::Ltn | Op::Eql => 3,
            Op::Hlt => 0
        };

        Instr { op, modes, num_params }
    }
}

impl fmt::Debug for Instr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}{:?}{:?}{:?}", self.modes[2], self.modes[1], self.modes[0], self.op)
    }
}


type Program = Vec<i64>;

fn eval<'a, I>(program: &mut Program, mut inputs: I) -> i64
where
    I: Iterator<Item = &'a i64> + 'a,
{
    let mut output = None;

    let mut eip = 0;
    loop {
        let instr = Instr::from_i64(program[eip]);
        if instr.op == Op::Hlt {
            return output.expect("No output!");
        };

        let mut params = Vec::new();
        for i in 0..instr.num_params {
            params.push((instr.modes[i], program[eip + i + 1]));
        };

        let get_param = |(mode, param)| {
            match mode {
                Mode::Pos => program[param as usize],
                Mode::Imm => param
            }
        };
        let get_idx = |(_, param)| param as usize;

        let mut jmp_occurred = false;
        match instr.op {
            Op::Add => {
                let val1 = get_param(params[0]);
                let val2 = get_param(params[1]);
                let ret_idx = get_idx(params[2]);
                program[ret_idx] = val1 + val2;
            },
            Op::Mul => {
                let val1 = get_param(params[0]);
                let val2 = get_param(params[1]);
                let ret_idx = get_idx(params[2]);
                program[ret_idx] = val1 * val2;
            },
            Op::Inp => {
                let ret_idx = get_idx(params[0]);
                program[ret_idx] = *inputs.next().unwrap();
            },
            Op::Out => {
                output.expect_none("Multiple outputs!");
                output = Some(get_param(params[0]));
            },
            Op::Jnz => {
                let val = get_param(params[0]);
                if val != 0 {
                    eip = get_param(params[1]) as usize;
                    jmp_occurred = true;
                };
            },
            Op::Jez => {
                let val = get_param(params[0]);
                if val == 0 {
                    eip = get_param(params[1]) as usize;
                    jmp_occurred = true;
                };
            },
            Op::Ltn => {
                let val1 = get_param(params[0]);
                let val2 = get_param(params[1]);
                let ret_idx = get_idx(params[2]);
                let ret = if val1 < val2 { 1 } else { 0 };
                program[ret_idx] = ret;
            },
            Op::Eql => {
                let val1 = get_param(params[0]);
                let val2 = get_param(params[1]);
                let ret_idx = get_idx(params[2]);
                let ret = if val1 == val2 { 1 } else { 0 };
                program[ret_idx] = ret;
            },
            opcode => panic!("Unknown opcode {:?}", opcode)
        };

        if !jmp_occurred {
            eip += instr.num_params + 1;
        };
    }
}

fn part1(program: &Program) -> () {
    let max_output = [0, 1, 2, 3, 4]
        .iter()
        .permutations(5)
        .map(|phases| {
            phases
                .iter()
                .fold(0, |input, &&phase|
                      eval(&mut program.to_vec(), [phase, input].iter()))
        })
        .max()
        .unwrap();

    println!("Part 1: {}", max_output);
}


fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let contents: String = fs::read_to_string(&args[1])?;

    let program = contents
        .trim()
        .split(|c| c == ',' || c == '\n')
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    part1(&program);

    Ok(())
}
