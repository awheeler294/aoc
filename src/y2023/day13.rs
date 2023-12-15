use std::iter::FromIterator;

use crate::grid::Grid;

pub fn solve(input: &[&str]) -> String {
    let (part1, part2) = reflection_summery(&input);

    format!(" Part1: {} \n Part2: {}", part1, part2)
}

fn reflection_summery(input: &[&str]) -> (usize, usize) {
    let patterns = input
        .split(|line| line.is_empty())
        .collect::<Vec<&[&str]>>();

    let horizontal_count = patterns
        .iter()
        .map(|pattern| horizontal_reflection(&pattern))
        .sum::<usize>();

    let vertical_count = patterns
        .iter()
        .map(|pattern| vertical_reflection(&pattern))
        .sum::<usize>();

    let mut horizontal_count_s = 0;
    let mut vertical_count_s = 0;
    for pattern in patterns.iter() {
        let h_count = smudged_horizontal_reflection(&pattern);
        let v_count = smudged_vertical_reflection(&pattern);
        // dbg!(pattern);
        // dbg!(h_count);
        // dbg!(v_count);
        horizontal_count_s += h_count;
        vertical_count_s += v_count;
    }

    (
        vertical_count + horizontal_count * 100,
        vertical_count_s + horizontal_count_s * 100,
    )
}

fn smudged_vertical_reflection(pattern: &[&str]) -> usize {
    let rotated = rotate(pattern);
    let pattern = rotated
        .iter()
        .map(|line| line.as_str())
        .collect::<Vec<&str>>();

    smudged_horizontal_reflection(&pattern)
}

fn smudged_horizontal_reflection(pattern: &[&str]) -> usize {
    for y in 1..pattern.len() {
        let mut smudge_found = false;
        let (mut i, mut j) = (y - 1, y);

        loop {
            if pattern[i] != pattern[j] {
                if check_for_smudge(pattern[i], pattern[j]) {
                    smudge_found = true;
                } else {
                    break;
                }
            }

            if i == 0 || j == pattern.len() - 1 {
                if smudge_found {
                    return y;
                }
                break;
            }

            i -= 1;
            j += 1;
        }
    }

    0
}

fn check_for_smudge(a: &str, b: &str) -> bool {
    let a = a.as_bytes();
    let b = b.as_bytes();

    let mut diff_count = 0;
    for i in 0..a.len() {
        if a[i] != b[i] {
            diff_count += 1;
        }
    }

    diff_count == 1
}

fn vertical_reflection(pattern: &[&str]) -> usize {
    let rotated = rotate(pattern);
    let pattern = rotated
        .iter()
        .map(|line| line.as_str())
        .collect::<Vec<&str>>();

    horizontal_reflection(&pattern)
}

fn horizontal_reflection(pattern: &[&str]) -> usize {
    for y in 1..pattern.len() {
        let mut is_reflection = true;
        let (mut i, mut j) = (y - 1, y);

        loop {
            if pattern[i] != pattern[j] {
                is_reflection = false;
                break;
            }

            if i == 0 || j == pattern.len() - 1 {
                break;
            }

            i -= 1;
            j += 1;
        }

        if is_reflection {
            return y;
        }
    }

    0
}

fn rotate(pattern: &[&str]) -> Vec<String> {
    let width = pattern.len();
    let height = pattern[0].len();

    let mut rotated = Grid::new(width, height, 'O');

    for (y, line) in pattern.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let rx = width - y - 1;
            let ry = x;
            rotated[(rx, ry)] = ch;
        }
    }

    let mut lines = vec![];
    for y in 0..height {
        lines.push(String::from_iter(&rotated[y * width..width + y * width]));
    }

    lines
}

#[cfg(test)]
mod tests {

    use std::str::FromStr;

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_reflection_summery() {
        let input = [
            "#.##..##.",
            "..#.##.#.",
            "##......#",
            "##......#",
            "..#.##.#.",
            "..##..##.",
            "#.#.##.#.",
            "",
            "#...##..#",
            "#....#..#",
            "..##..###",
            "#####.##.",
            "#####.##.",
            "..##..###",
            "#....#..#",
            "",
            ".#.##.#.#",
            ".##..##..",
            ".#.##.#..",
            "#......##",
            "#......##",
            ".#.##.#..",
            ".##..##.#",
            "",
            "#..#....#",
            "###..##..",
            ".##.#####",
            ".##.#####",
            "###..##..",
            "#..#....#",
            "#..##...#",
            "",
            "#.##..##.",
            "..#.##.#.",
            "##..#...#",
            "##...#..#",
            "..#.##.#.",
            "..##..##.",
            "#.#.##.#.",
        ];

        let expected = (709, 1400);
        let actual = reflection_summery(&input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_reflection_summery_1() {
        let input = [
            "###.##.##",
            "##.####.#",
            "##.#..#.#",
            "####..###",
            "....##...",
            "##.#..#.#",
            "...#..#..",
            "##..###.#",
            "##......#",
            "##......#",
            "..#.##.#.",
            "...#..#..",
            "##.####.#",
            "....##...",
            "...####..",
            "....##...",
            "##.####.#",
        ];

        let expected = (1, 5);
        let actual = reflection_summery(&input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_reflection_summery_2() {
        let input = [
            ".##.##...##...##.",
            "#####..##..##..##",
            ".....##..##..##..",
            ".##.#.#.####.#.#.",
            ".##...#.#..#.#...",
            "....#..........#.",
            "#..#..#......#..#",
            "....###.....####.",
            ".##...#.#..#.#...",
            ".....#..####..#..",
            "#..#...##..##...#",
            "....#...#..#...#.",
            "#..#.##########.#",
            "#..##...####...##",
            "#####.##.##.##.##",
        ];

        let expected = (2, 10);
        let actual = reflection_summery(&input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_reflection_summery_3() {
        #[rustfmt::skip]
        let input = [
            "##..#.#",
            "....##.",
            "##...##",
            "#...###",
            "..#.###",
            ".##..#.",
            "...#.#.",
            "...#.#.",
            ".#...#.",
            ".#...#.",
            "...#.#.",
            "...#.#.",
            ".##..#.",
        ];

        let expected = (900, 1100);
        let actual = reflection_summery(&input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_horizontal_reflection() {
        let pattern = [
            "#...##..#",
            "#....#..#",
            "..##..###",
            "#####.##.",
            "#####.##.",
            "..##..###",
            "#....#..#",
        ];

        let expected = 4;
        let actual = horizontal_reflection(&pattern);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_vertical_reflection() {
        let pattern = [
            "#.##..##.",
            "..#.##.#.",
            "##......#",
            "##......#",
            "..#.##.#.",
            "..##..##.",
            "#.#.##.#.",
        ];

        let expected = 5;
        let actual = vertical_reflection(&pattern);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_smudged_horizontal_reflection() {
        let pattern = [
            "#...##..#",
            "#....#..#",
            "..##..###",
            "#####.##.",
            "#####.##.",
            "..##..###",
            "#....#..#",
        ];

        let expected = 1;
        let actual = smudged_horizontal_reflection(&pattern);
        assert_eq!(actual, expected);

        let pattern = [
            "#.##..##.",
            "..#.##.#.",
            "##......#",
            "##......#",
            "..#.##.#.",
            "..##..##.",
            "#.#.##.#.",
        ];

        let expected = 3;
        let actual = smudged_horizontal_reflection(&pattern);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_rotate() {
        let pattern = [
            "#.##..##.",
            "..#.##.#.",
            "##......#",
            "##......#",
            "..#.##.#.",
            "..##..##.",
            "#.#.##.#.",
        ];

        let expected = vec![
            String::from_str("#..##.#").unwrap(),
            String::from_str("...##..").unwrap(),
            String::from_str("###..##").unwrap(),
            String::from_str(".#....#").unwrap(),
            String::from_str("#.#..#.").unwrap(),
            String::from_str("#.#..#.").unwrap(),
            String::from_str(".#....#").unwrap(),
            String::from_str("###..##").unwrap(),
            String::from_str("...##..").unwrap(),
        ];

        let actual = rotate(&pattern);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_check_for_smudge() {
        let a = "#.##..##.";
        let b = "..##..##.";

        let expected = true;
        let actual = check_for_smudge(a, b);
        assert_eq!(actual, expected);

        let a = "#...##..#";
        let b = "#....#..#";

        let expected = true;
        let actual = check_for_smudge(a, b);
        assert_eq!(actual, expected);
    }
}
