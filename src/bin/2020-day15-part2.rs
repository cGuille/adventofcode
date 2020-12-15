use std::collections::HashMap;
use std::time::Instant;

fn main() -> Result<(), String> {
    let start_time = Instant::now();
    assert_eq!(614, nth(&vec![14,3,1,0,9,5], 2020)?);
    println!("Verified part 1 in {:?}", start_time.elapsed());

    let start_time = Instant::now();
    assert_eq!(1065, nth(&vec![14,3,1,0,9,5], 30000000)?);
    println!("Verified part 2 in {:?}", start_time.elapsed());

    Ok(())
}

fn nth(starting_numbers: &Vec<usize>, n: usize) -> Result<usize, String> {
    assert!(starting_numbers.len() > 0);

    let mut turn = 1;
    let mut positions = HashMap::new();
    let mut prev_number: Option<usize> = None;

    loop {
        let new_number = if turn <= starting_numbers.len() {
            starting_numbers[turn - 1]
        } else {
            match positions.get(&prev_number.unwrap()) {
                None => 0,
                Some(prev_number_position) => (turn - 1) - prev_number_position,
            }
        };

        if turn >= n {
            return Ok(new_number);
        }

        if let Some(prev) = prev_number {
            positions.insert(prev, turn - 1);
        }

        prev_number = Some(new_number);
        turn +=1;
    }
}
