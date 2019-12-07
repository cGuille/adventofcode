use adventofcode::intcode::Computer;
use std::fs;
use std::process;

fn main() {
    let source_code = fs::read_to_string("input/2019-day2.txt");
    let source_code = source_code.expect("Input file could not be opened");
    let mut computer = Computer::init(&source_code);

    let searched_result = 19690720;

    for noun in 0..100 {
        for verb in 0..100 {
            computer.boot();

            computer.memset(1, noun);
            computer.memset(2, verb);

            computer.run();

            let result = computer.memget(0);

            if result == searched_result {
                println!("{}", 100 * noun + verb);
                process::exit(0);
            }
        }
    }

    process::exit(1);
}
