use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() {
    let file = File::open("input/2018-day2.txt").expect("Input file could not be opened");

    // Input file contains one box ID per line.
    // We have to compute TWICE × THRICE where:
    // - TWICE is the number of box IDs with at least one letter repeated exactly twice.
    // - THRICE is the number of box IDs with at least one letter repeated exactly thrice.

    let mut twice = 0;
    let mut thrice = 0;

    for line in BufReader::new(file).lines() {
        let box_id = line.expect("Line could not be read from buffer");

        let mut letter_counts = HashMap::new();

        for letter in box_id.chars() {
            *letter_counts.entry(letter).or_insert(0) += 1
        }

        let mut has_twice = false;
        let mut has_thrice = false;

        for count in letter_counts.values() {
            if count == &2 {
                has_twice = true;
            }
            if count == &3 {
                has_thrice = true;
            }

            if has_thrice && has_twice {
                break;
            }
        }

        if has_twice {
            twice += 1;
        }

        if has_thrice {
            thrice += 1;
        }
    }

    println!("{}", twice * thrice);
}
