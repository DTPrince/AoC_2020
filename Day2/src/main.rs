/* Derek Prince
 * Advent of Code
 * Day 2
 */

use std::fs::File;
use std::io::{Error, BufReader, BufRead};

fn read_file() -> Result<Vec<String>, Error> {
    let path = "./input/input.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let lines = buffered.lines().map(|l| l.unwrap());
    let mut items= Vec::new();
    for line in lines {
        items.push(line);
    }
    Ok(items)
}

// Just keeping this here as a record. Was just run in main() before
// fn part_one(lineitem: &Vec<String>) {
//     let mut valid_password_count = 0;
//     for line in lineitem {
//         // println!("line: {}", line);
//         let l: Vec<_> = line.split(':').collect();
//         // println!("l: {}, sz: {}", l[0], l.len());
//         if l.len() == 2 {   // this can be assumed since I know my input is clean. But still.
//             let range_key: Vec<_> = l[0].split(|c| c == '-' || c == ' ').collect();
//             let lower = range_key[0].parse::<usize>().unwrap();
//             let upper = range_key[1].parse::<usize>().unwrap();
//             let key = range_key[2];
//             let pw = l[1];
//             // println!("match count: {}",pw.matches(key).count());
//             if (lower <= pw.matches(key).count()) && (pw.matches(key).count() <= upper) {
//                 valid_password_count += 1;
//             }
//         }
//     }
// }

fn main() {
    let lineitem = read_file().unwrap();

    let mut valid_password_count = 0;
    for line in lineitem {
        // println!("line: {}", line);
        let l: Vec<_> = line.split(':').collect();
        // println!("l: {}, sz: {}", l[0], l.len());
        if l.len() == 2 {   // this can be assumed since I know my input is clean. But still.
            let range_key: Vec<_> = l[0].split(|c| c == '-' || c == ' ').collect();
            let first = range_key[0].parse::<usize>().unwrap();
            let second = range_key[1].parse::<usize>().unwrap();
            let key = range_key[2];
            let pw = l[1];
            // .find() will only find the offset to the fisrt one
            // just going to check the direct spot instead
            let a = pw.as_bytes()[first];
            let b = pw.as_bytes()[second];
            if (a == key.as_bytes()[0]) ^ (b == key.as_bytes()[0]) {
                valid_password_count += 1;
            }
        }
    }
    println!("No. valid pws: {}", valid_password_count);
}
