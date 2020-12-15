use std::convert::TryInto;
use std::num::TryFromIntError;

fn main() -> Result<(), String> {
    let mut numbers = vec![14,3,1,0,9,5];

    loop {
        let number = next(&numbers).map_err(|error| format!("{}", error))?;

        numbers.push(number);

        if numbers.len() >= 2020 {
            break;
        }
    }

    println!("{}", numbers.last().unwrap());

    Ok(())
}

fn next(numbers: &Vec<i64>) -> Result<i64, TryFromIntError> {
    let last = numbers.last().unwrap();
    let last_pos = numbers.len() - 1;

    numbers[..last_pos].iter().rev().position(|n| n == last).map_or(Ok(0), |pos| {
        (pos + 1).try_into()
    })
}
