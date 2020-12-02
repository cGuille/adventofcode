use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::Error as IoError;
use std::num::ParseIntError;

#[derive(Debug)]
enum Error {
    Io(IoError),
    IntParsing(ParseIntError),
    NotFound,
}

impl From<IoError> for Error {
    fn from(error: IoError) -> Self {
        Self::Io(error)
    }
}

impl From<ParseIntError> for Error {
    fn from(error: ParseIntError) -> Self {
        Self::IntParsing(error)
    }
}

fn main() -> Result<(), Error> {
    let file = File::open("input/2020-day1.txt")?;

    let entries: Vec<u64> = BufReader::new(file)
        .lines()

        // Why do I get the following error:
        //   error[E0277]: the trait bound `std::result::Result<u64, Error>: FromStr` is not satisfied
        // It looks like I call parse() on a Result<u64, Error>, but line is of type Result<String, IoError>
        // and I use the ? operator afterwards so I would have thought that it either calls parse on a String
        // (Ok variant of the Result) or early returns the IoError that gets converted to Error because of the
        // impl From<IoError> for Error.
        // What is happening??
        .map(|line| line?.parse()?)
//                        ^^^^^ the trait `FromStr` is not implemented for `std::result::Result<u64, Error>`
        .collect::<Result<Vec<u64>, Error>>()?;

    for (index1, entry1) in entries.iter().enumerate() {
        for (index2, entry2) in entries.iter().enumerate() {
            if index1 == index2 {
                continue;
            }

            if entry1 + entry2 == 2020 {
                println!("{}", entry1 * entry2);
                return Ok(());
            }
        }
    }

    Err(Error::NotFound)
}
