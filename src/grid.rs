use anyhow::{anyhow, Result};
use std::cmp::{max, min};
use std::fmt::Display;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::{collections::HashSet, fmt};

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
}

#[derive(PartialEq, Eq, Clone, Hash)]
pub struct Grid<T> {
    spaces: Vec<T>,
    pub width: usize,
    pub height: usize,
}

#[allow(dead_code)]
impl<T> Grid<T>
where
    T: std::fmt::Display + std::fmt::Debug + Clone + PartialEq,
{
    pub fn new(width: usize, height: usize, item: T) -> Self {
        let spaces = vec![item; width * height];
        Self {
            spaces,
            width,
            height,
        }
    }

    pub fn from_vec(spaces: Vec<T>, width: usize, height: usize) -> Self {
        Self {
            spaces,
            width,
            height,
        }
    }

    pub fn parse(lines: &[&str], parse_fn: fn(char) -> T) -> Self {
        let width = lines[0].len();
        let height = lines.len();
        let spaces = lines
            .join("")
            .chars()
            .map(|ch| parse_fn(ch))
            .collect::<Vec<T>>();

        Self {
            spaces,
            width,
            height,
        }
    }

    pub fn get(&self, point: &Point) -> Option<&T> {
        if self.is_in_bounds(point) {
            let y_part = point.y * self.width;
            self.spaces.get(y_part + point.x)
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, point: &Point) -> Option<&mut T> {
        if self.is_in_bounds(point) {
            let y_part = point.y * self.width;
            self.spaces.get_mut(y_part + point.x)
        } else {
            None
        }
    }

    pub fn get_direction(&self, point: &Point, direction: GridDirection) -> Option<&T> {
        if let Some((_, value)) = self.enumerate_direction(point, direction) {
            return Some(value);
        }

        None
    }

    pub fn get_direction_mut(&mut self, point: &Point, direction: GridDirection) -> Option<&mut T> {
        if let Some((_, value)) = self.enumerate_direction_mut(point, direction) {
            return Some(value);
        }

        None
    }

    pub fn enumerate_direction(
        &self,
        point: &Point,
        direction: GridDirection,
    ) -> Option<(Point, &T)> {
        if self.is_in_bounds(point) {
            let d_point = match direction {
                GridDirection::Up => {
                    let y = point.y.checked_sub(1)?;

                    Point::new(point.x, y)
                }

                GridDirection::Down => {
                    let y = point.y.checked_add(1)?;

                    Point::new(point.x, y)
                }

                GridDirection::Left => {
                    let x = point.x.checked_sub(1)?;

                    Point::new(x, point.y)
                }

                GridDirection::Right => {
                    let x = point.x.checked_add(1)?;

                    Point::new(x, point.y)
                }

                GridDirection::UpLeft => {
                    let x = point.x.checked_sub(1)?;
                    let y = point.y.checked_sub(1)?;

                    Point::new(x, y)
                }

                GridDirection::UpRight => {
                    let x = point.x.checked_add(1)?;
                    let y = point.y.checked_sub(1)?;

                    Point::new(x, y)
                }

                GridDirection::DownLeft => {
                    let x = point.x.checked_sub(1)?;
                    let y = point.y.checked_add(1)?;

                    Point::new(x, y)
                }

                GridDirection::DownRight => {
                    let x = point.x.checked_add(1)?;
                    let y = point.y.checked_add(1)?;

                    Point::new(x, y)
                }
            };

            if let Some(value) = self.get(&d_point) {
                return Some((d_point, value));
            }
        }

        None
    }

    pub fn enumerate_direction_mut(
        &mut self,
        point: &Point,
        direction: GridDirection,
    ) -> Option<(Point, &mut T)> {
        if self.is_in_bounds(point) {
            let d_point = match direction {
                GridDirection::Up => {
                    let y = point.y.checked_sub(1)?;

                    Point::new(point.x, y)
                }

                GridDirection::Down => {
                    let y = point.y.checked_add(1)?;

                    Point::new(point.x, y)
                }

                GridDirection::Left => {
                    let x = point.x.checked_sub(1)?;

                    Point::new(x, point.y)
                }

                GridDirection::Right => {
                    let x = point.x.checked_add(1)?;

                    Point::new(x, point.y)
                }

                GridDirection::UpLeft => {
                    let x = point.x.checked_sub(1)?;
                    let y = point.y.checked_sub(1)?;

                    Point::new(x, y)
                }

                GridDirection::UpRight => {
                    let x = point.x.checked_add(1)?;
                    let y = point.y.checked_sub(1)?;

                    Point::new(x, y)
                }

                GridDirection::DownLeft => {
                    let x = point.x.checked_sub(1)?;
                    let y = point.y.checked_add(1)?;

                    Point::new(x, y)
                }

                GridDirection::DownRight => {
                    let x = point.x.checked_add(1)?;
                    let y = point.y.checked_add(1)?;

                    Point::new(x, y)
                }
            };

            if let Some(value) = self.get_mut(&d_point) {
                return Some((d_point, value));
            }
        }

        None
    }

    pub fn set_at(&mut self, x: usize, y: usize, val: T) -> Result<()> {
        let to_modify = self.spaces.get_mut(y * self.width + x).ok_or_else(|| {
            anyhow!("Grid::set_at: x: `{x}`, y: `{y}` is outside the bounds of grid.")
        })?;

        *to_modify = val;

        Ok(())
    }

    pub fn set(&mut self, point: &Point, val: T) -> Result<()> {
        if let Some(to_modify) = self.spaces.get_mut(point.y * self.width + point.x) {
            *to_modify = val;

            Ok(())
        } else {
            Err(anyhow!(
                "Grid::set_at: x: {}, y: {} is outside the grid bounds. width: {}, height: {}",
                point.x,
                point.y,
                self.width,
                self.height
            ))
        }
    }

    pub fn rotate_clockwise(&mut self) {
        let r_width = self.height;
        let r_height = self.width;

        let mut rotated = Self::new(r_width, r_height, self[(0, 0)].clone());

        for (y, line) in self.spaces[..].chunks(self.width).enumerate() {
            for (x, val) in line.iter().enumerate() {
                let rx = r_width - y - 1;
                let ry = x;
                rotated[(rx, ry)] = val.clone();
            }
        }

        std::mem::swap(self, &mut rotated);
    }

    pub fn rotate_counter_clockwise(&mut self) {
        let r_width = self.height;
        let r_height = self.width;

        let mut rotated = Self::new(r_width, r_height, self[(0, 0)].clone());

        for (y, line) in self.spaces[..].chunks(self.width).enumerate() {
            for (x, val) in line.iter().enumerate() {
                let rx = y;
                let ry = r_height - x - 1;
                rotated[(rx, ry)] = val.clone();
            }
        }

        std::mem::swap(self, &mut rotated);
    }

    pub fn idx_point(&self, idx: usize) -> Point {
        let (x, y) = self.idx_xy(idx);

        Point::new(x, y)
    }

    pub fn point_idx(&self, point: Point) -> usize {
        point.y * self.width + point.x
    }

    fn xy_idx(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn idx_xy(&self, idx: usize) -> (usize, usize) {
        let x = idx % self.width;
        let y = idx / self.width;

        (x, y)
    }

    fn idx_xy_static(idx: usize, width: usize) -> (usize, usize) {
        let x = idx % width;
        let y = idx / width;

        (x, y)
    }

    pub fn is_in_bounds(&self, point: &Point) -> bool {
        point.y.checked_mul(self.width).is_some() && point.x < self.width && point.y < self.height
    }

    pub fn print_path(&self, path: &[Point]) {
        let path_idxs = path
            .iter()
            .map(|p| self.point_idx(*p))
            .collect::<HashSet<_>>();

        let mut line = String::new();
        for (i, v) in self.spaces.iter().enumerate() {
            if path_idxs.get(&i).is_some() {
                line.push_str(&format!("\x1b[1m\x1b[94m{}\x1b[0m\x1b[0m", v));
            } else {
                line.push_str(&format!("{}", v));
            }

            if i > 0 && (i + 1) % self.width == 0 {
                println!("{}", line);
                line.truncate(0);
            }
        }

        println!();

        // println!("\x1b[94mTest\x1b[0m");
        // println!("\x1b[1m\x1b[94mTest\x1b[0m\x1b[0m");
    }

    pub fn draw_horizontal_line(&mut self, start: &Point, end: &Point, to_draw: T) -> Result<()> {
        // println!("draw_horizontal_line: start: {:?}, end: {:?}", start, end);
        for x in min(start.x, end.x)..=max(start.x, end.x) {
            self.set_at(x, start.y, to_draw.clone())?;
        }

        Ok(())
    }

    pub fn draw_vertical_line(&mut self, start: &Point, end: &Point, to_draw: T) -> Result<()> {
        // println!("draw_vertical_line: start: {:?}, end: {:?}", start, end);
        for y in min(start.y, end.y)..=max(start.y, end.y) {
            self.set_at(start.x, y, to_draw.clone())?;
        }

        Ok(())
    }

    pub fn find_fn<F: Fn(&T) -> bool>(&self, to_find: F) -> Option<Point> {
        let mut location = None;

        for (i, space) in self.spaces.iter().enumerate() {
            if to_find(space) {
                location = Some(self.idx_point(i));
                break;
            }
        }

        location
    }

    pub fn find(&self, to_find: T) -> Option<Point> {
        self.find_fn(|e| *e == to_find)
    }
}

#[allow(dead_code)]
impl Grid<char> {
    pub fn parse_char(lines: &[&str]) -> Self {
        Self::parse(lines, |ch| ch)
    }
}

#[allow(dead_code)]
impl Grid<u32> {
    pub fn parse_u32(lines: &[&str]) -> Self {
        Self::parse(lines, |n| {
            n.to_digit(10)
                .unwrap_or_else(|| panic!("unable to parse {} as digit", n))
        })
    }
}

impl<T> Index<std::ops::Range<usize>> for Grid<T>
where
    T: Clone + std::fmt::Debug + PartialEq + std::fmt::Display,
{
    type Output = [T];

    fn index(&self, index: std::ops::Range<usize>) -> &Self::Output {
        &self.spaces[index]
    }
}

impl<T> Index<std::ops::RangeFull> for Grid<T>
where
    T: Clone + std::fmt::Debug + PartialEq + std::fmt::Display,
{
    type Output = [T];

    fn index(&self, index: std::ops::RangeFull) -> &Self::Output {
        &self.spaces[index]
    }
}

impl<T> Index<(usize, usize)> for Grid<T>
where
    T: Clone + std::fmt::Debug + PartialEq + std::fmt::Display,
{
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (x, y) = index;
        self.get(&Point::new(x, y)).expect(&format!(
            "Could not get x: {x}, y: {y}. Grid bounds width: {}, height: {}",
            self.width, self.height
        ))
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T>
where
    T: Clone + std::fmt::Debug + PartialEq + std::fmt::Display,
{
    fn index_mut(&mut self, index: (usize, usize)) -> &mut T {
        let (x, y) = index;
        self.get_mut(&Point::new(x, y))
            .expect(&format!("Could not get x: {x}, y: {y}."))
    }
}

impl<T> Index<&Point> for Grid<T>
where
    T: Clone + std::fmt::Debug + PartialEq + std::fmt::Display,
{
    type Output = T;

    fn index(&self, index: &Point) -> &Self::Output {
        self.get(index).expect(&format!(
            "Could not get point {index:?}. Grid bounds width: {}, height: {}",
            self.width, self.height
        ))
    }
}

impl<T> IndexMut<&Point> for Grid<T>
where
    T: Clone + std::fmt::Debug + PartialEq + std::fmt::Display,
{
    fn index_mut(&mut self, index: &Point) -> &mut T {
        self.get_mut(index)
            .expect(&format!("Could not get point {index:?}."))
    }
}

impl<T> Deref for Grid<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.spaces
    }
}

impl<T> DerefMut for Grid<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.spaces
    }
}

impl<T> fmt::Debug for Grid<T>
where
    T: std::fmt::Display + std::fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Grid: {{ ")?;

        if f.alternate() {
            let mut pretty_format = String::new();
            pretty_format.push_str("\n   ");

            write!(f, "{pretty_format}spaces: [ ")?;

            pretty_format.push_str("   ");

            let mut line = pretty_format.to_string();
            for (i, space) in self.spaces.iter().enumerate() {
                // line.push_str(&format!("{:#?}, ", space));
                line.push_str(&format!("{}", space));
                if i > 0 && (i + 1) % self.width == 0 {
                    write!(f, "{line}")?;
                    line = pretty_format.to_string();
                }
            }

            pretty_format.truncate(pretty_format.len() - 3);

            write!(f, "{pretty_format}], ")?;

            pretty_format.truncate(pretty_format.len() - 3);
        } else {
            write!(f, "{:?} ", self.spaces)?;
        }

        write!(f, "}}")
    }
}

impl<T> Display for Grid<T>
where
    T: std::fmt::Display + std::fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut whitespace = String::new();
        whitespace.push_str("\n   ");

        whitespace.push_str("   ");

        let mut line = whitespace.to_string();
        for (i, space) in self.spaces.iter().enumerate() {
            // line.push_str(&format!("{:#?}, ", space));
            line.push_str(&format!("{}", space));
            if i > 0 && (i + 1) % self.width == 0 {
                write!(f, "{line}")?;
                line = whitespace.to_string();
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_direction() {
        let grid_values = ["abcde", "fghij", "klmno", "pqrst", "uvwxy", "z1234"];

        let grid = Grid::parse_char(&grid_values);

        let cases = [
            // top left corner
            (Point::new(0, 0), GridDirection::Up, None),
            (Point::new(0, 0), GridDirection::Down, Some('f')),
            (Point::new(0, 0), GridDirection::Left, None),
            (Point::new(0, 0), GridDirection::Right, Some('b')),
            (Point::new(0, 0), GridDirection::UpLeft, None),
            (Point::new(0, 0), GridDirection::UpRight, None),
            (Point::new(0, 0), GridDirection::DownLeft, None),
            (Point::new(0, 0), GridDirection::DownRight, Some('g')),
            // top right corner
            (Point::new(4, 0), GridDirection::Up, None),
            (Point::new(4, 0), GridDirection::Down, Some('j')),
            (Point::new(4, 0), GridDirection::Left, Some('d')),
            (Point::new(4, 0), GridDirection::Right, None),
            (Point::new(4, 0), GridDirection::UpLeft, None),
            (Point::new(4, 0), GridDirection::UpRight, None),
            (Point::new(4, 0), GridDirection::DownLeft, Some('i')),
            (Point::new(4, 0), GridDirection::DownRight, None),
            // bottom left corner
            (Point::new(0, 5), GridDirection::Up, Some('u')),
            (Point::new(0, 5), GridDirection::Down, None),
            (Point::new(0, 5), GridDirection::Left, None),
            (Point::new(0, 5), GridDirection::Right, Some('1')),
            (Point::new(0, 5), GridDirection::UpLeft, None),
            (Point::new(0, 5), GridDirection::UpRight, Some('v')),
            (Point::new(0, 5), GridDirection::DownLeft, None),
            (Point::new(0, 5), GridDirection::DownRight, None),
            // bottom right corner
            (Point::new(4, 5), GridDirection::Up, Some('y')),
            (Point::new(4, 5), GridDirection::Down, None),
            (Point::new(4, 5), GridDirection::Left, Some('3')),
            (Point::new(4, 5), GridDirection::Right, None),
            (Point::new(4, 5), GridDirection::UpLeft, Some('x')),
            (Point::new(4, 5), GridDirection::UpRight, None),
            (Point::new(4, 5), GridDirection::DownLeft, None),
            (Point::new(4, 5), GridDirection::DownRight, None),
            // center
            (Point::new(2, 2), GridDirection::Up, Some('h')),
            (Point::new(2, 2), GridDirection::Down, Some('r')),
            (Point::new(2, 2), GridDirection::Left, Some('l')),
            (Point::new(2, 2), GridDirection::Right, Some('n')),
            (Point::new(2, 2), GridDirection::UpLeft, Some('g')),
            (Point::new(2, 2), GridDirection::UpRight, Some('i')),
            (Point::new(2, 2), GridDirection::DownLeft, Some('q')),
            (Point::new(2, 2), GridDirection::DownRight, Some('s')),
        ];

        for (point, direction, expected) in cases {
            let actual = grid.get_direction(&point, direction);

            assert_eq!(actual, expected.as_ref(), "Got {actual:?} when expecting {expected:?} from calling get_direction with point {point:?} and direction {direction:?}");
        }
    }

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

    #[test]
    fn test_rotate_clockwise() {
        let mut grid = Grid::parse_char(&[
            "#.##..##.",
            "..#.##.#.",
            "##......#",
            "##......#",
            "..#.##.#.",
            "..##..##.",
            "#.#.##.#.",
        ]);

        #[rustfmt::skip]
        let expected = Grid::parse_char(&[
            "#..##.#", 
            "...##..", 
            "###..##", 
            ".#....#", 
            "#.#..#.", 
            "#.#..#.", 
            ".#....#", 
            "###..##",
            "...##..",
        ]);

        grid.rotate_clockwise();

        assert_eq!(grid, expected);
    }

    #[test]
    fn test_rotate_counter_clockwise() {
        let mut grid = Grid::parse_char(&[
            "#.##..##.",
            "..#.##.#.",
            "##......#",
            "##......#",
            "..#.##.#.",
            "..##..##.",
            "#.#.##.#.",
        ]);

        #[rustfmt::skip]
        let expected = Grid::parse_char(&[
            "..##...", 
            "##..###", 
            "#....#.", 
            ".#..#.#", 
            ".#..#.#", 
            "#....#.", 
            "##..###", 
            "..##...",
            "#.##..#",
        ]);

        grid.rotate_counter_clockwise();

        assert_eq!(grid, expected);
    }
}
