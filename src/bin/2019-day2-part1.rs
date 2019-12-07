use adventofcode::intcode::Computer;
use std::fs;

fn main() {
    let source_code = fs::read_to_string("input/2019-day2.txt");
    let source_code = source_code.expect("Input file could not be opened");
    let mut computer = Computer::init(&source_code);

    computer.boot();

    computer.memset(1, 12);
    computer.memset(2, 2);

    computer.run();

    println!("{}", computer.memget(0));
}
