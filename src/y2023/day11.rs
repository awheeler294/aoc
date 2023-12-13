use crate::grid::Point;

pub fn solve(input: &[&str]) -> String {
    let part1 = path_sums(input, 2);
    let part2 = path_sums(input, 1_000_000);

    format!(" Part1: {} \n Part2: {}", part1, part2)
}

fn path_sums(input: &[&str], expansion_factor: usize) -> usize {

    let galaxies = parse_galaxies(input, expansion_factor);
    // dbg!(&galaxies);

    let mut pairs = vec![];
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            pairs.push((galaxies[i], galaxies[j]));
        }
    }

    pairs
        .iter()
        .fold(0, |acc, (a, b)| acc + a.manhattan_distance(*b))
}

fn parse_galaxies(input: &[&str], expansion_factor: usize) -> Vec<Point> {
    // dbg!(input);
    // dbg!(expansion_factor);
    let mut galaxies = vec![];

    for (y, line) in input.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                galaxies.push(Point::new(x, y));
            }
        }
    }

    if expansion_factor <= 1 {
        galaxies.sort_by_key(|galaxy| galaxy.x);
        return galaxies;
    }

    let mut y_expanded_galaxies = vec![];
    let mut expansions = 0;
    for i in 0..galaxies.len() {
        // dbg!(i);
        let gap = {
            if i == 0 {
                // dbg!(galaxies[i]);
                galaxies[i].y
            } else {
                // dbg!(galaxies[i]);
                // dbg!(galaxies[i - 1]);
                galaxies[i].y.saturating_sub(galaxies[i - 1].y + 1)
            }
        };
        // dbg!(gap);

        expansions += gap;
        // dbg!(expansions);

        let mut galaxy = galaxies[i].clone();
        galaxy.y = galaxy.y + (expansions * expansion_factor) - expansions;
        // dbg!(&galaxy);
        y_expanded_galaxies.push(galaxy);
    }

    y_expanded_galaxies.sort_by_key(|galaxy| galaxy.x);
    // dbg!(&galaxies);

    let mut expanded_galaxies = vec![];
    expansions = 0;
    for i in 0..y_expanded_galaxies.len() {
        // dbg!(i);
        let gap = {
            if i == 0 {
                y_expanded_galaxies[i].x
            } else {
                // dbg!(y_expanded_galaxies[i]);
                // dbg!(y_expanded_galaxies[i - 1]);
                y_expanded_galaxies[i]
                    .x
                    .saturating_sub(y_expanded_galaxies[i - 1].x + 1)
            }
        };

        expansions += gap;

        let mut galaxy = y_expanded_galaxies[i].clone();
        galaxy.x = galaxy.x + (expansions * expansion_factor) - expansions;
        expanded_galaxies.push(galaxy);
    }

    expanded_galaxies
}

#[cfg(test)]
mod tests {

    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_galaxies() {
        let input = [
            "...#......",
            ".......#..",
            "#.........",
            "..........",
            "......#...",
            ".#........",
            ".........#",
            "..........",
            ".......#..",
            "#...#.....",
        ];

        let mut expected = vec![
            Point { x: 3, y: 0 },
            Point { x: 7, y: 1 },
            Point { x: 0, y: 2 },
            Point { x: 6, y: 4 },
            Point { x: 1, y: 5 },
            Point { x: 9, y: 6 },
            Point { x: 7, y: 8 },
            Point { x: 0, y: 9 },
            Point { x: 4, y: 9 },
        ];
        expected.sort_by_key(|p| p.x);
        let actual = parse_galaxies(&input, 0);
        for i in 0..expected.len() {
            assert_eq!(actual[i], expected[i]);
        }
        assert_eq!(actual, expected);

        let expected = vec![
            Point { x: 0, y: 2 },
            Point { x: 0, y: 11 },
            Point { x: 1, y: 6 },
            Point { x: 4, y: 0 },
            Point { x: 5, y: 11 },
            Point { x: 8, y: 5 },
            Point { x: 9, y: 1 },
            Point { x: 9, y: 10 },
            Point { x: 12, y: 7 },
        ];

        let actual = parse_galaxies(&input, 2);
        for i in 0..expected.len() {
            assert_eq!(actual[i], expected[i]);
        }
        assert_eq!(actual, expected);

        let expected = {
            let mut expected = vec![
                Point { x: 5, y: 0 },   // 1
                Point { x: 11, y: 1 },  // 2
                Point { x: 0, y: 2 },   // 3
                Point { x: 10, y: 6 },  // 4
                Point { x: 1, y: 7 },   // 5
                Point { x: 15, y: 8 },  // 6
                Point { x: 11, y: 12 }, // 7
                Point { x: 0, y: 13 },  // 8
                Point { x: 6, y: 13 },  // 9
            ];
            expected.sort_by_key(|p| p.x);
            expected
        };
        let actual = parse_galaxies(&input, 3);
        for i in 0..expected.len() {
            assert_eq!(actual[i], expected[i]);
        }
        assert_eq!(actual, expected);

        let expected = vec![
            Point { x: 0, y: 2 },
            Point { x: 0, y: 27 },
            Point { x: 1, y: 14 },
            Point { x: 12, y: 0 },
            Point { x: 13, y: 27 },
            Point { x: 24, y: 13 },
            Point { x: 25, y: 1 },
            Point { x: 25, y: 26 },
            Point { x: 36, y: 15 },
        ];
        let actual = parse_galaxies(&input, 10);
        for i in 0..expected.len() {
            assert_eq!(actual[i], expected[i]);
        }
        assert_eq!(actual, expected);

        let galaxies = vec![
            Point { x: 0, y: 2 },
            Point { x: 0, y: 27 },
            Point { x: 1, y: 14 },
            Point { x: 12, y: 0 },
            Point { x: 13, y: 27 },
            Point { x: 24, y: 13 },
            Point { x: 25, y: 1 },
            Point { x: 25, y: 26 },
            Point { x: 36, y: 15 },
        ];
        let actual = {
            let mut pairs = vec![];
            for i in 0..galaxies.len() {
                for j in i + 1..galaxies.len() {
                    pairs.push((galaxies[i], galaxies[j]));
                }
            }

            pairs
                .iter()
                .fold(0, |acc, (a, b)| acc + a.manhattan_distance(*b))
        };

        let expected = 1030;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_path_sums() {
        let input = [
            "...#......",
            ".......#..",
            "#.........",
            "..........",
            "......#...",
            ".#........",
            ".........#",
            "..........",
            ".......#..",
            "#...#.....",
        ];

        let expected = 374;
        let actual = path_sums(&input, 2);
        assert_eq!(actual, expected);

        let expected = 1030;
        let actual = path_sums(&input, 10);
        assert_eq!(actual, expected);

        let expected = 8410;
        let actual = path_sums(&input, 100);
        assert_eq!(actual, expected);
    }
}
