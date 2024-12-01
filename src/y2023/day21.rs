use std::{
    collections::{HashSet, VecDeque},
    iter::FromIterator,
};

use crate::grid::{Grid, GridDirection};

pub fn solve(input: &[&str]) -> String {
    let part1 = end_positions_count(64, input);
    let part2 = "";

    format!(" Part1: {} \n Part2: {}", part1, part2)
}

fn end_positions_count(target_steps: usize, input: &[&str]) -> usize {
    let grid = Grid::parse_char(input);

    dbg!(&grid);

    let start_point = grid.idx_point(
        grid.iter()
            .enumerate()
            .find_map(|(i, t)| if *t == 'S' { Some(i) } else { None })
            .unwrap(),
    );

    dbg!(&start_point);

    let mut step_count = 0_usize;
    let mut possible_moves = VecDeque::new();
    let mut next_moves = HashSet::new();

    possible_moves.push_back(start_point);
    // dbg!(&possible_moves);

    let directions = [
        GridDirection::Up,
        GridDirection::Right,
        GridDirection::Down,
        GridDirection::Left,
    ];
    loop {
        if let Some(point) = possible_moves.pop_front() {
            // dbg!(step_count);
            for direction in directions.iter() {
                if let Some((neighbor, ch)) = grid.enumerate_direction(&point, *direction) {
                    if *ch != '#' {
                        next_moves.insert(neighbor);
                    }
                }
            }
        } else {
            step_count += 1;

            // dbg!(&possible_moves);
            let mut debug_grid = grid.clone();
            for m in next_moves.iter() {
                debug_grid[m] = 'O';
            }
            dbg!(debug_grid);

            if step_count == target_steps {
                break;
            }

            possible_moves = VecDeque::from_iter(next_moves.into_iter());
            next_moves = HashSet::new();
            // dbg!(&possible_moves);
            // dbg!(&next_moves);
        }
    }

    // dbg!(&next_moves);
    next_moves.len()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_end_positions_count() {
        let input = [
            "...........",
            ".....###.#.",
            ".###.##..#.",
            "..#.#...#..",
            "....#.#....",
            ".##..S####.",
            ".##..#...#.",
            ".......##..",
            ".##.#.####.",
            ".##..##.##.",
            "...........",
        ];

        let expected = 16;
        let actual = end_positions_count(6, &input);

        assert_eq!(actual, expected);
    }
}
