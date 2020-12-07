/* Derek Prince
 * Advent of Code day 6
 */

use std::fs::File;
use std::io::{Error, BufReader, BufRead};

#[derive(Clone, Default)]
struct Group {
    size: usize,
    responses: String,
    reduced_responses: String,
}

impl Group {
    fn valid_responses(&self) -> usize {
        let mut sum: usize = 0;
        for c in self.reduced_responses.chars() {
            let matches = self.responses.matches(c).count();
            if matches == self.size {
                sum += 1;
            }
        }
        sum
    }
}

fn read_input() -> Result<Vec<String>, Error> {
    let path = "./input/input.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let lines = buffered.lines().map(|l| l.unwrap()).collect();
    Ok(lines)
}

// Part two needs basically all of the information I removed to do part 1.
// Sooooooooooooooooooo I'm just going to make another fn to take care of it.
fn clean_input(mut fcontents: Vec<String>) -> Vec<Group> {
    let mut sorted_groups: Vec<Group> = Vec::new();
    for line in fcontents.iter_mut() {
        if line.is_empty() {
            line.push('%'); // new group indicator
        }
    }

    let s: String = fcontents.join(" ");
    let groups: Vec<String> = s.split("%").map(|s| s.to_string()).collect();
    for group in groups {
        let group_responses : Vec<&str> = group.trim().split(" ").collect();
        let group_size = group_responses.len();
        let responses: String = group_responses.join("");

        // resort for easy counting
        let mut sort: Vec<char> = responses.chars().collect();
        sort.sort_by(|a, b| a.cmp(b));


        let sorted_response = sort.iter().collect();
        // now remove dupes and consume
        sort.dedup();
        let reduced_responses = sort.into_iter().collect();

        let group = Group{ size: group_size, responses: sorted_response, reduced_responses };
        sorted_groups.push(group);
    }
    sorted_groups
}

fn main() -> Result<(), Error> {
    let fcontents = read_input()?;
    let cleaned_groups = clean_input(fcontents);

    let mut sum: usize = 0;
    for group in cleaned_groups.iter() {
        sum += group.valid_responses();
    }
    println!("Sum of questions answered in all groups: {}", sum);
    Ok(())
}
