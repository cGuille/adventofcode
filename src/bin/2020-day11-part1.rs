use std::convert::From;
use std::convert::TryFrom;

fn main() {
    let mut layout: SeatLayout = include_str!("../../input/2020-day11.txt")
        .lines()
        .map(|line| line.chars().map(|c| Position::from(c)).collect())
        .collect();

    loop {
        let new_layout = apply_rules(&layout);

        if new_layout == layout {
            break;
        }

        layout = new_layout;
    }

    println!("{:?}", count_occupied_seat(&layout));
}

fn apply_rules(layout: &SeatLayout) -> SeatLayout {
    layout.iter().enumerate().map(|(row_i, row)| {
        row.iter().enumerate().map(|(pos_i, pos)| {
            match pos {
                // If a seat is empty
                Position::EmptySeat => {
                    let has_adj_occupied = adjacent_positions(layout, row_i, pos_i)
                        .iter()
                        .any(|&pos| *pos == Position::OccupiedSeat);

                    if !has_adj_occupied { // …and there are no occupied seats adjacent to it…
                        Position::OccupiedSeat // …the seat becomes occupied.
                    } else {
                        Position::EmptySeat // Otherwise, the seat's state does not change.
                    }
                },
                // If a seat is occupied
                Position::OccupiedSeat => {
                    let adj_occupied_count = adjacent_positions(layout, row_i, pos_i)
                        .iter()
                        .filter(|pos| ***pos == Position::OccupiedSeat)
                        .count();

                    if adj_occupied_count >= 4 { // …and four or more seats adjacent to it are also occupied…
                        Position::EmptySeat // …the seat becomes empty.
                    } else {
                        Position::OccupiedSeat // Otherwise, the seat's state does not change.
                    }
                },
                Position::Floor => Position::Floor, // Otherwise, the seat's state does not change.
            }
        }).collect()
    }).collect()
}

fn adjacent_positions<'a>(layout: &'a SeatLayout, row_i: usize, pos_i: usize) -> Vec<&'a Position> {
    let row_i: i64 = i64::try_from(row_i).unwrap();
    let pos_i: i64 = i64::try_from(pos_i).unwrap();

    let adj_indices = vec![
        (row_i - 1, pos_i), // top
        (row_i - 1, pos_i + 1), // top right
        (row_i, pos_i + 1), // right
        (row_i + 1, pos_i + 1), // bottom right
        (row_i + 1, pos_i), // bottom
        (row_i + 1, pos_i - 1), // bottom left
        (row_i, pos_i - 1), // left
        (row_i - 1, pos_i - 1), // top left
    ];

    adj_indices.iter()
        .filter_map(|(row_i, pos_i)| position_at(layout, *row_i, *pos_i))
        .collect()
}

fn position_at<'a>(layout: &'a SeatLayout, row_i: i64, pos_i: i64) -> Option<&'a Position> {
    let row_i = usize::try_from(row_i);
    let pos_i = usize::try_from(pos_i);

    if row_i.is_err() || pos_i.is_err() {
        return None;
    }

    let row_i = row_i.unwrap();
    let pos_i = pos_i.unwrap();

    layout.get(row_i)?.get(pos_i)
}

fn count_occupied_seat(layout: &SeatLayout) -> usize {
    layout.iter()
        .map(|row| {
            row.iter()
                .filter(|pos| **pos == Position::OccupiedSeat)
                .count()
        })
        .sum()
}

type SeatLayout = Vec<Vec<Position>>;

#[derive(Clone, Debug, PartialEq)]
enum Position {
    Floor,
    EmptySeat,
    OccupiedSeat,
}

impl From<char> for Position {
    fn from(c: char) -> Self {
        match c {
            '.' => Position::Floor,
            'L' => Position::EmptySeat,
            '#' => Position::OccupiedSeat,
            _ => panic!("Unsupported position {:?}", c),
        }
    }
}
