use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input/2018-day1.txt").expect("Input file could not be opened");
    let mut frequency = 0;

    for line in BufReader::new(file).lines() {
        let line = line.expect("Line could not be read from buffer");
        let change = line.parse::<i32>().expect("Could not parse line into i32");

        frequency += change;
    }

    println!("{}", frequency);
}
