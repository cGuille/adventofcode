use std::collections::HashSet;

fn main() {
    let batch = include_str!("../../input/2020-day4.txt");

    let valid_passport_count = batch
        .split("\n\n")
        .filter(|passport_str| has_required_attributes(passport_str))
        .count();

    println!("{}", valid_passport_count);
}

fn has_required_attributes(passport_str: &str) -> bool {
    let attr_set: HashSet<&str> = passport_str
        .split_whitespace()
        .map(|attr_str| attr_str.split(":").next().unwrap())
        .collect();

    attr_set.contains("byr")
        && attr_set.contains("iyr")
        && attr_set.contains("eyr")
        && attr_set.contains("hgt")
        && attr_set.contains("hcl")
        && attr_set.contains("ecl")
        && attr_set.contains("pid")
}
