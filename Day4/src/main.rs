/* Derek Prince
 * Advent of Code Day 4
 * This is the string parse one
 */
use std::fs::File;
use std::io::{Error, BufReader, BufRead};
use std::ops::Index;

#[derive(Default, Clone)]
struct Passport {
    byr : String,
    iyr : String,
    eyr : String,
    hgt : String,
    hcl : String,
    ecl : String,
    pid : String,
    cid : String,
    no_required_fields : i32,
}

// impl Passport {
//     // pub fn default(&mut self) -> Passport {
//     //
//     // }
// }

fn read_input() -> Result<Vec<String>, Error> {
    let path = "./input/input.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    let lines: Vec<String> = buffered.lines().map(|l| l.unwrap()).collect();
    // for line in lines.iter() {
    //     let tmp = line.clone();
    //     println!("{}", &tmp);
    // }
    Ok(lines)
}

fn clean_input(fcontents: Vec<String>) -> Vec<Passport> {
    let mut pvec : Vec<Passport> = Vec::new();
    let mut delin : Vec<String> = Vec::new();
    // Purpose is to find the new lines that indicate new passport and ignore normal return lines
    for mut line in fcontents.clone() {
        if line.is_empty() {
            line.push('%'); // new entry indicator
        }
        delin.push(line);   // I hate this but... Borrows lol
    }

    let s = delin.join(" "); // convert to single string
    let itemized = s.split("%");
    // for i in itemized {
    //     println!("{}", i);
    // }
    for item in itemized {
        let mut passport = Passport::default();
        let split_container : Vec<&str> = item.trim().split(" ").collect();
        for sc in split_container {
            // println!("sc| {}",sc);
            let field_data: Vec<&str> = sc.split(":").collect();
            match field_data[0] {
                "byr" => {
                    if (field_data[1].parse::<i32>().unwrap() >= 1920) && (field_data[1].parse::<i32>().unwrap() <= 2002) {
                        passport.byr = field_data[1].to_string(); passport.no_required_fields += 1
                    }
                },
                "iyr" => {
                    if (field_data[1].parse::<i32>().unwrap() >= 2010) && (field_data[1].parse::<i32>().unwrap() <= 2020) {
                        passport.iyr = field_data[1].to_string(); passport.no_required_fields += 1
                    }
                },
                "eyr" => {
                    if (field_data[1].parse::<i32>().unwrap() >= 2020) && (field_data[1].parse::<i32>().unwrap() <= 2030) {
                        passport.eyr = field_data[1].to_string(); passport.no_required_fields += 1
                    }
                },
                "hgt" => {
                    let mut height = field_data[1].to_string();
                    if height.contains("cm") {
                        // find index of c and cut off remaining
                        let snum: String = height.drain(..height.find("c").unwrap()).collect();
                        let num = snum.parse::<i32>().unwrap();
                        if num >= 150 && num <= 193 {
                            passport.hgt = field_data[1].to_string();
                            passport.no_required_fields += 1;
                        }
                    } else if height.contains("in") {
                        // find index of i and cut off remaining
                        let snum: String = height.drain(..height.find("i").unwrap()).collect();
                        let num = snum.parse::<i32>().unwrap();
                        if num >= 59 && num <= 76 {
                            passport.hgt = field_data[1].to_string();
                            passport.no_required_fields += 1;
                        }
                    }
                },
                "hcl" => {
                    // wow I hate this but here we are
                    let hair_color = field_data[1].to_string();
                    let mut all_valid = false;
                    if hair_color.chars().nth(0).unwrap() == '#' && hair_color.len() == 7 {
                        all_valid = true;
                        for (i, c) in hair_color.chars().enumerate() {
                            if !((c >= '0' && c <= '9') || (c >= 'a' && c <= 'f')) && (i != 0) {
                                all_valid = false;
                            }
                        }
                        if all_valid {
                            passport.hcl = field_data[1].to_string();
                            passport.no_required_fields += 1;
                        }
                    }
                },
                "ecl" => {
                    match field_data[1] {
                        "amb" => { passport.ecl = field_data[1].to_string(); passport.no_required_fields += 1 },
                        "blu" => { passport.ecl = field_data[1].to_string(); passport.no_required_fields += 1 },
                        "brn" => { passport.ecl = field_data[1].to_string(); passport.no_required_fields += 1 },
                        "gry" => { passport.ecl = field_data[1].to_string(); passport.no_required_fields += 1 },
                        "grn" => { passport.ecl = field_data[1].to_string(); passport.no_required_fields += 1 },
                        "hzl" => { passport.ecl = field_data[1].to_string(); passport.no_required_fields += 1 },
                        "oth" => {passport.ecl = field_data[1].to_string(); passport.no_required_fields += 1 },
                        _ => { }
                    }
                },
                "pid" => {
                    if field_data[1].len() == 9 {
                        passport.pid = field_data[1].to_string(); passport.no_required_fields += 1;
                    }
                },
                "cid" => { passport.cid = field_data[1].to_string(); },
                _ => println!("No match"),
            }
        }
        pvec.push(passport);
    }

    pvec
}

fn main() -> Result<(), Error> {
    // println!("Test '0' as bytes: {}", '0'.to_string().as_bytes()[0]);
    // println!("Test '9' as bytes: {}", '9'.to_string().as_bytes()[0]);
    // println!("Test 'a' as bytes: {}", 'a'.to_string().as_bytes()[0]);
    // println!("Test 'f' as bytes: {}", 'f'.to_string().as_bytes()[0]);
    let fcontents = read_input()?;
    let passports = clean_input(fcontents);
    let mut no_valid_passports: i32 = 0;
    for passport in passports {
        if passport.no_required_fields >= 7 {
            no_valid_passports += 1;
        }
    }
    println!("Number of valid passwords: {}", no_valid_passports);
    Ok(())
}
