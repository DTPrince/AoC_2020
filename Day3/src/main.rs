/* Derek Prince
 * Advent of Code Day 3
 * Bonkin the noggin on the toboggan
*/

use std::fs::File;
use std::io::{Error, BufReader, BufRead};

#[derive(Clone, Default)]
struct ForrestMap {
    x: usize,
    y: usize,
    field: Vec<String>,
    no_trees: usize,
}

impl ForrestMap {
    fn traverse(&mut self, x_inc: usize, y_inc: usize) -> usize {
        self.no_trees = 0;  // for repeat calls
        self.y = 0;
        self.x = 0;
        let height = self.field.len();
        let mod_width = self.field[0].len();
        let tree_token = "#".as_bytes()[0];
        while self.y < height - 1 {
            self.x += x_inc;
            // adjust for overflow
            if self.x >= mod_width {
                self.x = self.x % mod_width;
            }
            self.y += y_inc;
            let intersect_token = self.field[self.y].as_bytes()[self.x];
            if intersect_token == tree_token {
                self.no_trees += 1;
            }
        }
        self.no_trees
    }
}

fn read_input() -> Result<Vec<String>, Error> {
    let path = "./input/input.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    let lines: Vec<String> = buffered.lines().map(|l| l.unwrap()).collect();
    // for line in lines.iter() {
    //     println!("{}", line);
    // }
    Ok(lines)
}

fn main() -> Result<(), Error> {
    let forrest = read_input()?;
    // println!("len: {}, line width: {}", forrest.len(), forrest[0].len());

    let mut fmap = ForrestMap { x: 0, y: 0, no_trees: 0, field: forrest };
    let traverse_1_1 = fmap.traverse(1, 1);
    println!("No. trees hit R1D1: {}", traverse_1_1);
    let traverse_3_1 = fmap.traverse(3, 1);
    println!("No. trees hit R3D1: {}", traverse_3_1);
    let traverse_5_1 = fmap.traverse(5, 1);
    println!("No. trees hit R5D1: {}", traverse_5_1);
    let traverse_7_1 = fmap.traverse(7, 1);
    println!("No. trees hit R7D1: {}", traverse_7_1);
    let traverse_1_2 = fmap.traverse(1, 2);
    println!("No. trees hit R1D2: {}", traverse_1_2);

    println!("Monstrous multiplication: {}",
             traverse_1_1 * traverse_3_1 * traverse_5_1 * traverse_7_1 * traverse_1_2);
    Ok(())
}
