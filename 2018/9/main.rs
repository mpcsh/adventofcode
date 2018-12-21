use std::env;
use std::cell::RefCell;
use std::rc::Rc;
use std::borrow::BorrowMut;

struct CirclePoint {
    marble: u64,
    clockwise: Rc<RefCell<CirclePoint>>,
    counterclockwise: Rc<RefCell<CirclePoint>>
}

fn insert_clockwise_of(current_marble: Rc<RefCell<CirclePoint>>, new_marble: u64) {
    // set up new marble
    let new_marble_point = Rc::new(RefCell::new(CirclePoint {
        marble: new_marble,
        clockwise: current_marble.borrow().clockwise.clone(),
        counterclockwise: current_marble.clone()
    }));

    // change adjacent references
    current_marble.borrow().clockwise.borrow().borrow_mut().counterclockwise = new_marble_point.clone();
    current_marble.borrow().clockwise = new_marble_point.clone();
}

fn place_marble(current_marble: Rc<RefCell<CirclePoint>>, new_marble: u64)
                -> (u64, Rc<RefCell<CirclePoint>>) {
    if new_marble % 23 == 0 {
        (0, current_marble)
    } else {
        (0, current_marble)
    }
}

fn part_1(num_players: usize, num_marbles: usize) -> u64 {

    0
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    let num_players = args[1].parse::<usize>().unwrap();
    let num_marbles = args[2].parse::<usize>().unwrap();

    println!("Part 1: highest score = {}", part_1(num_players, num_marbles));

    Ok(())
}
