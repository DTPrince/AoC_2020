/* Derek Prince
 * Advent of Code Day 10
 * jolts in yer jorts
 */
extern crate petgraph;

use std::fs::File;
use std::io::{Error, BufReader, BufRead};
use petgraph::{graph};
use petgraph::graphmap::{DiGraphMap};
use petgraph::algo::{astar};

#[derive(Copy, Clone, Default, Debug)]
struct Node {
    val: u32,
    weight: u32,
}

fn read_input() -> Result<Vec<u32>, Error> {
    let path = "./input/input.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let lines = buffered.lines().map(|l| l.unwrap().parse::<u32>().unwrap()).collect();
    Ok(lines)
}

fn clean_contents(mut fcontents: Vec<u32>) -> Vec<Node> {
    let mut adapters: Vec<Node> = Vec::new();
    fcontents.push(0);    // outlet
    fcontents.sort();
    fcontents.push(fcontents.last().unwrap() + 3);  // internal adapter

    // bring 2 elements into scope per iter

    for pair in fcontents.as_slice().windows(2) {
        let n: Node = Node { val: pair[0], weight: (pair[1] - pair[0]) };
        adapters.push(n);
    }
    // just do the last one here.
    let n: Node = Node{ val: *fcontents.last().unwrap(), weight: 0 };
    adapters.push(n);

    adapters
}

fn part1(adapters: Vec<Node>) {
    let mut ones = 0;
    let mut threes = 0;
    for adapter in adapters {
        match adapter.weight {
            1 => ones += 1,
            3 => threes += 1,
            _ => println!("No match, {}", adapter.weight),
        }
    }
    println!("Found {} 1s and {} 3s, p1: {}", ones, threes, ones*threes);
}

fn create_graph(mut fcontents: Vec<u32>) -> DiGraphMap<u32, u32> {
    let mut adapters : DiGraphMap<u32, u32> = DiGraphMap::new();
    fcontents.push(0);  // outlet
    fcontents.sort_by(|a, b| b.cmp(a));
    fcontents.push(fcontents.first().unwrap() + 3);  // self adapter

    for c in fcontents.clone() {
        adapters.add_node(c);
        // weights are reversed because we want the shortest path to be the fewest nodes (duh)
        if adapters.contains_node(c + 1){
            adapters.add_edge(c, c + 1, 3);
        }
        if adapters.contains_node(c + 2){
            adapters.add_edge(c, c + 2, 2);
        }
        if adapters.contains_node(c + 3){
            adapters.add_edge(c, c + 3, 1);
        }
    }
    adapters
}

fn main() -> Result<(), Error> {
    let fcontents = read_input()?;
    let max = *fcontents.clone().iter().max().unwrap();
    let adapters = clean_contents(fcontents.clone());
    part1(adapters);
    let g_adapters = create_graph(fcontents);
    let result = astar(g_adapters, 0, max, |e| *e.weight(), |_| 0).unwrap();
    Ok(())
}

/*
3 choose 1 + 3 choose 2 + 3 choose 3
5, 6, 11,
(0), 1, 4, 7, 10, 12, 15, 16, 19, (22)

 */