use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;
use std::process;

fn main() {
    let mut seen_frequencies = HashSet::new();
    let mut frequency = 0;

    seen_frequencies.insert(frequency);

    loop {
        let file = File::open("input/2018-day1.txt").expect("Input file could not be opened");

        for line in BufReader::new(file).lines() {
            let line = line.expect("Line could not be read from buffer");
            let change = line.parse::<i32>().expect("Could not parse line into i32");

            frequency += change;

            let already_seen = !seen_frequencies.insert(frequency);

            if already_seen {
                println!("{}", frequency);
                process::exit(0);
            }
        }
    }
}
