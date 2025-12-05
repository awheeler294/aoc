use crate::grid::{Grid, GridDirection, Point};

const TARGET_WORD: &str = "XMAS";

pub fn solve(input: &[&str]) -> String {
    let part1 = solve_part_1(input);
    let part2 = solve_part_2(input);

    format!(" Part1: {} \n Part2: {}", part1, part2)
}

fn solve_part_1(input: &[&str]) -> usize {
    let word_grid = Grid::parse_char(input);

    let mut count = 0;

    let first_ch = TARGET_WORD.chars().nth(0).unwrap();
    for (i, val) in word_grid.iter().enumerate() {
        if *val == first_ch {
            let position = word_grid.idx_point(i);
            for direction in GridDirection::all() {
                if let Some(next_point) = position.get_adjacent(direction) {
                    if search_word(&word_grid, next_point, &TARGET_WORD[1..], direction) {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

fn search_word(
    word_grid: &Grid<char>,
    position: Point,
    word: &str,
    direction: GridDirection,
) -> bool {
    let search_ch = word.chars().nth(0).unwrap();

    if word_grid.get(&position) == Some(&search_ch) {
        if word.len() == 1 {
            return true;
        } else {
            if let Some(next_point) = position.get_adjacent(direction) {
                return search_word(word_grid, next_point, &word[1..], direction);
            }
        }
    }

    false
}

fn solve_part_2(input: &[&str]) -> usize {
    let word_grid = Grid::parse_char(input);

    let mut count = 0;

    for y in 1..word_grid.height - 1 {
        for x in 1..word_grid.width - 1 {
            let position = Point::new(x, y);

            if let Some('A') = word_grid.get(&position) {
                let top_left = word_grid.get_direction(&position, GridDirection::UpLeft);
                let top_right = word_grid.get_direction(&position, GridDirection::UpRight);
                let bottom_left = word_grid.get_direction(&position, GridDirection::DownLeft);
                let bottom_right = word_grid.get_direction(&position, GridDirection::DownRight);

                // M.S  M.M  S.M  S.S
                // .A.  .A.  .A.  .A.
                // M.S  S.S  S.M  M.M
                if (top_left == Some(&'M')
                    && bottom_right == Some(&'S')
                    && bottom_left == Some(&'M')
                    && top_right == Some(&'S'))
                    || (top_left == Some(&'M')
                        && bottom_right == Some(&'S')
                        && bottom_left == Some(&'S')
                        && top_right == Some(&'M'))
                    || (top_left == Some(&'S')
                        && bottom_right == Some(&'M')
                        && bottom_left == Some(&'S')
                        && top_right == Some(&'M'))
                    || (top_left == Some(&'S')
                        && bottom_right == Some(&'M')
                        && bottom_left == Some(&'M')
                        && top_right == Some(&'S'))
                {
                    count += 1
                }
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_solve_part_1() {
        let input = [
            "MMMSXXMASM",
            "MSAMXMSMSA",
            "AMXSXMAAMM",
            "MSAMASMSMX",
            "XMASAMXAMM",
            "XXAMMXXAMA",
            "SMSMSASXSS",
            "SAXAMASAAA",
            "MAMMMXMMMM",
            "MXMXAXMASX",
        ];

        let expected = 18;

        let actual = solve_part_1(&input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_solve_part_2() {
        let input = [
            "MMMSXXMASM",
            "MSAMXMSMSA",
            "AMXSXMAAMM",
            "MSAMASMSMX",
            "XMASAMXAMM",
            "XXAMMXXAMA",
            "SMSMSASXSS",
            "SAXAMASAAA",
            "MAMMMXMMMM",
            "MXMXAXMASX",
        ];

        let expected = 9;

        let actual = solve_part_2(&input);

        assert_eq!(actual, expected);
    }
}
