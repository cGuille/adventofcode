pub mod geometry;
pub mod intcode;
pub mod orbital;

use std::io;
use std::io::BufRead;

#[derive(Debug)]
pub enum ParseLinesError {
    Io(io::Error),
    Parsing,
}

impl From<io::Error> for ParseLinesError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

pub fn parse_lines<R: io::Read, IntoT: std::str::FromStr>(input: R) -> Result<Vec<IntoT>, ParseLinesError> {
    let mut entries = Vec::new();

    for line_result in io::BufReader::new(input).lines() {
        let line = line_result?;

        match line.parse::<IntoT>() {
            Ok(entry) => entries.push(entry),
            Err(_) => return Err(ParseLinesError::Parsing),
        };
    }

    Ok(entries)
}
