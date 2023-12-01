use std::collections::HashSet;

pub fn solve(input: &[&str]) -> String {
    let part1 = houses_get_present(input[0]);
    let part2 = robo_santa(input[0]);

    format!(" Part1: {} \n Part2: {}", part1, part2)
}

fn houses_get_present(directions: &str) -> usize {
    let mut visited = HashSet::new();
    let mut x = 0;
    let mut y = 0;

    visited.insert((x, y));

    for d in directions.chars() {
        match d {
            '^' => {
                y -= 1;
            }
            'v' => {
                y += 1;
            }
            '>' => {
                x += 1;
            }
            '<' => x -= 1,
            _ => {}
        }

        visited.insert((x, y));
    }

    visited.len()
}

fn robo_santa(directions: &str) -> usize {
    let mut visited = HashSet::new();

    let mut sx = 0;
    let mut sy = 0;

    let mut rsx = 0;
    let mut rsy = 0;

    visited.insert((sx, sy));
    visited.insert((rsx, rsy));

    for (i, d) in directions.chars().enumerate() {
        if i % 2 != 0 {
            match d {
                '^' => {
                    sy -= 1;
                }
                'v' => {
                    sy += 1;
                }
                '>' => {
                    sx += 1;
                }
                '<' => sx -= 1,
                _ => {}
            }

            visited.insert((sx, sy));
        } else {
            match d {
                '^' => {
                    rsy -= 1;
                }
                'v' => {
                    rsy += 1;
                }
                '>' => {
                    rsx += 1;
                }
                '<' => rsx -= 1,
                _ => {}
            }

            visited.insert((rsx, rsy));
        }
    }

    visited.len()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_houses_get_present() {
        let directions = ">";
        let expected = 2;
        let actual = houses_get_present(directions);
        assert_eq!(actual, expected);

        let directions = "^>v<";
        let expected = 4;
        let actual = houses_get_present(directions);
        assert_eq!(actual, expected);

        let directions = "^v^v^v^v^v";
        let expected = 2;
        let actual = houses_get_present(directions);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_robo_santa() {
        let directions = "^v";
        let expected = 3;
        let actual = robo_santa(directions);
        assert_eq!(actual, expected);

        let directions = "^>v<";
        let expected = 3;
        let actual = robo_santa(directions);
        assert_eq!(actual, expected);

        let directions = "^v^v^v^v^v";
        let expected = 11;
        let actual = robo_santa(directions);
        assert_eq!(actual, expected);
    }
}
