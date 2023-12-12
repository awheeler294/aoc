use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    iter::FromIterator,
    thread,
};

use crate::grid::{Grid, GridDirection, Point};

pub fn solve(input: &[&str]) -> String {
    let part1 = furthest_point_steps(input);

    let part2 = enclosed_tile_count(input);

    format!(" Part1: {} \n Part2: {}", part1, part2)
}

fn furthest_point_steps(pipe_data: &[&str]) -> usize {
    let pipe_grid = parse_pipe_grid(pipe_data);

    find_path(&pipe_grid).len() / 2
}

fn enclosed_tile_count(pipe_data: &[&str]) -> usize {
    let mut pipe_grid = parse_pipe_grid(pipe_data);

    let path = find_path(&pipe_grid);
    // dbg!(&path);

    let start_point = path.last().unwrap();
    let start_node = pipe_grid.get_mut(&start_point).unwrap();
    start_node.tile = {
        let connections: HashSet<GridDirection> =
            HashSet::from_iter(start_node.connections.clone());
        if connections == HashSet::from_iter([GridDirection::Up, GridDirection::Down]) {
            '|'
        } else if connections == HashSet::from_iter([GridDirection::Left, GridDirection::Right]) {
            '-'
        } else if connections == HashSet::from_iter([GridDirection::Up, GridDirection::Left]) {
            'L'
        } else if connections == HashSet::from_iter([GridDirection::Up, GridDirection::Right]) {
            'J'
        } else if connections == HashSet::from_iter([GridDirection::Left, GridDirection::Down]) {
            '7'
        } else if connections == HashSet::from_iter([GridDirection::Right, GridDirection::Down]) {
            'F'
        } else {
            unreachable!()
        }
    };

    let path: HashSet<&Point> = HashSet::from_iter(path.iter());

    let mut is_inside = false;
    let mut inside_outside = vec![];
    let verticals: HashSet<char> = HashSet::from_iter(['|', 'J', 'L']);

    for (i, node) in pipe_grid.iter().enumerate() {
        let point = pipe_grid.idx_point(i);
        if path.contains(&point) {
            if verticals.contains(&node.tile) {
                is_inside = !is_inside;
            }
        } else {
            inside_outside.push((point, is_inside));
        }
    }

    for (point, is_inside) in inside_outside {
        pipe_grid.get_mut(&point).unwrap().tile = match is_inside {
            true => 'I',
            false => 'O',
        };
    }

    dbg!(&pipe_grid);

    pipe_grid.iter().filter(|node| node.tile == 'I').count()
}

fn find_path(pipe_grid: &Grid<Node>) -> Vec<Point> {
    let starting_point = pipe_grid.find_fn(|node| node.tile == 'S').unwrap();

    let mut visited = HashMap::new();
    visited.insert(starting_point, true);

    // overflows the default stack, so run it in a thread with a larger stack :)
    thread::scope(|s| {
        thread::Builder::new()
            .stack_size(200 * 1024 * 1024)
            .spawn_scoped(s, || {
                find_path_rec(&pipe_grid, &vec![starting_point], &mut visited).unwrap()
            })
            .unwrap()
            .join()
    })
    .unwrap()
    // dbg!(&path);
}

fn find_path_rec(
    pipe_grid: &Grid<Node>,
    path: &[Point],
    mut visited: &mut HashMap<Point, bool>,
) -> Option<Vec<Point>> {
    if let Some(position) = path.last() {
        // dbg!(&position);
        let node = pipe_grid.get(position).unwrap();
        // dbg!(&node);
        let mut paths = node
            .connections
            .clone()
            .iter()
            .filter_map(|direction| {
                let (next_point, next) =
                    pipe_grid.enumerate_direction(position, *direction).unwrap();

                let mut path = Vec::from(path);
                path.push(next_point);

                if path.len() > 3 && next.tile == 'S' {
                    return Some(path);
                }

                let is_visited = visited.entry(next_point).or_insert(false);
                if *is_visited == false {
                    *is_visited = true;

                    if let Some(path) = find_path_rec(pipe_grid, &path, &mut visited) {
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

fn parse_pipe_grid(pipe_data: &[&str]) -> Grid<Node> {
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

    pipe_grid
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node {
    tile: char,
    connections: Vec<GridDirection>,
}

impl Node {
    fn new(tile: char, connections: Vec<GridDirection>) -> Self {
        Self { tile, connections }
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

    #[test]
    fn test_enclosed_tile_count() {
        let input = [
            "...........",
            ".S-------7.",
            ".|F-----7|.",
            ".||.....||.",
            ".||.....||.",
            ".|L-7.F-J|.",
            ".|..|.|..|.",
            ".L--J.L--J.",
            "...........",
        ];

        let expected = 4;
        let actual = enclosed_tile_count(&input);
        assert_eq!(actual, expected);

        let input = [
            "..........",
            ".S------7.",
            ".|F----7|.",
            ".||OOOO||.",
            ".||OOOO||.",
            ".|L-7F-J|.",
            ".|II||II|.",
            ".L--JL--J.",
            "..........",
        ];

        let expected = 4;
        let actual = enclosed_tile_count(&input);
        assert_eq!(actual, expected);

        let input = [
            ".F----7F7F7F7F-7....",
            ".|F--7||||||||FJ....",
            ".||.FJ||||||||L7....",
            "FJL7L7LJLJ||LJ.L-7..",
            "L--J.L7...LJS7F-7L7.",
            "....F-J..F7FJ|L7L7L7",
            "....L7.F7||L7|.L7L7|",
            ".....|FJLJ|FJ|F7|.LJ",
            "....FJL-7.||.||||...",
            "....L---J.LJ.LJLJ...",
        ];

        let expected = 8;
        let actual = enclosed_tile_count(&input);
        assert_eq!(actual, expected);

        let input = [
            "FF7FSF7F7F7F7F7F---7",
            "L|LJ||||||||||||F--J",
            "FL-7LJLJ||||||LJL-77",
            "F--JF--7||LJLJ7F7FJ-",
            "L---JF-JLJ.||-FJLJJ7",
            "|F|F-JF---7F7-L7L|7|",
            "|FFJF7L7F-JF7|JL---7",
            "7-L-JL7||F7|L7F-7F7|",
            "L.L7LFJ|||||FJL7||LJ",
            "L7JLJL-JLJLJL--JLJ.L",
        ];

        let expected = 10;
        let actual = enclosed_tile_count(&input);
        assert_eq!(actual, expected);
    }
}
