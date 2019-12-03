use adventofcode::intcode::Computer;
use std::fs;

fn main() {
    let source_code = fs::read_to_string("input/2019-day2.txt");
    let source_code = source_code.expect("Input file could not be opened");
    let mut computer = Computer::init(&source_code);

    println!("{}", computer.run(12, 2));
}
