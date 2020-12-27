/* Derek Prince
 * Advent of Code Day 11
 * Seats, please
 */

extern crate petgraph;

use std::fs::File;
use std::io::{Error, BufReader, BufRead};
use std::collections::HashMap;
use petgraph::graphmap::{UnGraphMap};

const ROW_WIDTH: usize = 95;   // change as needed for test cases and what-have-you


#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum State { // L, #, .
    Empty,
    Occupied,
    Floor   //maybe not even this
}

impl Default for State {
    fn default() -> Self { State::Empty }
}

// Scary derives
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Position {
    id: usize,
    seat: State,
    row: usize,
    column: usize,
}

impl Position {
    fn as_id(row: usize, column: usize) -> usize {
        (row*ROW_WIDTH) + column
    }
}

fn read_input() -> Result<Vec<String>, Error> {
    let path = "./input/input.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let lines: Vec<String> = buffered.lines().map(|l| l.unwrap()).collect();
    // println!("len: {}", lines.first().unwrap().len());   // checking len for wrap conditions
    Ok(lines)
}

fn create_map(fcontents: Vec<String>) -> HashMap<usize, Position> {
    // The fact that I can only store numbers as nodes is beyond infuriating.
    // Let me reference m'fuckin' structs with an ID and associate edges.
    let mut map: HashMap<usize, Position> = HashMap::new();
    let mut neighbors = UnGraphMap::<usize, ()>::new();

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
            seat.row = num;
            seat.column = offset;
            seat.id = (num*ROW_WIDTH)+offset;
            map.insert(seat.id, seat);
            neighbors.add_node(seat.id);
        }
    }
    // gmap.contains_node(96);
    //associate edges
    for seat in 0..map.len() {
        let center = map.get(&seat).unwrap();

        let adjacents = vec![
            Position::as_id(center.row - 1, center.column - 1), Position::as_id(center.row - 1, center.column), Position::as_id(center.row - 1, center.column + 1),
            Position::as_id(center.row, center.column - 1), Position::as_id(center.row, center.column), Position::as_id(center.row, center.column + 1),
            Position::as_id(center.row + 1, center.column - 1), Position::as_id(center.row + 1, center.column), Position::as_id(center.row + 1, center.column + 1)];


    }
    map
}

fn seat_people(seat_map : &mut HashMap<usize, Position>) -> bool {
    let seating_changed = false;
    let map_len = seat_map.len();
    for seat in 0..map_len {
        let current = seat_map.get_mut(&seat).unwrap();
        match current.seat {
            State::Empty => {

            }
            State::Occupied => {}
            State::Floor => {}
        }
    }
    // for (key, val) in seat_map.iter_mut() {
    //     println!("Key: {}, Val: R{}C{}", key, val.row, val.column);
    // }
    seating_changed
}

fn main() -> Result<(), Error> {
    let fcontents = read_input()?;
    let mut seat_map : HashMap<usize, Position> = create_map(fcontents);
    while seat_people(&mut seat_map) {}

    Ok(())
}
