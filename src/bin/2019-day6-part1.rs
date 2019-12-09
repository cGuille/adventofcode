use adventofcode::orbital::UniversalOrbitMap;
use std::fs::File;

fn main() {
    let file = File::open("input/2019-day6.txt").expect("Input file could not be opened");
    let uom = UniversalOrbitMap::from(file);

    let result: usize = uom
        .objects()
        .iter()
        .map(|object| uom.path(object, "COM").unwrap())
        .map(|path| path.len())
        .sum();

    println!("{}", result);
}
