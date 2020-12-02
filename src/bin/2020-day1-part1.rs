use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;

fn main() {
    let file = File::open("input/2020-day1.txt").expect("Input file could not be opened");

    let entries: Result<Vec<u64>, String> = BufReader::new(file)
        .lines()
        .map(|line| {
            line.map_err(|e| e.to_string())?
                .parse()
                .map_err(|e: ParseIntError| e.to_string())
        })
        .collect();

    let entries = entries.unwrap();

    for (index1, entry1) in entries.iter().enumerate() {
        for (index2, entry2) in entries.iter().enumerate() {
            if index1 == index2 {
                continue;
            }

            if entry1 + entry2 == 2020 {
                println!("{}", entry1 * entry2);
                return;
            }
        }
    }

    println!("Not found");
}
