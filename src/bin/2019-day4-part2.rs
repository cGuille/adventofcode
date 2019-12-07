fn main() {
    let mut match_count = 0;

    for n in 136818..(685979 + 1) {
        let digits = as_digits(n);
        let mut seq_len = 1;
        let mut has_seq = false;
        let mut no_decrease = true;

        let mut previous = -1;
        for digit in digits {
            if previous == digit {
                seq_len += 1;
            } else if seq_len == 2 {
                has_seq = true;
            } else {
                seq_len = 1;
            }

            if previous > digit {
                no_decrease = false;
                break;
            }

            previous = digit;
        }

        if seq_len == 2 {
            has_seq = true;
        }

        if has_seq && no_decrease {
            match_count += 1;
        }
    }

    println!("{}", match_count);
}

fn as_digits(n: i32) -> Vec<i32> {
    let mut digits = Vec::new();
    let mut n = n;

    while n > 9 {
        digits.push(n % 10);
        n = n / 10;
    }

    digits.push(n);
    digits.reverse();

    digits
}
