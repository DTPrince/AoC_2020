/* Derek Prince
 * Advent of Code Day 5
 * Recursive Binary Ranging
 * There will be a very easy way to do this
 * lettuce see if it is obvious
 */

use std::io::{Error, BufReader, BufRead};
use std::fs::File;

#[derive(Default, Clone)]
struct BoardingPass {
    string_location : String,
    row : i32,
    seat : i32,
}

impl BoardingPass {
    fn get_seat_number(&self) -> i32 {
        (self.row * 8) + self.seat
    }
}

fn read_input() -> Result<Vec<BoardingPass>, Error> {
    let path = "./input/input.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let lines = buffered.lines().map(|l| l.unwrap());
    let mut items : Vec<BoardingPass> = Vec::new();
    for line in lines {
        items.push(BoardingPass{string_location: line, row: -1, seat: -1 });  // we lose int parse errors here but whatever
    }
    Ok(items)
}

fn find_row(bp: &BoardingPass) -> i32 {
    let row_string: &str = &bp.string_location.as_str()[..7];
    // println!("First 7 chars: {}", row_string);
    let mut upper : i32 = 127;
    let mut lower : i32 = 0;
    for c in row_string.chars() {
        match c {
            'F' => { upper = (upper + lower) / 2; },        // this does indeed equal 63 from 127
            'B' => { lower = (upper + lower + 1) / 2; },    // forces round up for lower bound
            _ => { println!("No match found for row. c : {}", c) }
        }
    }
    return match row_string.chars().last().unwrap() {
        'F' => lower,
        'B' => upper,
        _ => -1
    }
}

fn find_seat(bp: &BoardingPass) -> i32 {
    let seat_string: &str = &bp.string_location.as_str()[7..];
    // println!("Last 3 chars: {}", seat_string);
    let mut upper : i32 = 7;
    let mut lower : i32 = 0;
    for c in seat_string.chars() {
        match c {
            'L' => { upper = (upper + lower) / 2; },
            'R' => { lower = (upper + lower + 1) / 2; },    //+1 forces round up for lower bound
            _ => { println!("No match found for seat. c : {}", c) }
        }
    }
    return match seat_string.chars().last().unwrap() {
        'L' => lower,
        'R' => upper,
        _ => -1
    }
}

fn find_seats(bps : &mut Vec<BoardingPass>) {
    for bpass in &mut bps.iter_mut() {
        bpass.row = find_row(&bpass);
        bpass.seat = find_seat(&bpass);
    }
}

fn find_highest(bps : &Vec<BoardingPass>) -> BoardingPass {
    let mut highest : BoardingPass = bps.first().unwrap().clone();

    for bp in bps {
        if bp.get_seat_number() > highest.get_seat_number() {
            highest = bp.clone();   // clone and be done with it
        }
    }
    highest
}

fn find_my_seat(bps : &Vec<BoardingPass>) -> i32{
    let mut seats: Vec<i32> = Vec::new();
    for bp in bps { //just setting up because I would rather simply call .contains()
        seats.push(bp.get_seat_number());
    }
    let max = seats.iter().max().unwrap().clone();
    let min = seats.iter().min().unwrap().clone();
    for seat in min..max {
        if seats.contains(&(seat + 1)) && seats.contains(&(seat - 1)) && !seats.contains(&seat) {
            return seat
        }
    }
    0
}

//ex: BFFFBBFRRR: row 70, column 7, seat ID 567.
//                              70*8 + 7 = 567
// Physical representation, not counted from 0
fn main() -> Result<(), Error>{
    let mut bpasses = read_input()?;
    find_seats(&mut bpasses);
    let high = find_highest(&bpasses);
    println!("Highest seat found: {}", high.get_seat_number());
    let myseat = find_my_seat(&bpasses);
    println!("My seat: {}", myseat);

    Ok(())
}

