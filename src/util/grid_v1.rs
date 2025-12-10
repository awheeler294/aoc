use anyhow::{Result, anyhow};
use std::cmp::{max, min};
use std::{collections::HashSet, fmt};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn manhattan_distance(&self, other: Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct GridDirections {}
#[allow(dead_code)]
impl GridDirections {
    pub const UP: (i32, i32) = (0, -1);
    pub const DOWN: (i32, i32) = (0, 1);
    pub const LEFT: (i32, i32) = (-1, 0);
    pub const RIGHT: (i32, i32) = (1, 0);

    pub const UP_LEFT: (i32, i32) = (-1, -1);
    pub const UP_RIGHT: (i32, i32) = (1, -1);
    pub const DOWN_LEFT: (i32, i32) = (-1, 1);
    pub const DOWN_RIGHT: (i32, i32) = (1, 1);
}

#[derive(PartialEq, Eq)]
pub struct Grid<T> {
    pub spaces: Vec<T>,
    pub width: usize,
    pub height: usize,
}

#[allow(dead_code)]
impl<T> Grid<T>
where
    T: std::fmt::Display + std::fmt::Debug + Clone + PartialEq,
{
    pub fn new(spaces: Vec<T>, width: usize, height: usize) -> Self {
        Self {
            spaces,
            width,
            height,
        }
    }

    pub fn build(width: usize, height: usize, item: T) -> Self {
        let spaces = vec![item; width * height];
        Self {
            spaces,
            width,
            height,
        }
    }

    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        y as usize * self.width + x as usize
    }

    pub fn xy_idx_i32(&self, x: i32, y: i32) -> i32 {
        (y * self.width as i32 + x) as i32
    }

    pub fn at(&self, x: usize, y: usize) -> Option<&T> {
        self.spaces.get(y * self.width + x)
    }

    pub fn at_point(&self, point: &Point) -> Option<&T> {
        if self.is_in_bounds(point.x, point.y) {
            let y_part = (point.y as usize).checked_mul(self.width).expect(&format!(
                "Grid::at_point: attempt to multiply with overflow: point: {:#?}, width: {:#?}",
                point, self.width
            ));
            self.spaces.get(y_part + point.x as usize)
        } else {
            None
        }
    }

    pub fn set_at(&mut self, x: usize, y: usize, val: T) -> Result<()> {
        let to_modify = self.spaces.get_mut(y * self.width + x).ok_or_else(|| {
            anyhow!("Grid::set_at: x: `{x}`, y: `{y}` is outside the bounds of grid.")
        })?;

        *to_modify = val;

        Ok(())
    }

    pub fn set_at_point(&mut self, point: &Point, val: T) -> Result<()> {
        self.set_at(point.x as usize, point.y as usize, val)
    }

    pub fn idx_xy(&self, idx: usize) -> (i32, i32) {
        let x = idx % self.width;
        let y = idx / self.width;

        (x as i32, y as i32)
    }

    pub fn idx_xy_static(idx: usize, width: usize) -> (i32, i32) {
        let x = idx % width;
        let y = idx / width;

        (x as i32, y as i32)
    }

    pub fn idx_point(&self, idx: usize) -> Point {
        let (x, y) = self.idx_xy(idx);

        Point::new(x, y)
    }

    pub fn point_idx(&self, point: Point) -> usize {
        point.y as usize * self.width + point.x as usize
    }

    pub fn is_in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32
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
            self.set_at(x as usize, start.y as usize, to_draw.clone())?;
        }

        Ok(())
    }

    pub fn draw_vertical_line(&mut self, start: &Point, end: &Point, to_draw: T) -> Result<()> {
        // println!("draw_vertical_line: start: {:?}, end: {:?}", start, end);
        for y in min(start.y, end.y)..=max(start.y, end.y) {
            self.set_at(start.x as usize, y as usize, to_draw.clone())?;
        }

        Ok(())
    }

    pub fn find(&self, to_find: T) -> Option<Point> {
        let mut location = None;

        for (i, space) in self.spaces.iter().enumerate() {
            if *space == to_find {
                location = Some(self.idx_point(i));
                break;
            }
        }

        location
    }
}

#[allow(dead_code)]
impl Grid<i32> {
    pub fn from_lines(lines: &[&str]) -> Self {
        let width = lines[0].len();
        let height = lines.len();
        let spaces = lines
            .join("")
            .chars()
            .map(|n| {
                n.to_digit(10)
                    .unwrap_or_else(|| panic!("unable to parse {} as digit", n))
                    as i32
            })
            .collect::<Vec<i32>>();

        Self {
            spaces,
            width,
            height,
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_manhattan_distance() {
        let cases = [
            (Point::new(2, 18), Point::new(-2, 15), 7),
            (Point::new(9, 16), Point::new(0, 16), 9),
            (Point::new(13, 2), Point::new(15, 3), 3),
            (Point::new(12, 14), Point::new(10, 16), 4),
            (Point::new(10, 20), Point::new(10, 16), 4),
            (Point::new(14, 17), Point::new(10, 16), 5),
            (Point::new(8, 7), Point::new(2, 10), 9),
            (Point::new(2, 0), Point::new(2, 10), 10),
            (Point::new(0, 11), Point::new(2, 10), 3),
            (Point::new(20, 14), Point::new(25, 17), 8),
            (Point::new(17, 20), Point::new(21, 22), 6),
            (Point::new(16, 7), Point::new(15, 3), 5),
            (Point::new(14, 3), Point::new(15, 3), 1),
            (Point::new(20, 1), Point::new(15, 3), 7),
        ];

        for (a, b, expected) in cases {
            let actual = a.manhattan_distance(b);
            assert_eq!(
                actual, expected,
                "Got {actual} when expecting {expected} from calling manhattan_distance on {:#?} and {:#?}",
                a, b
            );
            let actual = b.manhattan_distance(a);
            assert_eq!(
                actual, expected,
                "Got {actual} when expecting {expected} from calling manhattan_distance on {:#?} and {:#?}",
                b, a
            );
        }
    }
}
