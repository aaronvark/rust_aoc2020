use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() {
    // HashMap per case
    let mut groups : Vec<HashMap<char, u16>> = Vec::new();
    let mut current : HashMap<char, u16> = HashMap::new();

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./day6.txt") {
        let mut group_len : u16 = 0;
        for line in lines {
            let unwrapped = line.unwrap();
            if unwrapped.len() == 0 {
                current = keep_highest(&mut current, group_len);
                groups.push(current);
                current = HashMap::new();
                group_len = 0;
            } else {
                group_len = group_len + 1;
                let letters: Vec<char> = unwrapped.chars().collect();
                for letter in letters {
                    if !current.contains_key(&letter) {
                        current.insert(letter, 1);
                    }
                    else {
                        if let Some(x) = current.get_mut(&letter) {
                            *x = *x + 1;
                        }
                    }
                }
            }
        }

        //push the last entry
        current = keep_highest(&mut current, group_len);
        groups.push(current);
        println!("Groups: {}", groups.len());

        let mut count : u32 = 0;
        // loop through and multiply all group counts
        for set in groups {
            assert!( set.len() <= 26 );
            count = count + ( set.len() as u32 );
            //println!("Set: {:?}", set );
        }

        println!("Count: {}", count );
    }
}

fn keep_highest( set : &mut HashMap<char, u16>, highest : u16, ) -> HashMap<char, u16> {
    //println!("Checking set for value {}: {:?}", highest, set);
    let mut new_map : HashMap<char, u16> = HashMap::new();
    for ( key, val ) in set.iter() {
        if *val == highest {
            new_map.insert(*key, *val);
        }
    }

    return new_map;
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}