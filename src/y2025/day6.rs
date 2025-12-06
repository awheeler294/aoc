use std::iter::FromIterator;

pub fn solve(input: &[&str]) -> String {
    let part_1 = solve_part_1(input);
    let part_2 = solve_part_2(input);

    format!(" Part1: {part_1} \n Part2: {part_2}")
}

fn solve_part_1(input: &[&str]) -> u64 {
    let (columns, operations) = {
        let mut columns = vec![];
        let mut operations = vec![];

        for (l, line) in input.into_iter().enumerate() {
            for (c, value) in line.trim().split_ascii_whitespace().enumerate() {
                if l == 0 {
                    let num = value
                        .parse::<u64>()
                        .unwrap_or_else(|e| panic!("unable to parse number from {}: {}", value, e));
                    columns.push(vec![num]);
                } else if l == input.len() - 1 {
                    operations.push(value);
                } else {
                    let num = value
                        .parse::<u64>()
                        .unwrap_or_else(|e| panic!("unable to parse number from {}: {}", value, e));
                    columns[c].push(num);
                }
            }
        }

        (columns, operations)
    };

    let mut result = 0;
    for (i, column) in columns.into_iter().enumerate() {
        if operations[i] == "+" {
            result += column.into_iter().sum::<u64>();
        } else if operations[i] == "*" {
            result += column.into_iter().product::<u64>();
        }
    }

    result
}

fn solve_part_2(input: &[&str]) -> u64 {
    let (columns, operations) = {
        let mut columns = vec![];
        let operations = input
            .last()
            .unwrap_or_else(|| panic!("Empty input!"))
            .split_ascii_whitespace()
            .collect::<Vec<_>>();

        let input = input
            .iter()
            .take(input.len() - 1)
            .map(|line| Vec::from_iter(line.chars()))
            .collect::<Vec<Vec<char>>>();

        let mut c = input[0].len() - 1;

        let mut column = vec![];
        loop {
            let mut l = input.len() - 1;
            let mut num = 0;
            let mut power = 0;

            loop {
                if let Some(digit) = input[l][c].to_digit(10) {
                    num += digit as u64 * 10_u64.pow(power);
                    power += 1;
                }

                if l == 0 {
                    break;
                }

                l -= 1
            }

            if num > 0 {
                column.push(num);
            } else {
                columns.push(column);
                column = vec![];
            }

            if c == 0 {
                columns.push(column);
                break;
            }

            c -= 1;
        }

        columns.reverse();

        (columns, operations)
    };

    let mut result = 0;
    for (i, column) in columns.into_iter().enumerate() {
        if operations[i] == "+" {
            result += column.into_iter().sum::<u64>();
        } else if operations[i] == "*" {
            result += column.into_iter().product::<u64>();
        }
    }

    result
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_solve_part_1() {
        #[rustfmt::skip]
        let input = [
            "123 328  51 64 ",
            " 45 64  387 23 ",
            "  6 98  215 314",
            "*   +   *   +  ",
        ];

        let expected = 4277556;

        let actual = solve_part_1(&input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_solve_part_2() {
        #[rustfmt::skip]
        let input = [
            "123 328  51 64 ",
            " 45 64  387 23 ",
            "  6 98  215 314",
            "*   +   *   +  ",
        ];

        let expected = 3263827;

        let actual = solve_part_2(&input);

        assert_eq!(actual, expected);
    }
}
