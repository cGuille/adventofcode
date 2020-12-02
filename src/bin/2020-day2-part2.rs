use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input/2020-day2.txt").expect("Input file could not be opened");

    let valid_password_count = BufReader::new(file)
        .lines()
        .map(|line| line.expect("Line could not be read from buffer"))
        .filter(|line| is_valid(&line))
        .count();

    println!("{}", valid_password_count);
}

fn is_valid(line: &str) -> bool {
    let parts: Vec<&str> = line.trim().split(": ").collect();

    let policy: Policy = parts[0].into();
    let password = parts[1];

    policy.accepts(password)
}

#[derive(Debug)]
struct Policy {
    c: char,
    pos1: usize,
    pos2: usize,
}

impl From<&str> for Policy {
    fn from(input: &str) -> Self {
        let parts: Vec<&str> = input.trim().split(" ").collect();

        let occurs: Vec<usize> = parts[0]
            .split("-")
            .map(|part| part.parse().expect("parse error"))
            .collect();

        let c = parts[1];

        Self {
            c: c.parse().expect("parse error"),
            pos1: occurs[0],
            pos2: occurs[1],
        }
    }
}

impl Policy {
    fn accepts(&self, password: &str) -> bool {
        let pos1_matches = password.chars().nth(self.pos1 - 1).unwrap() == self.c;
        let pos2_matches = password.chars().nth(self.pos2 - 1).unwrap() == self.c;

        (pos1_matches || pos2_matches) && (!pos1_matches || !pos2_matches)
    }
}
