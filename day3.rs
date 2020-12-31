use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut grid : Vec<Vec<char>> = Vec::new();

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./day3.txt") {
        let mut _valid: i32 = 0;
        for line in lines {
            let x : Vec<char> = line.unwrap().chars().collect::<Vec<char>>();
            grid.push(x);
        }

        let mut total : u32;

        total = run_grid(&grid,1,1);
        total = total * run_grid(&grid,3,1);
        total = total * run_grid(&grid,5,1);
        total = total * run_grid(&grid,7,1);
        total = total * run_grid(&grid,1,2);

        println!("Compiled treecount: {}", total);
    }
}

fn run_grid( grid : &Vec<Vec<char>> , x_step: usize, y_step: usize ) -> u32 {
    let mut x : usize = 0;
    let mut y : usize = 0;
    let mut tree_count : u32 = 0;

    let w : usize = grid[0].len();
    let h : usize = grid.len();

    while y < h && x < w {
        //println!("[{},{}] {}", x, y, grid[y][x] );
        if grid[y][x] == '#' {
            tree_count += 1;
        }
        x = ( x + x_step ) % w;
        y += y_step;
    }

    return tree_count;
}


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}