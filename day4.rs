use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() {
    // HashMap per case
    let mut passports : Vec<HashMap<String, String>> = Vec::new();
    let mut current : HashMap<String,String> = HashMap::new();

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./day4.txt") {
        for line in lines {
            let unwrapped = line.unwrap();
            if unwrapped.len() == 0 {
                passports.push(current);
                current = HashMap::new();
            }
            else {
                let sets : Vec<&str> = unwrapped.split(" ").collect();
                for set in sets {
                    //println!("{}", set);
                    let split_set : Vec<&str> = set.split(":").collect();
                    current.insert(split_set[0].to_string(), split_set[1].to_string());
                }
            }
        }

        //push the last entry
        passports.push(current);

        //find valif passports]
        let mut valid_count : i16 = 0;
        for passport in passports {
            let ids = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
            let mut valid : bool = true;
            for id in ids {
                if !passport.contains_key(id) {
                    //println!("Not found: {}", id );
                    valid = false;
                }
                else if valid {
                    match &id[..] {
                        "byr"=> valid = valid_byr(&passport[id]),
                        "iyr"=> valid = valid_iyr(&passport[id]),
                        "eyr"=> valid = valid_eyr(&passport[id]),
                        "hgt"=> valid = valid_hgt(&passport[id]),
                        "hcl"=> valid = valid_hcl(&passport[id]),
                        "ecl"=> valid = valid_ecl(&passport[id]),
                        "pid"=> valid = valid_pid(&passport[id]),
                        &_=>(),
                    };
                }
            }
            if valid {
                valid_count += 1;
            }
        }

        println!("Valid passports: {}", valid_count );
    }
}

fn valid_byr( value : &str ) -> bool {
    let i = value.parse::<i32>().unwrap();
    return i >= 1920 && i <= 2002;
}

fn valid_iyr( value : &str ) -> bool {
    let i = value.parse::<i32>().unwrap();
    return i >= 2010 && i <= 2020;
}

fn valid_eyr( value : &str ) -> bool {
    let i = value.parse::<i32>().unwrap();
    return i >= 2020 && i <= 2030;
}

fn valid_hgt( value : &str ) -> bool {
    let i : usize = value.len();
    if i <= 2 {
        return false;
    }

    let format = &value[i-2..];
    if format == "in" {
        let ival : i32 = value[0..i-2].parse::<i32>().unwrap();
        return ival >= 59 && ival <= 76;
    }
    else if format == "cm" {
        let ival : i32 = value[0..i-2].parse::<i32>().unwrap();
        return ival >= 150 && ival <= 193;
    }
    else {
        return false;
    }
}

fn valid_hcl( value : &str ) -> bool {
    let chars : Vec<char> = value.chars().collect();
    if chars[0] != '#' || chars.len() != 7 {
        return false;
    }

    return true;
}

fn valid_ecl( value : &str ) -> bool {
    let valid_colors : Vec<&str> = vec![&"amb", &"blu", &"brn", &"gry", &"grn", &"hzl", &"oth"];
    return valid_colors.contains(&value);
}

fn valid_pid( value : &str ) -> bool {
    return value.len() == 9;
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}