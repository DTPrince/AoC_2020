/* Derek Prince
 * Advent of Code Day 9
 * Sliding pointer frame sums
 */

use std::fs::File;
use std::io::{Error, BufReader, BufRead};

// let lower_frame
// let upper_frame
// let frame_size

// At least this one can just be Vec<u64> from the get-go
fn read_input() -> Result<Vec<u64>, Error> {
    let path = "./input/input.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let lines = buffered.lines().map(|l| l.unwrap().parse::<u64>().unwrap()).collect();
    Ok(lines)
}

fn can_make(lower: usize, upper: usize, numbers: &Vec<u64>) -> bool {
    let target = numbers[upper+1];
    for i in lower..upper+1 {
        let first = numbers[i]; //only their own variable for debug
        for j in lower..upper+1 {
            if j != i {
                let second = numbers[j];
                if first + second == target {
                    return true
                }
            }
        }
    }
    false
}

fn find_problem(numbers: &Vec<u64>) -> usize {
    let frame_size : usize = 25;    // given by prompt
    let mut lower_frame : usize = 0;
    let mut upper_frame : usize = frame_size - 1;

    for i in (frame_size)..numbers.len() {
        if !can_make(lower_frame, upper_frame, &numbers){
            println!("Cannot make {} ({})", numbers[i], i);
            return  i;
        }
        lower_frame += 1;
        upper_frame += 1;
    }
    0
}

fn find_continuous_sum(index: usize, numbers: &Vec<u64>) {
    let target = numbers[index];
    'inner: for i in 0..numbers.len() {
        let mut sum: u64 = numbers[i];
        'outer: for j in i..numbers.len() {
            if j != i {
                sum += numbers[j];
                if sum > target {
                    break 'outer;
                }
                if sum == target {
                    let slice : Vec<u64> = numbers[i..j].to_vec();
                    println!("Indexes {},{} found. Sum:{}", i, j, slice.iter().min().unwrap() + slice.iter().max().unwrap())

                }
            }
        }
    }
}

fn main() -> Result<(), Error>{
    let fcontents = read_input()?;
    let index = find_problem(&fcontents);
    find_continuous_sum(index, &fcontents);
    Ok(())
}
