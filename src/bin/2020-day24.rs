use std::collections::HashMap;

fn main() -> Result<(), String> {
    let mut grid = Grid::new();

    include_str!("../../input/2020-day24.txt")
        .lines()
        .map(|path| path_to_position(path))
        .for_each(|position| grid.flip(&position));

    let black_tile_count = grid.flipped.iter()
        .filter(|(_pos, tile)| **tile == TileColor::Black)
        .count();

    println!("Part 1: {}", black_tile_count);

    for _ in 0..100 {
        grid = Grid::from_previous_day(&grid);
    }

    let black_tile_count = grid.flipped.iter()
        .filter(|(_pos, tile)| **tile == TileColor::Black)
        .count();

    println!("Part 2: {}", black_tile_count);

    Ok(())
}

fn path_to_position(path: &str) -> Position {
    let mut position = Position::default();

    let mut chars = path.chars();

    loop {
        match chars.next() {
            None => break,
            Some('e') => position.goto(&Direction::East),
            Some('w') => position.goto(&Direction::West),
            Some(c1) => {
                match chars.next() {
                    None => panic!("Invalid path: {}", path),
                    Some(c2) => {
                        match (c1, c2) {
                            ('n', 'w') => position.goto(&Direction::NorthWest),
                            ('n', 'e') => position.goto(&Direction::NorthEast),
                            ('s', 'w') => position.goto(&Direction::SouthWest),
                            ('s', 'e') => position.goto(&Direction::SouthEast),
                            _  => panic!("Invalid path: {}", path),
                        }
                    },
                };
            },
        };
    }

    position
}

#[derive(Clone, Debug)]
struct Grid {
    flipped: HashMap<Position, TileColor>,
    black_neighbour_counts: HashMap<Position, u8>,
}

impl Grid {
    fn new() -> Self {
        Grid { flipped: HashMap::new(), black_neighbour_counts: HashMap::new() }
    }

    fn from_previous_day(previous_grid: &Grid) -> Self {
        let mut new_grid = previous_grid.clone();

        previous_grid.flipped.iter()
            .filter(|(_, tile)| **tile == TileColor::Black)
            .filter(|(position, _)| {
                let count = previous_grid.black_neighbour_counts.get(position).unwrap_or(&0);
                *count == 0 || *count > 2
            })
            .for_each(|(position, _)| new_grid.flip(position));

        previous_grid.black_neighbour_counts.iter()
            .filter(|(_, count)| **count == 2)
            .filter(|(position, _)| {
                let color = previous_grid.flipped.get(position).unwrap_or(&TileColor::White);
                *color == TileColor::White
            })
            .for_each(|(position, _)| new_grid.flip(position));

        new_grid
    }

    fn flip(&mut self, position: &Position) {
        let previous = self.flipped.get(position);

        let new = match previous {
            None => TileColor::Black,
            Some(TileColor::Black) => TileColor::White,
            Some(TileColor::White) => TileColor::Black,
        };

        for neighbour_pos in position.neighbours().iter() {
            let previous_count = self.black_neighbour_counts.get(neighbour_pos).unwrap_or(&0);
            let new_count = match new {
                TileColor::Black => previous_count + 1,
                TileColor::White => previous_count - 1,
            };

            self.black_neighbour_counts.insert(neighbour_pos.clone(), new_count);
        }

        self.flipped.insert(position.clone(), new);
    }
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
struct Position {
    col: i64,
    row: i64,
}

impl Position {
    fn goto(&mut self, direction: &Direction) {
        match direction {
            Direction::East => self.col += 1,
            Direction::SouthEast => self.row += 1,
            Direction::SouthWest => { self.col -= 1; self.row += 1; },
            Direction::West => self.col -= 1,
            Direction::NorthWest => self.row -= 1,
            Direction::NorthEast => { self.col += 1; self.row -= 1; },
        };
    }

    fn neighbours(&self) -> Vec<Self> {
        [
            Direction::East,
            Direction::SouthEast,
            Direction::SouthWest,
            Direction::West,
            Direction::NorthWest,
            Direction::NorthEast,
        ]
            .iter()
            .map(|direction| self.neighbour(direction))
            .collect()
    }

    fn neighbour(&self, direction: &Direction) -> Self {
        let mut neighbour = self.clone();

        neighbour.goto(direction);

        neighbour
    }
}

#[derive(Debug)]
enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

#[derive(Clone, Debug, PartialEq)]
enum TileColor {
    Black,
    White,
}
