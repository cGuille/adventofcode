use adventofcode::{parse_lines, ParseLinesError};
use std::fs::File;
use std::io;

#[derive(Debug)]
enum MainError {
    Io(io::Error),
    InputParsing(ParseLinesError),
    NotFound,
}

impl From<ParseLinesError> for MainError {
    fn from(error: ParseLinesError) -> Self {
        Self::InputParsing(error)
    }
}

impl From<io::Error> for MainError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

fn main() -> Result<(), MainError> {
    let entries: Vec<u64> = parse_lines(File::open("input/2020-day1.txt")?)?;

    for (index1, entry1) in entries.iter().enumerate() {
        for (index2, entry2) in entries.iter().enumerate() {
            for (index3, entry3) in entries.iter().enumerate() {
                if index1 == index2 || index1 == index3 || index2 == index3 {
                    continue;
                }

                if entry1 + entry2 + entry3 == 2020 {
                    println!("{}", entry1 * entry2 * entry3);
                    return Ok(());
                }
            }
        }
    }

    Err(MainError::NotFound)
}
