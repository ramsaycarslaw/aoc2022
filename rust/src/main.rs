use std::fs::File;
use std::io::prelude::*;

mod leaderboard;

// read file into list of strings 
fn read_file(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let reader = std::io::BufReader::new(file);
    reader.lines().map(|l| l.unwrap()).collect()
}

fn day1a(input: &Vec<String>) -> i32 {
    let mut max = 0;
    let mut curr = 0;

    // iterate over the lines
    for line in input {
        if line == "" {
            if curr > max {
                max = curr;
            }

            curr = 0;
            continue;
        }

        curr += line.parse::<i32>().unwrap();
    }
    max
}   

fn day1b(input: &Vec<String>) -> i32 {
    let mut arr = Vec::new();
    let mut curr = 0;

    for line in input {
        if line == "" {
            arr.push(curr);
            curr = 0;
            continue;
        }

        curr += line.parse::<i32>().unwrap();
    }
    arr.sort();
    
    arr.pop().unwrap() + arr.pop().unwrap() + arr.pop().unwrap()
}

fn main() {
   println!("day 1 part a: {}", day1a(&read_file("input/day1")));
   println!("day 1 part b: {}", day1b(&read_file("input/day1")));
}
