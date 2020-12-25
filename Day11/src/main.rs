/* Derek Prince
 * Advent of Code Day 11
 * Seats, please
 */

use std::fs::File;
use std::io::{Error, BufReader, BufRead};
use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
enum State { // L, #, .
    Empty,
    Occupied,
    Floor   //maybe not even this
}

impl Default for State {
    fn default() -> Self { State::Empty }
}

#[derive(Copy, Clone, Default, Debug)]
struct Position {
    seat: State,
    id: usize,
}

// impl Position {
//
// }

fn read_input() -> Result<Vec<String>, Error> {
    let path = "./input/input.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let lines = buffered.lines().map(|l| l.unwrap()).collect();
    Ok(lines)
}

fn create_map(fcontents: Vec<String>) -> HashMap<usize, Position> {
    let mut map: HashMap<usize, Position> = HashMap::new();

    for (num, line) in fcontents.iter().enumerate() {
        let pos = line.as_str();
        for (offset, p) in pos.chars().enumerate() {
            let mut seat = Position::default();
            match p {
                'L' => seat.seat = State::Empty,
                '#' => seat.seat = State::Occupied,
                '.' => seat.seat = State::Floor,
                _ => println!("Somehow got a miss-matched state. Consider cleaning your input."),
            }
            map.insert(num+offset, seat);
        }
    }
    map
}

fn seat_people(mut seat_map : &HashMap<usize, Position>) -> bool {
    let seating_changed = true;
    


    seating_changed
}

fn main() -> Result<(), Error> {
    let fcontents = read_input()?;
    let mut seat_map : HashMap<usize, Position> = create_map(fcontents);
    while seat_people(&seat_map) {}

    Ok(())
}
