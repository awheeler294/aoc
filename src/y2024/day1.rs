use std::{collections::HashMap, iter::zip};

pub fn solve(input: &[&str]) -> String {
    let part1 = solve_part_1(input);
    let part2 = solve_part_2(input);

    format!(" Part1: {} \n Part2: {}", part1, part2)
}

fn solve_part_1(input: &[&str]) -> u32 {
    let mut list1 = vec![];
    let mut list2 = vec![];

    for line in input {
        let (left, right) = line
            .split_once(" ")
            .expect(&format!("Could not split {line} on whitespace"));
        list1.push(
            left.trim()
                .parse::<u32>()
                .expect(&format!("could not parse {left} as u32")),
        );
        list2.push(
            right
                .trim()
                .parse::<u32>()
                .expect(&format!("could not parse {right} as u32")),
        );
    }

    list1.sort();
    list2.sort();

    zip(list1, list2).fold(0, |acc, (l, r)| acc + l.abs_diff(r))
}

fn solve_part_2(input: &[&str]) -> u32 {
    let mut list1 = vec![];
    let mut list2 = HashMap::new();

    for line in input {
        let (left, right) = line
            .split_once(" ")
            .expect(&format!("Could not split {line} on whitespace"));
        list1.push(
            left.trim()
                .parse::<u32>()
                .expect(&format!("could not parse {left} as u32")),
        );

        let r_val = right
            .trim()
            .parse::<u32>()
            .expect(&format!("could not parse {right} as u32"));
        *list2.entry(r_val).or_insert(0) += 1;
    }

    list1
        .into_iter()
        .fold(0, |acc, val| acc + val * *(list2.get(&val).unwrap_or(&0)))
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_solve_part_1() {
        #[rustfmt::skip]
        let input = [
            "3   4",
            "4   3",
            "2   5",
            "1   3",
            "3   9",
            "3   3",
    ];

        let expected = 11;
        let actual = solve_part_1(&input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_solve_part_2() {
        #[rustfmt::skip]
        let input = [
            "3   4",
            "4   3",
            "2   5",
            "1   3",
            "3   9",
            "3   3",
    ];

        let expected = 31;
        let actual = solve_part_2(&input);
        assert_eq!(actual, expected);
    }
}
