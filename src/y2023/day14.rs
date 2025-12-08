use std::{char, collections::HashMap};

use crate::util::grid::{Grid, GridDirection, Point};

pub fn solve(input: &[&str]) -> String {
    let part1 = calculate_load_once(&input);
    let part2 = calculate_load_spin(&input);

    format!(" Part1: {} \n Part2: {}", part1, part2)
}

fn calculate_load_once(input: &[&str]) -> usize {
    let mut platform = Grid::parse_char(input);

    tilt(&mut platform);

    calculate_load(&platform)
}

fn calculate_load_spin(input: &[&str]) -> usize {
    let mut platform = Grid::parse_char(input);

    let mut seen: Option<HashMap<Grid<char>, usize>> = Some(HashMap::new());

    let iterations = 1_000_000_000;

    let mut i = 0;
    while i < iterations {
        for _ in 0..4 {
            tilt(&mut platform);

            platform.rotate_clockwise();
        }

        i += 1;

        if let Some(ref mut platforms) = seen {
            if let Some(n) = platforms.get(&platform) {
                let cycle_start = n;
                let cycle_end = i;
                let cycle_len = cycle_end - cycle_start;

                i += ((iterations - i) / cycle_len) * cycle_len;

                seen = None;
            } else {
                platforms.insert(platform.clone(), i);
            }
        }
    }

    calculate_load(&platform)
}

fn calculate_load(platform: &Grid<char>) -> usize {
    // dbg!(&platform);

    let mut load = 0;
    for y in 0..platform.height {
        for x in 0..platform.width {
            if platform[(x, y)] == 'O' {
                load += platform.height - y;
            }
        }
    }

    load
}

fn tilt(platform: &mut Grid<char>) {
    for x in 0..platform.width {
        for y in 0..platform.height {
            if platform[(x, y)] == 'O' {
                for y in (0..=y).rev() {
                    let point = Point::new(x, y);
                    if let Some((north, symbol)) =
                        platform.enumerate_direction(&point, GridDirection::Up)
                    {
                        if *symbol == '.' {
                            platform[&north] = 'O';
                            platform[(x, y)] = '.';
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_calculate_load() {
        #[rustfmt::skip]
        let input = [
            "O....#....",
            "O.OO#....#",
            ".....##...",
            "OO.#O....O",
            ".O.....O#.",
            "O.#..O.#.#",
            "..O..#O..O",
            ".......O..",
            "#....###..",
            "#OO..#....",
        ];

        let expected = 136;
        let actual = calculate_load_once(&input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_calculate_load_spin() {
        #[rustfmt::skip]
        let input = [
            "O....#....",
            "O.OO#....#",
            ".....##...",
            "OO.#O....O",
            ".O.....O#.",
            "O.#..O.#.#",
            "..O..#O..O",
            ".......O..",
            "#....###..",
            "#OO..#....",
        ];

        let expected = 64;
        let actual = calculate_load_spin(&input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_rotate_tilt() {
        #[rustfmt::skip]
        let input = [
            "O....#....",
            "O.OO#....#",
            ".....##...",
            "OO.#O....O",
            ".O.....O#.",
            "O.#..O.#.#",
            "..O..#O..O",
            ".......O..",
            "#....###..",
            "#OO..#....",
        ];
        let mut platform = Grid::parse_char(&input);

        // North
        tilt(&mut platform);

        #[rustfmt::skip]
        let expected = Grid::parse_char(&[
            "OOOO.#.O..",
            "OO..#....#",
            "OO..O##..O",
            "O..#.OO...",
            "........#.",
            "..#....#.#",
            "..O..#.O.O",
            "..O.......",
            "#....###..",
            "#....#....",
        ]);
        assert_eq!(platform, expected);

        platform.rotate_clockwise();

        #[rustfmt::skip]
        let expected = Grid::parse_char(&[
            "##....OOOO",
            ".......OOO",
            "..OO#....O",
            "......#..O",
            ".......O#.",
            "##.#..O#.#",
            ".#....O#..",
            ".#.O#....O",
            ".....#....",
            "...O#..O#.",
        ]);
        assert_eq!(platform, expected);

        // West
        tilt(&mut platform);

        #[rustfmt::skip]
        let expected = Grid::parse_char(&[
            "##OO..OOOO",
            ".......OOO",
            "....#..O.O",
            "......#..O",
            "......O.#.",
            "##.#..O#.#",
            ".#.O...#.O",
            ".#.O#..O..",
            ".....#....",
            "....#...#.",
        ]);
        assert_eq!(platform, expected);

        platform.rotate_clockwise();

        #[rustfmt::skip]
        let expected = Grid::parse_char(&[
            "....#....#",
            "..###....#",
            ".........O",
            "..OO#....O",
            "#.#....#..",
            ".#........",
            "....OO#..O",
            "..O##..OOO",
            "#....#..OO",
            "...O#.OOOO",
        ]);
        assert_eq!(platform, expected);

        // South
        tilt(&mut platform);

        #[rustfmt::skip]
        let expected = Grid::parse_char(&[
            "....#O..O#",
            "..###...O#",
            "..OO....OO",
            "....#....O",
            "#.#.O..#.O",
            ".#O....O.O",
            "......#O.O",
            "...##.O..O",
            "#..O.#....",
            "....#.....",
        ]);
        assert_eq!(platform, expected);

        platform.rotate_clockwise();

        #[rustfmt::skip]
        let expected = Grid::parse_char(&[
            ".#...#....",
            "....#.....",
            "....O#.O#.",
            ".O#....O#.",
            "#.#..O#.##",
            ".#.......O",
            "..O#......",
            "...OO#....",
            ".......OOO",
            "..OOOOOO##",
        ]);
        assert_eq!(platform, expected);

        // East
        tilt(&mut platform);

        #[rustfmt::skip]
        let expected = Grid::parse_char(&[
            ".#...#.O..",
            ".O..#..O..",
            "....O#.O#.",
            "..#.OO.O#.",
            "#.#.O.#.##",
            ".#O...O.OO",
            "..O#.....O",
            "...O.#....",
            "...O.O....",
            "........##",
        ]);
        assert_eq!(platform, expected);

        //North
        platform.rotate_clockwise();

        #[rustfmt::skip]
        let expected = Grid::parse_char(&[
            ".....#....",
            "....#...O#",
            "...OO##...",
            ".OO#......",
            ".....OOO#.",
            ".O#...O#.#",
            "....O#....",
            "......OOOO",
            "#...O###..",
            "#..OO#....",
        ]);
        assert_eq!(platform, expected);
    }
}
