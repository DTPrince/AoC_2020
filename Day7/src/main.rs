/* Derek Prince
 * Advent of Code day 7
 */

extern crate regex;

use std::fs::File;
use std::io::{Error, BufReader, BufRead};
use std::collections::HashMap;
use regex::Regex;

//to be used as data for a hash map.
// key will be string value bag color
#[derive(Clone, Default)]
struct BagRules {
    number_of_bags: Vec<usize>,
    rules : Vec<String>,
}

impl BagRules {
    fn num_rules(&self) -> usize {
        self.number_of_bags.len()
    }

    fn num_bags(&self) -> usize {
        let mut bags = 0;
        for i in self.number_of_bags.iter() {
            bags += i;
        }
        bags
    }

    fn remain_bags(&self) -> usize {
        let mut bags = 0;
        for (i, num) in self.number_of_bags.iter().enumerate() {
            let test = self.rules[i].clone();
            println!("test: {}", test);
            if test == "shiny gold".to_string() {
                println!("contains {} shiny gold bags", *num);
            } else {
                bags += num;
            }
        }
        bags
    }
    fn contain_gold(&self) -> bool {
        for bag in &self.rules {
            if bag.clone() == "shiny gold".to_string(){
                return true
            }
        }
        false
    }

    fn contains_none(&self) -> bool {
        if self.rules[0] == "no other".to_string() {
            return true
        }
        false
    }
}

fn read_input() -> Result<Vec<String>, Error> {
    let path = "./input/input.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let lines = buffered.lines().map(|l| l.unwrap()).collect();
    Ok(lines)
}

fn make_hmap(fcontents: Vec<String>) -> HashMap<String, BagRules> {
    let mut bag_map : HashMap<String, BagRules> = HashMap::new();
    for line in fcontents {
        // split on " contain(s) "
        let split_regex = Regex::new(r"\scontains?\s").unwrap();
        let mut key_rules_split = split_regex.split(&*line).into_iter();
        // let mut it = key_rules_split.into_iter();
        let mut key = key_rules_split.next().unwrap().to_string();
        let rule_string = key_rules_split.next().unwrap().to_string();
        key = key.replace(" bags", "");
        let rules : Vec<String> = rule_string.split(", ")
            .map(|s| s.to_string()).collect();

        let mut bag_rules : BagRules = BagRules::default();
        // captures (first digits)?, (bag color) leaves bags.
        let re= Regex::new(r"([\d]+)?\s?([\s\D]+)\sbags?\.?").unwrap();
        for rule in rules {
            let captures = re.captures(&*rule).unwrap();
            // first index 0 is full string, 1 is digit (if exists), 2 is color
            let brules = captures.get(2).map_or("", |r| r.as_str().trim());
            bag_rules.rules.push(brules.to_owned());
            let num = captures.get(1).map_or("0", |n| n.as_str().trim());   // I dont think trim is actually needed here
            bag_rules.number_of_bags.push(num.parse::<usize>().unwrap());
        }
        bag_map.insert(key, bag_rules);
    }
    bag_map
}

// //the recursive one
// fn explore_bags(bag: BagRules, bmap: &HashMap<String, BagRules>) -> (usize, usize) {
//     let mut gold_bag_sum : usize = 0;
//     let mut num_possible: usize = 0;
//     let mut remaining_bags = bag.num_bags();
//     while remaining_bags != 0 {
//         for (i, rule) in bag.rules.iter().enumerate() {
//             if rule.contains("shiny gold") {
//                 num_possible += 1;
//                 gold_bag_sum += bag.number_of_bags[i];
//                 remaining_bags -= bag.number_of_bags[i];
//             } else {
//                 let next = bmap.get(&*rule).unwrap();
//                 explore_bags(next.clone(), bmap);
//             }
//         }
//     }
//     (num_possible, gold_bag_sum)
// }
//the recursive one
fn explore_bags(bag: BagRules, bmap: &HashMap<String, BagRules>) -> usize {
    let mut num_possible: usize = 0;
    for (i, rule) in bag.rules.iter().enumerate() {
        if rule != &"no other".to_string(){
            let next = bmap.get(&*rule).unwrap();
            num_possible += explore_bags(next.clone(), bmap);
        }
    }
    if bag.contain_gold(){
        num_possible += 1;
    }
    println!("num poss: {}", num_possible);
    num_possible
}

fn num_bags(bmap: &HashMap<String, BagRules>) {
    let mut total_sum: usize = 0;
    let mut total_possible: usize = 0;
    for key in bmap.keys() {
        println!("Top key: {}", key);
        let bag = bmap.get(key).unwrap().clone();
        let possible = explore_bags(bag, &bmap.clone());
        total_possible += possible;
        // total_sum += sum;
    }
    println!("Total possible: {}", total_possible);
}

fn regex_tests() {
    let re = Regex::new(r"([\d]+)?\s?([\s\D]+)\sbags?\.?").unwrap();
    let test = re.captures(r"no matching bags.");
    if test.is_some() {
        let tst = test.unwrap();
        println!("tst.len(): {}", tst.len());
        println!("test: {}", tst.get(0).map_or("", |m| m.as_str()));
        println!("test: {}", tst.get(1).map_or("", |m| m.as_str()));
        println!("test: {}", tst.get(2).map_or("", |m| m.as_str()));
    } else {
        println!("Nothing to unwrap");
    }
}


fn main() -> Result<(), Error> {
    let fcontents = read_input()?;
    let bmap = make_hmap(fcontents);
    num_bags(&bmap);

    // regex_tests();
    Ok(())
}
