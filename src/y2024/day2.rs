pub fn solve(input: &[&str]) -> String {
    let part1 = solve_part_1(input);
    let part2 = solve_part_2(input);

    format!(" Part1: {} \n Part2: {}", part1, part2)
}

fn solve_part_1(input: &[&str]) -> usize {
    input
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|n| {
                    n.parse::<i32>()
                        .expect(&format!("Could not parse {n} as i32"))
                })
                .collect::<Vec<i32>>()
        })
        .map(|report| is_safe(&report))
        .filter(|r| *r)
        .count()
}

fn is_safe(report: &[i32]) -> bool {
    if report.len() < 2 {
        return true;
    }

    let direction = {
        let diff = report[0] - report[1];
        if diff < 0 {
            -1
        } else if diff > 0 {
            1
        } else {
            return false;
        }
    };

    for (i, n2) in report.iter().enumerate().skip(1) {
        let n1 = report[i - 1];
        let diff = n1 - n2;

        if diff == 0 {
            return false;
        }

        if diff.abs() > 3 {
            return false;
        }

        if direction > 0 && diff < 0 {
            return false;
        }

        if direction < 0 && diff > 0 {
            return false;
        }
    }

    true
}

fn solve_part_2(input: &[&str]) -> usize {
    input
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|n| {
                    n.parse::<i32>()
                        .expect(&format!("Could not parse {n} as i32"))
                })
                .collect::<Vec<i32>>()
        })
        .map(|report| is_safe_with_dampner(&report))
        .filter(|r| *r)
        .count()
}

fn is_safe_with_dampner(report: &[i32]) -> bool {
    // dbg!(&report);
    if is_safe(report) {
        // eprintln!("safe");
        return true;
    } else {
        for i in 0..report.len() {
            let mut damped = Vec::new();
            damped.extend_from_slice(&report[0..i]);
            damped.extend_from_slice(&report[i + 1..]);

            if is_safe(&damped) {
                // eprintln!("safe by damping {i}");
                return true;
            }
        }
    }

    // eprintln!("unsafe");
    false
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_solve_part_1() {
        #[rustfmt::skip]
        let input = [
            "7 6 4 2 1",
            "1 2 7 8 9",
            "9 7 6 2 1",
            "1 3 2 4 5",
            "8 6 4 4 1",
            "1 3 6 7 9",
        ];

        let expected = 2;
        let actual = solve_part_1(&input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_solve_part_2() {
        #[rustfmt::skip]
        let input = [
            "7 6 4 2 1",
            "1 2 7 8 9",
            "9 7 6 2 1",
            "1 3 2 4 5",
            "8 6 4 4 1",
            "1 3 6 7 9",
        ];

        let expected = 4;
        let actual = solve_part_2(&input);

        assert_eq!(actual, expected);
    }
}
