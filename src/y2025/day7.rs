use std::collections::HashSet;

use crate::util::grid::Grid;

pub fn solve(input: &[&str]) -> String {
    let (part_1, part_2) = simulate_beams(input);

    format!(" Part1: {part_1} \n Part2: {part_2}")
}

fn simulate_beams(input: &[&str]) -> (u64, u64) {
    let grid = Grid::parse_char(input);

    let mut beams = HashSet::new();
    let mut paths = vec![0; grid.width];
    let mut splits = 0;
    
    for x in 0..grid.width {
        if grid[(x, 0)] == 'S' {
            beams.insert(x);
            paths[x] += 1;
            break;
        }
    }

    for y in 2..grid.height {

        let mut next_gen = HashSet::new();
        let mut next_paths = vec![0; grid.width];

        for x in beams {
            if grid[(x, y)] == '^' {
                splits += 1;
                next_paths[x - 1] += paths[x];
                next_paths[x + 1] += paths[x];
                next_gen.insert(x - 1);
                next_gen.insert(x + 1);
            } else {
                next_gen.insert(x);
                next_paths[x] += paths[x];
            }
        }

        beams = next_gen;
        paths = next_paths;
    }

    (splits, paths.iter().sum())

}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;

    fn solve_part_1(input: &[&str]) -> u64 {
        simulate_beams(input).0
    }

    fn solve_part_2(input: &[&str]) -> u64 {
        simulate_beams(input).1
    }

    #[test]
    fn test_solve_part_1() {
        #[rustfmt::skip]
      let input = [
            ".......S.......",
            "...............",
            ".......^.......",
            "...............",
            "......^.^......",
            "...............",
            ".....^.^.^.....",
            "...............",
            "....^.^...^....",
            "...............",
            "...^.^...^.^...",
            "...............",
            "..^...^.....^..",
            "...............",
            ".^.^.^.^.^...^.",
            "...............",
        ];

        let expected = 21;

        let actual = solve_part_1(&input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_solve_part_2() {
        #[rustfmt::skip]
      let input = [
            ".......S.......",
            "...............",
            ".......^.......",
            "...............",
            "......^.^......",
            "...............",
            ".....^.^.^.....",
            "...............",
            "....^.^...^....",
            "...............",
            "...^.^...^.^...",
            "...............",
            "..^...^.....^..",
            "...............",
            ".^.^.^.^.^...^.",
            "...............",
        ];

        let expected = 40;

        let actual = solve_part_2(&input);

        assert_eq!(actual, expected);
    }
}
