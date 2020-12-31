use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

fn main() {
    //hashmap for seat ids
    // Is there such a thing as a set?
    let mut map : HashSet<i32> = HashSet::new();

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./day5.txt") {
        let mut highest_id : i32 = 0;
        for line in lines {
            let chars : Vec<char> = line.unwrap().chars().collect();
            let mut lower : i32 = 0;
            let mut upper : i32 = 127;
            for i in 0..7 {
                if chars[i] == 'F' {
                    upper = upper - ( upper - lower + 1 ) / 2;
                }
                else if chars[i] == 'B' {
                    lower = lower + ( upper - lower + 1 ) / 2;
                }
                //println!("ROW: lower and upper after {}, {} {}", chars[i], lower, upper );
            }

            let row : i32 = upper; // upper and lower should be the same thing at this point
            //println!("row: {}", row);

            //get column
            lower = 0;
            upper = 7;
            for i in 7..10 {
                if chars[i] == 'L' {
                    upper = upper - ( upper - lower + 1 ) / 2;
                }
                else if chars[i] == 'R' {
                    lower = lower + ( upper - lower + 1 ) / 2;
                }
                //println!("COL: lower and upper after {}, {} {}", chars[i], lower, upper );
            }

            let col : i32 = upper;
            let id = row * 8 + col;

            map.insert(id);
            if id > highest_id {
                highest_id = id;
            }
        }

        println!("highest id: {}", highest_id);
        for i in 1..1023 { //max = 127 * 8 + 7
            if !map.contains(&i) {
                if map.contains(&(i-1)) && map.contains(&(i+1)) {
                    println!("Found my seat id: {}", i );
                }
            }
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}