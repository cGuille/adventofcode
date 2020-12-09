fn main() -> Result<(), &'static str> {
    let target = 14144619; // result of part 1

    let input: Vec<i64> = include_str!("../../input/2020-day9.txt")
        .lines()
        .map(|number| number.parse().unwrap())
        .collect();


    for (i, n) in input.iter().enumerate() {
        let mut sum = *n;

        for (j, m) in (&input[i + 1..]).iter().enumerate() {
            sum += m;

            if sum == target {
                let terms = &input[i..(i + j)];
                println!("{}", terms.iter().min().unwrap() + terms.iter().max().unwrap());
                return Ok(())
            }

            if sum > target {
                break;
            }
        }
    }

    Err("Not found")
}
