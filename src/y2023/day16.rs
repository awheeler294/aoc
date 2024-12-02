use crate::grid::{Grid, GridDirection, Point};
use std::{collections::HashSet, thread};

pub fn solve(input: &[&str]) -> String {
    let part1 = total_energized(input);
    let part2 = max_energized(input);

    format!(" Part1: {} \n Part2: {}", part1, part2)
}

fn max_energized(grid_data: &[&str]) -> usize {
    let grid = Grid::parse_char(grid_data);

    let mut results = vec![];
    for x in 0..grid.width {
        results.push(thread::scope(|_| {
            energized(
                &Point::new(x, 0),
                GridDirection::Down,
                &grid,
                &mut HashSet::new(),
            )
            .into_iter()
            .count()
        }));

        results.push(thread::scope(|_| {
            energized(
                &Point::new(x, grid.height - 1),
                GridDirection::Up,
                &grid,
                &mut HashSet::new(),
            )
            .into_iter()
            .count()
        }));
    }

    for y in 0..grid.height {
        results.push(thread::scope(|_| {
            energized(
                &Point::new(0, y),
                GridDirection::Right,
                &grid,
                &mut HashSet::new(),
            )
            .into_iter()
            .count()
        }));

        results.push(thread::scope(|_| {
            energized(
                &Point::new(grid.width - 1, y),
                GridDirection::Left,
                &grid,
                &mut HashSet::new(),
            )
            .into_iter()
            .count()
        }));
    }

    *results.iter().max().unwrap()
}

fn total_energized(grid_data: &[&str]) -> usize {
    let grid = Grid::parse_char(grid_data);
    let energized_tiles = energized(
        &Point::new(0, 0),
        GridDirection::Right,
        &grid,
        &mut HashSet::new(),
    );

    // let mut g = grid.clone();
    // for p in energized_tiles.iter() {
    //     g[p] = '#';
    // }
    // dbg!(g);

    energized_tiles.iter().count()
}

fn energized(
    position: &Point,
    direction: GridDirection,
    grid: &Grid<char>,
    visited: &mut HashSet<(Point, GridDirection)>,
) -> HashSet<Point> {
    // dbg!((&position, &direction));

    if visited.contains(&(position.clone(), direction)) {
        return HashSet::new();
    } else {
        visited.insert((position.clone(), direction));
    }

    let mut energized_tiles;
    match grid[position] {
        '.' => {
            if let Some((next, _tile)) = grid.enumerate_direction(position, direction) {
                energized_tiles = energized(&next, direction, grid, visited);
            } else {
                energized_tiles = HashSet::new();
            }
        }

        '/' => {
            let new_direction = match direction {
                GridDirection::Up => GridDirection::Right,
                GridDirection::Down => GridDirection::Left,
                GridDirection::Left => GridDirection::Down,
                GridDirection::Right => GridDirection::Up,
                _ => unreachable!(),
            };

            if let Some((next, _tile)) = grid.enumerate_direction(position, new_direction) {
                energized_tiles = energized(&next, new_direction, grid, visited);
            } else {
                energized_tiles = HashSet::new();
            }
        }

        '\\' => {
            let new_direction = match direction {
                GridDirection::Up => GridDirection::Left,
                GridDirection::Down => GridDirection::Right,
                GridDirection::Left => GridDirection::Up,
                GridDirection::Right => GridDirection::Down,
                _ => unreachable!(),
            };

            if let Some((next, _tile)) = grid.enumerate_direction(position, new_direction) {
                energized_tiles = energized(&next, new_direction, grid, visited);
            } else {
                energized_tiles = HashSet::new();
            }
        }

        '|' => match direction {
            GridDirection::Up | GridDirection::Down => {
                if let Some((next, _tile)) = grid.enumerate_direction(position, direction) {
                    energized_tiles = energized(&next, direction, grid, visited);
                } else {
                    energized_tiles = HashSet::new();
                }
            }
            GridDirection::Left | GridDirection::Right => {
                if let Some((next, _tile)) = grid.enumerate_direction(position, GridDirection::Up) {
                    energized_tiles = energized(&next, GridDirection::Up, grid, visited);
                } else {
                    energized_tiles = HashSet::new();
                }

                if let Some((next, _tile)) = grid.enumerate_direction(position, GridDirection::Down)
                {
                    energized_tiles
                        .extend(energized(&next, GridDirection::Down, grid, visited).into_iter());
                }
            }
            _ => unreachable!(),
        },

        '-' => match direction {
            GridDirection::Up | GridDirection::Down => {
                if let Some((next, _tile)) = grid.enumerate_direction(position, GridDirection::Left)
                {
                    energized_tiles = energized(&next, GridDirection::Left, grid, visited);
                } else {
                    energized_tiles = HashSet::new();
                }

                if let Some((next, _tile)) =
                    grid.enumerate_direction(position, GridDirection::Right)
                {
                    energized_tiles
                        .extend(energized(&next, GridDirection::Right, grid, visited).into_iter());
                }
            }
            GridDirection::Left | GridDirection::Right => {
                if let Some((next, _tile)) = grid.enumerate_direction(position, direction) {
                    energized_tiles = energized(&next, direction, grid, visited);
                } else {
                    energized_tiles = HashSet::new();
                }
            }
            _ => unreachable!(),
        },

        _ => unreachable!(),
    }
    energized_tiles.insert(*position);
    energized_tiles
}

#[cfg(test)]
mod tests {

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_energized() {
        let input = [
            r#".|...\...."#,
            r#"|.-.\....."#,
            r#".....|-..."#,
            r#"........|."#,
            r#".........."#,
            r#".........\"#,
            r#"..../.\\.."#,
            r#".-.-/..|.."#,
            r#".|....-|.\"#,
            r#"..//.|...."#,
        ];

        let expected = 46;
        let actual = total_energized(&input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_energized_1() {
        let input = [
            r#"\|........"#,
            r#".-.....|.."#,
            r#".........."#,
            r#".........."#,
            r#".........."#,
            r#".........."#,
            r#".........."#,
            r#".........."#,
            r#".........."#,
            r#".........."#,
        ];

        let grid = Grid::parse_char(&input);
        let energized_tiles = energized(
            &Point::new(0, 0),
            GridDirection::Down,
            &grid,
            &mut HashSet::new(),
        );
        let mut g = grid.clone();
        for p in energized_tiles.iter() {
            g[p] = '#';
        }
        dbg!(g);

        let expected = 19;
        let actual = energized_tiles.iter().count();

        assert_eq!(actual, expected);

        let input = [
            r#".......\./"#,
            r#".........."#,
            r#".........."#,
            r#".........."#,
            r#".........."#,
            r#".........."#,
            r#".........."#,
            r#".........\"#,
            r#".........."#,
            r#".........."#,
        ];

        let grid = Grid::parse_char(&input);
        let energized_tiles = energized(
            &Point::new(9, 0),
            GridDirection::Left,
            &grid,
            &mut HashSet::new(),
        );
        let mut g = grid.clone();
        for p in energized_tiles.iter() {
            g[p] = '#';
        }
        dbg!(g);

        let expected = 8;
        let actual = energized_tiles.iter().count();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_max_energized() {
        let input = [
            r#".|...\...."#,
            r#"|.-.\....."#,
            r#".....|-..."#,
            r#"........|."#,
            r#".........."#,
            r#".........\"#,
            r#"..../.\\.."#,
            r#".-.-/..|.."#,
            r#".|....-|.\"#,
            r#"..//.|...."#,
        ];

        let expected = 51;
        let actual = max_energized(&input);
        assert_eq!(actual, expected);
    }
}
