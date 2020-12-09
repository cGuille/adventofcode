use std::collections::VecDeque;

fn main() -> Result<(), &'static str> {
    let input = include_str!("../../input/2020-day9.txt")
        .lines()
        .map(|number| number.parse::<i64>().unwrap());

    let mut preamble = VecDeque::new();

    for current in input {
        if preamble.len() >= 25 && !is_sum_from(current, &preamble) {
            println!("{}", current);
            return Ok(());
        }

        preamble.push_back(current);
        if preamble.len() > 25 {
            preamble.pop_front();
        }
    }

    Err("Not found")
}

fn is_sum_from(sum: i64, terms: &VecDeque<i64>) -> bool {
    for (i1, term1) in terms.iter().enumerate() {
        for (i2, term2) in terms.iter().enumerate() {
            if i1 == i2 {
                continue;
            }

            if sum == term1 + term2 {
                return true;
            }
        }
    }

    false
}
