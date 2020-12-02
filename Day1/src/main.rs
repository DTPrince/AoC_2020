/* Derek Prince
 * Advent of Code
 * Day 1
 */

use std::fs::File;
use std::io::{Error, BufReader, BufRead};

fn read_input() -> Result<Vec<i32>, Error> {
    let path = "./input/input.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let lines = buffered.lines().map(|l| l.unwrap());
    let mut items : Vec<i32> = Vec::new();
    for line in lines {
        items.push(line.parse::<i32>().unwrap());   // we lose int parse errors here but whatever
    }
    Ok(items)
}

fn main() {
    let values = read_input().unwrap(); // just assume it worked for the purpose of this sketch

    // find two numbers that sum to 2020
    for value in &values {
        let x = value;
        let y = values.iter().find(|&&check| check == (2020 - x));
        if y.is_some() {
            println!("x: {}, y: {}, x*y: {}", x, y.unwrap(), x*y.unwrap());
        }
    }
    // find three numbers that sum to 2020
    for value1 in &values {
        let x = value1;
        for value2 in &values {
            let y = value2;
            let z = values.iter().find(|&&check| check == (2020 - x - y));
            if z.is_some() {
                println!("x: {}, y: {}, z: {}, x*y*z: {}", x, y, z.unwrap(), x*y*z.unwrap());
            }
        }
    }
}
