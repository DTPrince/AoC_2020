/* Derek Prince
 * Advent of Code day 8
 * The processor one
 */

extern crate regex;

use std::fs::File;
use std::io::{Error, BufReader, BufRead};
use regex::Regex;
use std::borrow::Borrow;

#[derive(Clone, Default, Debug)]
struct Instruction {
    // Keeping as strings to avoid ownership issues with &str
    op: String,
    sign: String,
    num: usize,
}

fn read_input() -> Result<Vec<String>, Error> {
    let path = "./input/input.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let lines = buffered.lines().map(|l| l.unwrap()).collect();
    Ok(lines)
}

fn clean_input (fcontents: Vec<String>) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();
    // capture first 3 letters (opcode), sign, up to 3 digits
    let re = Regex::new(r"([a-z]{3})\s(.)(\d{0,3})").unwrap();
    for line in fcontents {
        let captures = re.captures(&*line).unwrap();
        let mut instruction = Instruction::default();
        // 0 is whole string, 1 is opcode, 2 is sign, 3 is number
        instruction.op = captures.get(1)
            .map_or("err".to_string(), |op| op.as_str().trim().to_string());
        instruction.sign = captures.get(2)
            .map_or("".to_string(), |op| op.as_str().trim().to_string());
        instruction.num = captures.get(3)
            .map_or(0, |op| op.as_str().trim().parse::<usize>().unwrap());
        instructions.push(instruction);
    }
    instructions
}

fn run_program(instructions: &Vec<Instruction>) -> bool {
    let mut acc : usize = 0;
    let mut i = 0;
    let mut visited : Vec<usize> = Vec::new();
    let mut looped = false;
    while !visited.contains(&i) ^ (i >= instructions.len()) {
        visited.push(i);
        match instructions[i].op.as_str() {
            "acc" => {
                if instructions[i].sign.as_str() == "+" {
                   acc += instructions[i].num;
                } else if instructions[i].sign.as_str() == "-" {
                    acc -= instructions[i].num;
                }

                i += 1;
            }
            "jmp" => {
                if instructions[i].sign.as_str() == "+" {
                    i += instructions[i].num;
                } else if instructions[i].sign.as_str() == "-" {
                    i -= instructions[i].num;
                }
            }
            "nop" => {
                i += 1;
            }
            _ => { println!("No match found in instruction: {}", instructions[i].op)}
        }
        looped = visited.contains(&i)
    }
    println!("Acc reached {}", acc);
    // returns true if the program looped
    looped
}

fn find_instruction(instructions: &mut Vec<Instruction>) {
    for i in 0..instructions.len() {
        match instructions[i].op.as_str() {
            "nop" => {
                instructions[i].op = "jmp".to_string();
                if !run_program(instructions) {
                    break;
                } else {
                    //revert
                    instructions[i].op = "nop".to_string();
                }
            }
            "jmp" => {
                instructions[i].op = "nop".to_string();
                if !run_program(instructions) {
                    break;
                } else {
                    //revert
                    instructions[i].op = "jmp".to_string();
                }
            }
            _ => {}
        }
    }
}

fn main() -> Result<(), Error> {
    let fcontents = read_input()?;
    let mut instructions = clean_input(fcontents);
    // let _test = run_program(instructions);
    find_instruction(&mut instructions);
    Ok(())
}
