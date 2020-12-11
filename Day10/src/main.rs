/* Derek Prince
 * Advent of Code Day 10
 * jolts in yer jorts
 */

// theft because I spent a long time messing around with graphs and
// although it was great it never got me anywhere loooooooool.
// So I imported someone else's part 2 to learn how they did some things as it was very similar
// https://www.reddit.com/r/adventofcode/comments/ka8z8x/2020_day_10_solutions
extern crate petgraph;

use std::fs::File;
use std::io::{Error, BufReader, BufRead};
use petgraph::{graph};
use petgraph::graphmap::{DiGraphMap};
use petgraph::algo::{astar};
use std::convert::TryInto;
use std::collections::HashMap;


#[derive(Copy, Clone, Default, Debug)]
struct Node {
    val: usize,
    weight: usize,
}

fn read_input() -> Result<Vec<usize>, Error> {
    let path = "./input/input.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let lines = buffered.lines().map(|l| l.unwrap().parse::<usize>().unwrap()).collect();
    Ok(lines)
}

fn clean_contents(mut fcontents: Vec<usize>) -> Vec<Node> {
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

fn create_graph(mut fcontents: Vec<usize>) -> DiGraphMap<usize, usize> {
    let mut adapters : DiGraphMap<usize, usize> = DiGraphMap::new();
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

fn part_2(jolts: Vec<usize>) -> usize {
    let adapter_connection_option_counts: Vec<_> = jolts
        .iter()
        .enumerate()
        .map(|(index, value)| {
            (1..=3)
                .map(|num| jolts.get(index + num).map(|other| other - value))
                .filter(|&diff| match diff {
                    Some(diff) => diff <= 3,
                    None => false,
                })
                .count()
        })
        .collect();

    let next_adapter_is_skippable_list: Vec<bool> = adapter_connection_option_counts
        .iter()
        .map(|&option_count| option_count > 1)
        .collect();

    let single_count = next_adapter_is_skippable_list
        .iter()
        .enumerate()
        .filter(|(index, &is_skippable)| {
            is_skippable == true
                && (index == &0 || next_adapter_is_skippable_list.get(index - 1) != Some(&true))
                && next_adapter_is_skippable_list.get(index + 1) != Some(&true)
                && next_adapter_is_skippable_list.get(index + 2) != Some(&true)
        })
        .count();

    let two_consecutive_count = next_adapter_is_skippable_list
        .iter()
        .enumerate()
        .filter(|(index, &is_skippable)| {
            is_skippable == true
                && (index == &0 || next_adapter_is_skippable_list.get(index - 1) != Some(&true))
                && next_adapter_is_skippable_list.get(index + 1) == Some(&true)
                && next_adapter_is_skippable_list.get(index + 2) != Some(&true)
        })
        .count();

    let three_consecutive_count = next_adapter_is_skippable_list
        .iter()
        .enumerate()
        .filter(|(index, &is_skippable)| {
            is_skippable == true
                && (index == &0 || next_adapter_is_skippable_list.get(index - 1) != Some(&true))
                && next_adapter_is_skippable_list.get(index + 1) == Some(&true)
                && next_adapter_is_skippable_list.get(index + 2) == Some(&true)
        })
        .count();

    // There are 2 ways to connect a single skippable adapter,
    //
    // 1: 5, 6, 7
    // 2: 5,  , 7
    //
    2usize.pow(single_count.try_into().unwrap())
        //
        // There are 4 ways to connect two consecutive skippable adapters,
        //
        // 1: 5, 6, 7, 8
        // 2: 5, 6,  , 8
        // 3: 5,  , 7, 8
        // 4: 5,  ,  , 8
        //
        * 4usize.pow(two_consecutive_count.try_into().unwrap())
        //
        // There are 7 ways to connect three consecutive skippable adapters,
        //
        // 1: 5, 6, 7, 8, 9
        // 2: 5, 6,  , 8, 9
        // 3: 5,  , 7, 8, 9
        // 4: 5, 6, 7,  , 9
        // 5: 5,  ,  , 8, 9
        // 6: 5, 6,  ,  , 9
        // 7: 5,  , 7,  , 9
        //
        * 7usize.pow(three_consecutive_count.try_into().unwrap())
}

fn main() -> Result<(), Error> {
    let mut fcontents = read_input()?;
    // let max = *fcontents.clone().iter().max().unwrap();
    let adapters = clean_contents(fcontents.clone());
    part1(adapters);
    fcontents.push(0);
    fcontents.sort();
    fcontents.push(fcontents.clone().iter().max().unwrap() + 3);
    println!("Part 2: {}", part_2(fcontents.clone()));
    Ok(())
}

// just checking someone's very rust solution
fn part1_notme(adaptors: &[u64]) -> Result<u64, E> {
    let [_, ones, _, threes] =
        adaptors
            .windows(2)
            .map(|pair| pair[1] - pair[0])
            .fold([1; 4], |mut counts, it| {
                counts[it as usize] += 1;
                counts
            });
    Ok(ones * threes)
}

fn part2_search(adaptors: &[u64], db: &mut HashMap<u64, u64>) -> u64 {
    match adaptors.split_first() {
        Some((_, [])) => 1,
        Some((first, rest)) => rest
            .iter()
            .take_while(|a| *a - first <= 3)
            .enumerate()
            .map(|(idx, val)| {
                db.get(val).copied().unwrap_or_else(|| {
                    let sub_count = part2_search(&rest[idx..], db);
                    *db.entry(*val).or_insert(sub_count)
                })
            })
            .sum(),
        None => 0, // Shouldn't get an empty list, but just in case...
    }
}