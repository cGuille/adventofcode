use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;

fn main() {
    let input_file_path = "input/2018-day2.txt";

    let file = File::open(input_file_path).expect("Input file could not be opened");

    // Input file contains one box ID per line.
    // We have to find two IDs having only one different letter
    // at the same position in order to print the letters that
    // are common between those two IDs.

    for line in BufReader::new(file).lines() {
        let box_id = line.expect("Line could not be read from buffer");

        let reopened_file = File::open(input_file_path).expect("Input file could not be opened");
        for other_line in BufReader::new(reopened_file).lines() {
            let other_box_id = other_line.expect("Line could not be read from buffer");

            let mut chars1 = box_id.chars();
            let mut chars2 = other_box_id.chars();
            let mut diff_count = 0;

            loop {
                let c1 = chars1.next();
                let c2 = chars2.next();

                if c1.is_none() || c2.is_none() {
                    break;
                }

                let c1 = c1.unwrap();
                let c2 = c2.unwrap();

                if c1 != c2 {
                    diff_count += 1;
                }

                if diff_count > 1 {
                    break;
                }
            }

            if diff_count == 1 {
                let mut chars1 = box_id.chars();
                let mut chars2 = other_box_id.chars();

                loop {
                    let c1 = chars1.next();
                    let c2 = chars2.next();

                    if c1.is_none() || c2.is_none() {
                        break;
                    }

                    let c1 = c1.unwrap();
                    let c2 = c2.unwrap();

                    if c1 == c2 {
                        print!("{}", c1);
                    }
                }
                println!("");

                process::exit(0);
            }
        }
    }

    process::exit(1);
}
