
use std::ops::Add;
use std::cmp::{max, min};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn manhattan_distance(&self, other: Point) -> usize {
        (max(self.x, other.x) - min(self.x, other.x))
            + (max(self.y, other.y) - min(self.y, other.y))
    }

    pub fn get_adjacent(self, direction: GridDirection) -> Option<Self> {
        Some(match direction {
            GridDirection::Up => {
                let y = self.y.checked_sub(1)?;

                Self::new(self.x, y)
            }

            GridDirection::Down => {
                let y = self.y.checked_add(1)?;

                Self::new(self.x, y)
            }

            GridDirection::Left => {
                let x = self.x.checked_sub(1)?;

                Self::new(x, self.y)
            }

            GridDirection::Right => {
                let x = self.x.checked_add(1)?;

                Self::new(x, self.y)
            }

            GridDirection::UpLeft => {
                let x = self.x.checked_sub(1)?;
                let y = self.y.checked_sub(1)?;

                Self::new(x, y)
            }

            GridDirection::UpRight => {
                let x = self.x.checked_add(1)?;
                let y = self.y.checked_sub(1)?;

                Self::new(x, y)
            }

            GridDirection::DownLeft => {
                let x = self.x.checked_sub(1)?;
                let y = self.y.checked_add(1)?;

                Self::new(x, y)
            }

            GridDirection::DownRight => {
                let x = self.x.checked_add(1)?;
                let y = self.y.checked_add(1)?;

                Self::new(x, y)
            }
        })
    }
}

impl Add<GridDirection> for Point {
    type Output = Self;

    fn add(self, direction: GridDirection) -> Self {
        match direction {
            GridDirection::Up => {
                let y = self.y - 1;

                Self::new(self.x, y)
            }

            GridDirection::Down => {
                let y = self.y + 1;

                Self::new(self.x, y)
            }

            GridDirection::Left => {
                let x = self.x - 1;

                Self::new(x, self.y)
            }

            GridDirection::Right => {
                let x = self.x + 1;

                Self::new(x, self.y)
            }

            GridDirection::UpLeft => {
                let x = self.x - 1;
                let y = self.y - 1;

                Self::new(x, y)
            }

            GridDirection::UpRight => {
                let x = self.x + 1;
                let y = self.y - 1;

                Self::new(x, y)
            }

            GridDirection::DownLeft => {
                let x = self.x - 1;
                let y = self.y + 1;

                Self::new(x, y)
            }

            GridDirection::DownRight => {
                let x = self.x + 1;
                let y = self.y + 1;

                Self::new(x, y)
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum GridDirection {
    Up,
    Down,
    Left,
    Right,

    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl GridDirection {
    pub fn all() -> [Self; 8] {
        [
            Self::UpLeft,
            Self::Up,
            Self::UpRight,
            Self::Right,
            Self::DownRight,
            Self::Down,
            Self::DownLeft,
            Self::Left,
        ]
    }

    pub fn turn_clockwise_90(&mut self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::UpLeft => Self::UpRight,
            Self::UpRight => Self::DownRight,
            Self::DownRight => Self::DownLeft,
            Self::DownLeft => Self::UpLeft,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_manhattan_distance() {
        let cases = [
            (Point::new(9, 16), Point::new(0, 16), 9),
            (Point::new(13, 2), Point::new(15, 3), 3),
            (Point::new(12, 14), Point::new(10, 16), 4),
            (Point::new(10, 20), Point::new(10, 16), 4),
            (Point::new(14, 17), Point::new(10, 16), 5),
            (Point::new(8, 7), Point::new(2, 10), 9),
            (Point::new(2, 0), Point::new(2, 10), 10),
            (Point::new(0, 11), Point::new(2, 10), 3),
            (Point::new(20, 14), Point::new(25, 17), 8),
            (Point::new(25, 17), Point::new(20, 14), 8),
            (Point::new(17, 20), Point::new(21, 22), 6),
            (Point::new(16, 7), Point::new(15, 3), 5),
            (Point::new(14, 3), Point::new(15, 3), 1),
            (Point::new(20, 1), Point::new(15, 3), 7),
            (Point::new(4, 0), Point::new(9, 10), 15),
            (Point::new(0, 2), Point::new(12, 7), 17),
            (Point::new(0, 11), Point::new(5, 11), 5),
            (Point::new(1, 15), Point::new(13, 0), 27),
            (Point::new(1, 15), Point::new(37, 1), 50),
            (Point::new(1, 15), Point::new(0, 2), 14),
            (Point::new(1, 15), Point::new(36, 14), 36),
        ];
        //             Point { x: 13, y: 0  },
        //             Point { x: 37, y: 1  },
        //             Point { x: 0,  y: 2  },
        //             Point { x: 36, y: 14 },
        //             Point { x: 1,  y: 15 },
        for (a, b, expected) in cases {
            let actual = a.manhattan_distance(b);
            assert_eq!(actual, expected, "Got {actual} when expecting {expected} from calling manhattan_distance on {:#?} and {:#?}", a, b);
            let actual = b.manhattan_distance(a);
            assert_eq!(actual, expected, "Got {actual} when expecting {expected} from calling manhattan_distance on {:#?} and {:#?}", b, a);
        }
    }
}
