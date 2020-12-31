use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./day2.txt") {
        let mut _valid : i32 = 0;
        for line in lines {
            if is_valid2(&mut line.unwrap()) {
                _valid += 1;
            }
        }
        println!("Number of valid passwords: {}", _valid);
    }
}

fn is_valid2(_line : &mut String) -> bool {
    let spl: Vec<&str> = _line.split(':').collect();
    let min_max_letter: Vec<&str> = spl[0].split('-').collect();
    let max_letter: Vec<&str> = min_max_letter[1].split(' ').collect();

    let min: usize = min_max_letter[0].parse::<usize>().unwrap();
    let max: usize = max_letter[0].parse::<usize>().unwrap();
    let letter: char = max_letter[1].chars().collect::<Vec<char>>()[0];
    let pw: &str = &spl[1].replace(" ", "");
    let pw_chars : Vec<char> = pw.chars().collect();

    return ( pw_chars[min-1] == letter || pw_chars[max-1] == letter ) && pw_chars[min-1] != pw_chars[max-1];
}

fn is_valid(_line : &mut String) -> bool {
    let spl : Vec<&str> = _line.split(':').collect();
    let min_max_letter : Vec<&str> = spl[0].split('-').collect();
    let max_letter : Vec<&str> = min_max_letter[1].split(' ').collect();

    let min : i16 = min_max_letter[0].parse::<i16>().unwrap();
    let max : i16 = max_letter[0].parse::<i16>().unwrap();
    let letter : char = max_letter[1].chars().collect::<Vec<char>>()[0];
    let pw : &str = &spl[1].replace(" ", "");

    let chars : Vec<char> = pw.chars().collect();
    let mut count : i16 = 0;
    for c in chars {
        if c == letter {
            count += 1;
        }
    }

    //println!("{} {} {} {} {}", min, max, letter, pw, count);
    return count >= min && count <= max;
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}