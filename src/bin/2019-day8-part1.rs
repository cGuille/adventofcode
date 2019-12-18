use std::fs::File;
use std::io::prelude::*;

fn main() {
    let file = File::open("input/2019-day8.txt").expect("Could not open input file");

    let image_width = 25;
    let image_height = 6;

    let digits = file
        .bytes()
        .map(|b| b.expect("Could not read byte"))
        .map(char::from)
        .filter(|c| c.is_digit(10));

    let mut layers = Vec::new();
    let mut layer = Vec::new();
    let mut row = Vec::new();

    for digit in digits {
        if layer.len() >= image_height {
            layers.push(layer);
            layer = Vec::new();
        }

        if row.len() >= image_width {
            layer.push(row);
            row = Vec::new();
        }

        row.push(digit);
    }

    let mut min_zeros = std::usize::MAX;
    let mut selected_layer = None;

    for layer in layers {
        let zeros = layer
            .iter()
            .map(|row| row.iter().filter(|digit| **digit == '0').count())
            .sum();

        if zeros < min_zeros {
            min_zeros = zeros;
            selected_layer = Some(layer);
        }
    }

    let selected_layer = selected_layer.expect("No layer found");
    let ones: usize = selected_layer
        .iter()
        .map(|row| row.iter().filter(|digit| **digit == '1').count())
        .sum();
    let twos: usize = selected_layer
        .iter()
        .map(|row| row.iter().filter(|digit| **digit == '2').count())
        .sum();

    println!("{}", ones * twos);
}
