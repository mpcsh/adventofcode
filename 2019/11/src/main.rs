#![feature(option_expect_none)]

use std::env;
use std::fs;
use std::io;

use std::fmt;
use std::hash::Hash;
use std::cmp::{PartialEq, Eq};
use std::collections::{HashMap, VecDeque};

use log::{debug, trace, log_enabled};
use log::Level::Trace;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    Pos = 0,
    Imm = 1,
    Rel = 2
}

impl Mode {
    fn from_i64(i: i64) -> Self {
        match i {
            0 => Mode::Pos,
            1 => Mode::Imm,
            2 => Mode::Rel,
            _ => panic!("Unknown mode {}", i)
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Op {
    Add = 1,
    Mul = 2,
    Inp = 3,
    Out = 4,
    Jnz = 5,
    Jez = 6,
    Ltn = 7,
    Eql = 8,
    Rbo = 9,
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
            9 => Op::Rbo,
            99 => Op::Hlt,
            _ => panic!("Unknown mode {}", i)
        }
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
            Op::Rbo => 1,
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
    OutputYield,
    Halted
}

struct Node {
    label: char,

    state: State,
    program: Program,
    ip: usize,
    rb: usize,

    input: VecDeque<i64>,
    output: VecDeque<i64>
}

impl Node {
    fn get(&self, index: usize) -> i64 {
        if index < self.program.len() {
            self.program[index]
        } else { 0 }
    }
    fn set(&mut self, index: usize, value: i64) -> () {
        if index >= self.program.len() {
            self.program.resize(index + 1, 0);
        };
        self.program[index] = value;
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Node {} in state {:?}\n\
                    \tInstruction pointer: {}, relative base: {}\n\
                    \tInput: {:?}\n\
                    \tOutput: {:?}",
                self.label, self.state,
                self.ip, self.rb,
                self.input,
                self.output)
    }
}

struct Param {
    mode: Mode,
    raw: i64,
    as_idx: Option<usize>,
    deref: i64
}

impl fmt::Debug for Param {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?}, {}, {})", self.mode, self.raw, self.deref)
    }
}

fn run(node: &mut Node) -> () {
    loop {
        node.state = State::Running;

        let instr = Instr::from_i64(node.get(node.ip));

        let mut params = Vec::new();
        for i in 0..instr.arity {
            let mode = instr.modes[i];
            let raw = node.get(node.ip + i + 1);
            params.push(Param {
                mode,
                raw,
                as_idx: match mode {
                    Mode::Pos => Some(raw as usize),
                    Mode::Rel => Some(((node.rb as i64) + raw) as usize),
                    Mode::Imm => None
                },
                deref: match mode {
                    Mode::Pos => node.get(raw as usize),
                    Mode::Imm => raw,
                    Mode::Rel => node.get(((node.rb as i64) + raw) as usize)
                },
            });
                    
        };

        let ternary_setup = || (params[0].deref, params[1].deref, params[2].as_idx.expect("No index set!"));

        trace!("{:?}", node);
        trace!("{:?}", node.program);
        trace!("ip: {} -> {}", node.ip, node.get(node.ip));
        debug!("About to execute instruction {:?} with parameters {:?}",
               instr.op, params);

        if log_enabled!(Trace) {
            println!("Press Enter to execute");
            let _ = io::stdin().read_line(&mut String::new()).expect("Couldn't read line!");
        };

        let mut jmp_occurred = false;
        match instr.op {
            Op::Add => {
                let (val1, val2, ret_idx) = ternary_setup();
                node.set(ret_idx, val1 + val2);
            },
            Op::Mul => {
                let (val1, val2, ret_idx) = ternary_setup();
                node.set(ret_idx, val1 * val2);
            },
            Op::Inp => {
                match node.input.pop_front() {
                    Some(input) => {
                        let idx = params[0].as_idx.expect("No index set!");
                        node.set(idx, input);
                    },
                    None => {
                        node.state = State::InputWait;
                        return;
                    }
                };
            },
            Op::Out => {
                let output = params[0].deref;
                node.output.push_back(output);
                node.state = State::OutputYield;
                node.ip += instr.arity + 1;
                return;
            },
            Op::Jnz => {
                let val = params[0].deref;
                if val != 0 {
                    node.ip = params[1].deref as usize;
                    jmp_occurred = true;
                };
            },
            Op::Jez => {
                let val = params[0].deref;
                if val == 0 {
                    node.ip = params[1].deref as usize;
                    jmp_occurred = true;
                };
            },
            Op::Ltn => {
                let (val1, val2, ret_idx) = ternary_setup();
                let ret = if val1 < val2 { 1 } else { 0 };
                node.set(ret_idx, ret);
            },
            Op::Eql => {
                let (val1, val2, ret_idx) = ternary_setup();
                let ret = if val1 == val2 { 1 } else { 0 };
                node.set(ret_idx, ret);
            },
            Op::Rbo => {
                node.rb = ((node.rb as i64) + params[0].deref) as usize;
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


#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Left = 0,
    Right = 1,
    Up = 2,
    Down = 3
}

impl Direction {
    fn turn_left(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up
        }
    }
}

type Hull = HashMap<(i64, i64), Vec<i64>>;

fn paint_at(hull: &mut Hull, loc: (i64, i64), paint: i64) -> () {
    hull
        .entry(loc)
        .or_insert(Vec::new())
        .push(paint);
}

fn execute(program: &Program, mut starting_panel: Option<i64>) -> (Hull, (i64, i64), Direction) {
    let mut hull: Hull = HashMap::new();
    let (mut bot_x, mut bot_y) = (0, 0);
    let mut bot_direction = Direction::Up;
    let mut bot = Node {
        label: 'B',

        state: State::Boot,
        program: program.to_vec(),
        ip: 0,
        rb: 0,

        input: VecDeque::new(),
        output: VecDeque::new(),
    };

    while bot.state != State::Halted {
        // run bot until either input needed or output given
        debug!("BEGIN RUN: {:?}", bot);
        run(&mut bot);
        debug!("END RUN: {:?}", bot);

        match bot.state {
            State::InputWait => {
                bot.input.push_back(
                    starting_panel.take().unwrap_or(
                        *(
                            hull
                                .entry((bot_x, bot_y))
                                .or_insert(Vec::new())
                                .last()
                                .unwrap_or(&0)
                         )
                    )
                );
            },
            State::OutputYield => {
                if bot.output.len() == 2 {
                    let paint = bot.output.pop_front().unwrap();
                    let turn = bot.output.pop_front().unwrap();
                    paint_at(&mut hull, (bot_x, bot_y), paint);
                    match turn {
                        0 => bot_direction = bot_direction.turn_left(),
                        1 => bot_direction = bot_direction.turn_right(),
                        _ => panic!("Unknown turn direction {}", turn)
                    };
                    match bot_direction {
                        Direction::Up => bot_y -= 1,
                        Direction::Right => bot_x += 1,
                        Direction::Down => bot_y += 1,
                        Direction::Left => bot_x -= 1
                    };
                };
            },
            _ => ()
        };
    }

    (hull, (bot_x, bot_y), bot_direction)
}

fn visualize(hull: &Hull, bot_loc: (i64, i64), bot_dir: Direction) -> () {
    let (bot_x, bot_y) = bot_loc;

    let (min_x, _) = *hull
        .keys()
        .min_by(|(x1, _), (x2, _)| x1.cmp(x2))
        .expect("No min x!");

    let (_, min_y) = *hull
        .keys()
        .min_by(|(_, y1), (_, y2)| y1.cmp(y2))
        .expect("No min y!");

    let (max_x, _) = *hull
        .keys()
        .max_by(|(x1, _), (x2, _)| x1.cmp(x2))
        .expect("No max x!");

    let (_, max_y) = *hull
        .keys()
        .max_by(|(_, y1), (_, y2)| y1.cmp(y2))
        .expect("No max y!");

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if x == bot_x && y == bot_y {
                match bot_dir {
                    Direction::Up => print!("⬆️"),
                    Direction::Right => print!("➡️"),
                    Direction::Down => print!("⬅️"),
                    Direction::Left => print!("⬇️")
                };
            } else {
                let paint = match hull.get(&(x, y)) {
                    Some(instructions) => *instructions.last().expect("Expected paint but found none"),
                    None => 0
                };
                match paint {
                    0 => print!("⬛"),
                    1 => print!("⬜"),
                    _ => panic!("Unknown paint {}", paint)
                };
            };
        }
        println!("");
    };
}


fn part1(program: &Program) -> () {
    let (hull, _, _) = execute(program, None);
    println!("Part 1: {}", hull.keys().count());
}

fn part2(program: &Program) -> () {
    let (hull, bot_loc, bot_dir) = execute(program, Some(1));
    println!("Part 2:");
    visualize(&hull, bot_loc, bot_dir);
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
