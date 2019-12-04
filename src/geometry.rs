extern crate num_traits;

use num_traits::Signed;
use std::ops::Sub;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point<T> {
    x: T,
    y: T,
}

impl<T: Sub + Signed + Copy> Point<T> {
    pub fn new(x: T, y: T) -> Point<T> {
        Point { x: x, y: y }
    }

    pub fn x(&self) -> T {
        self.x
    }

    pub fn y(&self) -> T {
        self.y
    }

    pub fn manhattan_distance(&self, other: &Point<T>) -> T {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::Point;

    #[test]
    fn test_manhattan_distance() {
        let p1 = Point::new(0, 0);
        let p2 = Point::new(0, 0);

        assert_eq!(p1.manhattan_distance(&p2), 0);
        assert_eq!(p2.manhattan_distance(&p1), 0);

        let p2 = Point::new(1, 1);
        assert_eq!(p1.manhattan_distance(&p2), 2);
        assert_eq!(p2.manhattan_distance(&p1), 2);
    }
}
