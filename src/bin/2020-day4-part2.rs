use std::num::ParseIntError;

fn main() {
    let batch = include_str!("../../input/2020-day4.txt");

    let valid_passport_count = batch
        .split("\n\n")
        .map(|s| Passport::from(s))
        .filter(|p| p.is_valid())
        .count();

    println!("{}", valid_passport_count);
}

#[derive(Debug, Default)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Passport {
    fn is_valid(&self) -> bool {
        self.byr_is_valid()
            && self.iyr_is_valid()
            && self.eyr_is_valid()
            && self.hgt_is_valid()
            && self.hcl_is_valid()
            && self.ecl_is_valid()
            && self.pid_is_valid()
    }

    fn byr_is_valid(&self) -> bool {
        match &self.byr {
            None => false,
            Some(val) => val
                .parse::<u16>()
                .map_or(false, |n| (1920..2003).contains(&n)),
        }
    }

    fn iyr_is_valid(&self) -> bool {
        match &self.iyr {
            None => false,
            Some(val) => val
                .parse::<u16>()
                .map_or(false, |n| (2010..2021).contains(&n)),
        }
    }

    fn eyr_is_valid(&self) -> bool {
        match &self.eyr {
            None => false,
            Some(val) => val
                .parse::<u16>()
                .map_or(false, |n| (2020..2031).contains(&n)),
        }
    }

    fn hgt_is_valid(&self) -> bool {
        match &self.hgt {
            None => false,
            Some(val) => val.parse::<Distance>().map_or(false, accepted_height),
        }
    }

    fn hcl_is_valid(&self) -> bool {
        match &self.hcl {
            None => false,
            Some(val) => is_hex_color(&val),
        }
    }

    fn ecl_is_valid(&self) -> bool {
        match &self.ecl {
            None => false,
            Some(val) => match val.as_str() {
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                _ => false,
            },
        }
    }

    fn pid_is_valid(&self) -> bool {
        match &self.pid {
            None => false,
            Some(val) => val.len() == 9 && val.chars().all(|c| c.is_numeric()),
        }
    }
}

fn accepted_height(distance: Distance) -> bool {
    match distance.unit.as_ref() {
        "cm" => (150..194).contains(&distance.amount),
        "in" => (59..77).contains(&distance.amount),
        _ => false,
    }
}

fn is_hex_color(input: &str) -> bool {
    if input.len() != 7 {
        return false;
    }

    let mut chars = input.chars();

    if chars.next() != Some('#') {
        return false;
    }

    chars.all(|c| c.is_numeric() || ('a'..'g').contains(&c))
}

impl From<&str> for Passport {
    fn from(input: &str) -> Self {
        input
            .split_whitespace()
            .map(|attr_str| {
                let mut attr = attr_str.split(":");
                let name = attr.next().unwrap();
                let val = attr.collect::<Vec<&str>>().join("");

                (name, val)
            })
            .fold(Passport::default(), |mut passport, attribute| {
                match attribute {
                    ("byr", val) => passport.byr = Some(val),
                    ("iyr", val) => passport.iyr = Some(val),
                    ("eyr", val) => passport.eyr = Some(val),
                    ("hgt", val) => passport.hgt = Some(val),
                    ("hcl", val) => passport.hcl = Some(val),
                    ("ecl", val) => passport.ecl = Some(val),
                    ("pid", val) => passport.pid = Some(val),
                    ("cid", val) => passport.cid = Some(val),
                    (name, _) => panic!("Invalid attribute name: {}", name),
                };

                passport
            })
    }
}

#[derive(Debug)]
struct Distance {
    amount: u32,
    unit: String,
}

impl std::str::FromStr for Distance {
    type Err = ParseIntError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut amount = String::new();
        let mut unit = String::new();

        let mut amount_done = false;

        for c in input.chars() {
            if !c.is_numeric() {
                amount_done = true;
            }

            if !amount_done {
                amount.push(c);
            } else {
                unit.push(c);
            }
        }

        let amount = amount.parse()?;

        Ok(Self { amount, unit })
    }
}
