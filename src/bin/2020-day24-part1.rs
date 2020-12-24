use std::collections::HashMap;

fn main() -> Result<(), String> {
    let mut grid = Grid::new();

    include_str!("../../input/2020-day24.txt")
        .lines()
        .map(|path| path_to_position(path))
        .for_each(|position| grid.flip(position));

    let black_tile_count = grid.map.iter()
        .filter(|(_pos, tile)| **tile == TileColor::Black)
        .count();

    println!("{}", black_tile_count);

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
                    None => panic!("Invalid path: {}", path), // TODO: actual error handling?
                    Some(c2) => {
                        match (c1, c2) {
                            ('n', 'w') => position.goto(&Direction::NorthWest),
                            ('n', 'e') => position.goto(&Direction::NorthEast),
                            ('s', 'w') => position.goto(&Direction::SouthWest),
                            ('s', 'e') => position.goto(&Direction::SouthEast),
                            _  => panic!("Invalid path: {}", path), // TODO: actual error handling?
                        }
                    },
                };
            },
        };
    }

    position
}

#[derive(Debug)]
struct Grid {
    map: HashMap<Position, TileColor>,
}

impl Grid {
    fn new() -> Self {
        Grid { map: HashMap::new() }
    }

    fn flip(&mut self, position: Position) {
        let current = self.map.get(&position).unwrap_or_default();

        let new = match current {
            TileColor::Black => TileColor::White,
            TileColor::White => TileColor::Black,
        };

        self.map.insert(position, new);
    }
}

#[derive(Debug, Default, Eq, Hash, PartialEq)]
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
}

enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

#[derive(Debug, PartialEq)]
enum TileColor {
    Black,
    White,
}

impl Default for &TileColor {
    fn default() -> Self {
        &TileColor::White
    }
}
