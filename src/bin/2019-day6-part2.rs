use adventofcode::orbital::UniversalOrbitMap;
use std::fs::File;
use std::process;

fn main() {
    let file = File::open("input/2019-day6.txt").expect("Input file could not be opened");
    let uom = UniversalOrbitMap::from(file);

    let you_to_com = uom.path("YOU", "COM").unwrap();
    let san_to_com = uom.path("SAN", "COM").unwrap();

    for (you_index, object) in you_to_com.iter().enumerate() {
        let position = san_to_com.iter().position(|value| value == object);

        match position {
            None => continue,
            Some(san_index) => {
                let distance = you_index + san_index;
                println!("{}", distance);
                process::exit(0);
            }
        };
    }

    process::exit(1);
}
