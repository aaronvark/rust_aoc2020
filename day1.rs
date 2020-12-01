use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

// This is the main function
fn main() {
    // Statements here are executed when the compiled binary is called
    let path = Path::new("day1.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => doTheThing(&mut s),
    }
}

fn doTheThing( s: &mut String ) {
    let split = s.split('\n');
    let lines: Vec<&str> = split.collect();
    let mut ints = vec![0; lines.len()];

    for i in 0..lines.len() {
        ints[i] = lines[i].parse::<i32>().unwrap();
    }

    let mut result: i32 = 0;

    'outer: for i in &ints {
        'inner: for j in &ints {
            'in_inner: for k in &ints {
                if i+j+k == 2020 {
                    result = i * j * k;
                    break 'outer;
                }
            }
        }
    }

    print!("Result: {}", result );
}
