#![feature(option_expect_none)]

use std::env;
use std::fs;

use std::fmt;
use std::cmp::{PartialEq, Eq};
use std::collections::VecDeque;

use itertools::Itertools;
use log::debug;


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
    arity: usize
}

impl Instr {
    fn from_i64(i: i64) -> Self {
        let op = Op::from_i64(i % 100);

        let mode1 = Mode::from_i64(i / 100 % 10);
        let mode2 = Mode::from_i64(i / 1000 % 10);
        let mode3 = Mode::from_i64(i / 10000 % 10);
        let modes = vec!(mode1, mode2, mode3);

        let arity = match op {
            Op::Add | Op::Mul => 3,
            Op::Inp | Op::Out => 1,
            Op::Jnz | Op::Jez => 2,
            Op::Ltn | Op::Eql => 3,
            Op::Hlt => 0
        };

        Instr { op, modes, arity }
    }
}

impl fmt::Debug for Instr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}{:?}{:?}{:?}", self.modes[2], self.modes[1], self.modes[0], self.op)
    }
}


type Program = Vec<i64>;

#[derive(PartialEq, Eq, Debug)]
enum State {
    Boot,
    Running,
    InputWait,
    Output,
    Halted
}

struct Node {
    label: char,

    state: State,
    program: Program,
    ip: usize,

    input: VecDeque<i64>,
    output: VecDeque<i64>,
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Node {} in state {:?}\n\
                    \tInstruction pointer: {}\n\
                    \tInput: {:?}\n\
                    \tOutput: {:?}",
                self.label, self.state,
                self.ip,
                self.input,
                self.output)
    }
}

fn run(node: &mut Node) -> () {
    loop {
        node.state = State::Running;

        let instr = Instr::from_i64(node.program[node.ip]);

        let mut params = Vec::new();
        for i in 0..instr.arity {
            params.push((instr.modes[i], node.program[node.ip + i + 1]));
        };

        let get_param = |(mode, param)| {
            match mode {
                Mode::Pos => node.program[param as usize],
                Mode::Imm => param
            }
        };
        let get_idx = |(_, param)| param as usize;

        debug!("Node {}: executing instruction {:?} with parameters {:?}",
               node.label,
               instr.op, params);

        let mut jmp_occurred = false;
        match instr.op {
            Op::Add => {
                let val1 = get_param(params[0]);
                let val2 = get_param(params[1]);
                let ret_idx = get_idx(params[2]);
                debug!("Node {}: storing {} + {} = {} at index {}",
                       node.label,
                       val1, val2, val1 + val2,
                       ret_idx);
                node.program[ret_idx] = val1 + val2;
            },
            Op::Mul => {
                let val1 = get_param(params[0]);
                let val2 = get_param(params[1]);
                let ret_idx = get_idx(params[2]);
                debug!("Node {}: storing {} * {} = {} at index {}",
                       node.label,
                       val1, val2, val1 * val2,
                       ret_idx);
                node.program[ret_idx] = val1 * val2;
            },
            Op::Inp => {
                match node.input.pop_front() {
                    Some(input) => {
                        let idx = get_idx(params[0]);
                        debug!("Node {}: storing input {} at index {}",
                               node.label,
                               input, idx);
                        node.program[idx] = input;
                    },
                    None => {
                        debug!("Node {}: no input available",
                               node.label);
                        node.state = State::InputWait;
                        return;
                    }
                };
            },
            Op::Out => {
                let output = get_param(params[0]);
                debug!("Node {}: outputting value {}",
                       node.label,
                       output);
                node.output.push_back(output);
                node.state = State::Output;
            },
            Op::Jnz => {
                let val = get_param(params[0]);
                if val != 0 {
                    node.ip = get_param(params[1]) as usize;
                    jmp_occurred = true;
                };
            },
            Op::Jez => {
                let val = get_param(params[0]);
                if val == 0 {
                    node.ip = get_param(params[1]) as usize;
                    jmp_occurred = true;
                };
            },
            Op::Ltn => {
                let val1 = get_param(params[0]);
                let val2 = get_param(params[1]);
                let ret_idx = get_idx(params[2]);
                let ret = if val1 < val2 { 1 } else { 0 };
                node.program[ret_idx] = ret;
            },
            Op::Eql => {
                let val1 = get_param(params[0]);
                let val2 = get_param(params[1]);
                let ret_idx = get_idx(params[2]);
                let ret = if val1 == val2 { 1 } else { 0 };
                node.program[ret_idx] = ret;
            },
            Op::Hlt => {
                node.state = State::Halted;
                return;
            }
        };

        if !jmp_occurred {
            node.ip += instr.arity + 1;
        };
    }
}


fn run_with_phases(program: &Program, phases: Vec<&i64>) -> i64 {
    let labels = ['A', 'B', 'C', 'D', 'E'];
    let mut nodes: Vec<Node> = labels
        .iter()
        .zip(phases.iter())
        .map(|(&label, &&phase)| Node {
            label,

            state: State::Boot,
            program: program.to_vec(),
            ip: 0,

            input: VecDeque::from(vec!(phase)),
            output: VecDeque::new(),
        })
        .collect();

    nodes[0].input.push_back(0);

    let (mut curr_node, mut next_node) = (0, 1);
    loop {
        debug!("BEGIN RUN: {:?}", nodes[curr_node]);
        run(&mut nodes[curr_node]);
        debug!("END RUN: {:?}", nodes[curr_node]);
        if nodes.iter().all(|node| node.state == State::Halted) {
            break;
        };
        match nodes[curr_node].output.pop_front() {
            Some(output) => {
                nodes[next_node].input.push_back(output);
            },
            None => ()
        };
        curr_node += 1;
        curr_node %= nodes.len();
        next_node += 1;
        next_node %= nodes.len();
    }

    let outputs = &nodes.last().expect("No last node!").output;
    assert!(outputs.len() < 2, "Multiple outputs!");
    *outputs.front().expect("No outputs!")
}

fn find_max_output(program: &Program, possible_phases: Vec<i64>) -> i64 {
    possible_phases
        .iter()
        .permutations(possible_phases.len())
        .map(|phases| run_with_phases(program, phases.to_vec()))
        .max()
        .expect("No maximum output!")
}

fn part1(program: &Program) -> () {
    let max_output = find_max_output(program, vec!(0, 1, 2, 3, 4));
    println!("Part 1: {}", max_output);
}


fn part2(program: &Program) -> () {
    let max_output = find_max_output(program, vec!(5, 6, 7, 8, 9));
    println!("Part 2: {}", max_output);
}


fn main() -> Result<(), std::io::Error> {
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    let contents: String = fs::read_to_string(&args[1])?;

    let program = contents
        .trim()
        .split(|c| c == ',' || c == '\n')
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    part1(&program);
    part2(&program);

    Ok(())
}
