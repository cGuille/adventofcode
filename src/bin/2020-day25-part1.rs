fn main() -> Result<(), String> {
    let card_pub_key = 5290733;
    let door_pub_key = 15231938;

    let card_loop_size = find_loop_size(7, card_pub_key);
    let door_loop_size = find_loop_size(7, door_pub_key);

    let card_enc_key = transform(door_pub_key, card_loop_size);
    let door_enc_key = transform(card_pub_key, door_loop_size);

    assert_eq!(card_enc_key, door_enc_key);

    println!("{}", card_enc_key);

    Ok(())
}

fn find_loop_size(subject_number: u64, expected_result: u64) -> u64 {
    let mut loop_size = 1;
    let mut current_result = 1;

    loop {
        current_result *= subject_number;
        current_result %= 20201227;

        if current_result == expected_result {
            break;
        }

        loop_size += 1;
    }

    loop_size
}

fn transform(subject_number: u64, loop_size: u64) -> u64 {
    let mut result = 1;

    for _ in 0..loop_size {
        result *= subject_number;
        result %= 20201227;
    }

    result
}
