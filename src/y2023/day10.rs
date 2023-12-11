use std::{fmt::Display, thread};

use crate::grid::{Grid, GridDirection, Point};

pub fn solve(input: &[&str]) -> String {
    let part1 = furthest_point_steps(input);

    let part2 = "";

    format!(" Part1: {} \n Part2: {}", part1, part2)
}

fn furthest_point_steps(pipe_data: &[&str]) -> usize {
    let mut pipe_grid = Grid::parse(pipe_data, Node::from);

    // dbg!(&pipe_grid);
    let starting_point = pipe_grid.find_fn(|node| node.tile == 'S').unwrap();

    let mut connections: Vec<GridDirection> = [
        GridDirection::Up,
        GridDirection::Right,
        GridDirection::Down,
        GridDirection::Left,
    ]
    .iter()
    .filter_map(|direction| {
        if let Some(node) = pipe_grid.get_direction(&starting_point, *direction) {
            match *direction {
                GridDirection::Up => {
                    if node.tile == 'F' || node.tile == '7' || node.tile == '|' {
                        return Some(*direction);
                    }
                }

                GridDirection::Right => {
                    if node.tile == 'J' || node.tile == '7' || node.tile == '-' {
                        return Some(*direction);
                    }
                }
                GridDirection::Down => {
                    if node.tile == 'L' || node.tile == 'J' || node.tile == '|' {
                        return Some(*direction);
                    }
                }

                GridDirection::Left => {
                    if node.tile == 'F' || node.tile == 'L' || node.tile == '-' {
                        return Some(*direction);
                    }
                }

                _ => {}
            }
        }

        None
    })
    .collect();

    let start_node = pipe_grid.get_mut(&starting_point).unwrap();
    start_node.connections.append(&mut connections);
    start_node.visited = true;

    // overflows the default stack, so run it in a thread with a larger stack :)
    let path = thread::scope(|s| {
        thread::Builder::new()
            .stack_size(200 * 1024 * 1024)
            .spawn_scoped(s, || {
                find_path(&mut pipe_grid, &vec![starting_point]).unwrap()
            })
            .unwrap()
            .join()
    })
    .unwrap();
    // dbg!(&path);

    path.len() / 2
}

fn find_path(mut pipe_grid: &mut Grid<Node>, path: &[Point]) -> Option<Vec<Point>> {
    if let Some(position) = path.last() {
        // dbg!(&position);
        let node = pipe_grid.get(position).unwrap();
        // dbg!(&node);
        let mut paths = node
            .connections
            .clone()
            .iter()
            .filter_map(|direction| {
                let (next_point, next) = pipe_grid
                    .enumerate_direction_mut(position, *direction)
                    .unwrap();

                let mut path = Vec::from(path);
                path.push(next_point);

                if path.len() > 3 && next.tile == 'S' {
                    return Some(path);
                }

                if next.visited == false {
                    next.visited = true;

                    if let Some(path) = find_path(&mut pipe_grid, &path) {
                        return Some(path);
                    }
                }

                None
            })
            .collect::<Vec<_>>();

        paths.sort_by(|a, b| a.len().cmp(&b.len()));

        return paths.first().cloned();
    }

    None
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node {
    tile: char,
    connections: Vec<GridDirection>,
    visited: bool,
}

impl Node {
    fn new(tile: char, connections: Vec<GridDirection>) -> Self {
        Self {
            tile,
            connections,
            visited: false,
        }
    }

    fn add_connection(&mut self, connection: GridDirection) {
        self.connections.push(connection);
    }
}

impl From<char> for Node {
    fn from(value: char) -> Self {
        let connections = match value {
            '|' => vec![GridDirection::Up, GridDirection::Down],
            '-' => vec![GridDirection::Left, GridDirection::Right],
            'L' => vec![GridDirection::Up, GridDirection::Right],
            'J' => vec![GridDirection::Up, GridDirection::Left],
            '7' => vec![GridDirection::Left, GridDirection::Down],
            'F' => vec![GridDirection::Right, GridDirection::Down],
            _ => vec![],
        };

        Self::new(value, connections)
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.tile)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_furthest_point_steps() {
        #[rustfmt::skip]
        let pipe_data = [
            ".....", 
            ".S-7.", 
            ".|.|.", 
            ".L-J.", 
            "....."
        ];

        let expected = 4;
        let actual = furthest_point_steps(&pipe_data);
        assert_eq!(actual, expected);

        #[rustfmt::skip]
        let pipe_data = [
            "-L|F7",
            "7S-7|",
            "L|7||",
            "-L-J|",
            "L|-JF",
        ];

        let expected = 4;
        let actual = furthest_point_steps(&pipe_data);
        assert_eq!(actual, expected);

        #[rustfmt::skip]
        let pipe_data = [
            "..F7.",
            ".FJ|.",
            "SJ.L7",
            "|F--J",
            "LJ...",
        ];

        let expected = 8;
        let actual = furthest_point_steps(&pipe_data);
        assert_eq!(actual, expected);

        #[rustfmt::skip]
        let pipe_data = [
            "7-F7-",
            ".FJ|7",
            "SJLL7",
            "|F--J",
            "LJ.LJ",
        ];

        let expected = 8;
        let actual = furthest_point_steps(&pipe_data);
        assert_eq!(actual, expected);
    }
}
