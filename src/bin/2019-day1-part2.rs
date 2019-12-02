use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input/2019-day1.txt").expect("Input file could not be opened");

    let total_fuel: f64 = BufReader::new(file)
        .lines()
        .map(|result| result.expect("Line could not be read from buffer"))
        .map(parse_module_mass)
        .map(module_fuel)
        .sum();

    println!("{}", total_fuel);
}

fn parse_module_mass(line: String) -> f64 {
    line.parse::<f64>()
        .expect("Could not parse line into float")
}

fn module_fuel(module_mass: f64) -> f64 {
    let fuel = mass_to_fuel(module_mass);

    if fuel <= 0.0 {
        0.0
    } else {
        fuel + module_fuel(fuel)
    }
}

fn mass_to_fuel(module_mass: f64) -> f64 {
    (module_mass / 3.0).floor() - 2.0
}
