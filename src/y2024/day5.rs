use std::{cmp::Ordering, collections::HashMap, u32};

pub fn solve(input: &[&str]) -> String {
    let part1 = solve_part_1(input);
    let part2 = solve_part_2(input);

    format!(" Part1: {} \n Part2: {}", part1, part2)
}

fn solve_part_1(input: &[&str]) -> u32 {
    let (rules, updates) = parse_input(input);

    let mut sum = 0;

    for u in updates.iter().filter(|update| is_valid(update, &rules)) {
        let center_idx = u.len() / 2;
        sum += u[center_idx];
    }

    sum
}

fn solve_part_2(input: &[&str]) -> u32 {
    let (rules, mut updates) = parse_input(input);

    let mut sum = 0;

    for u in updates
        .iter_mut()
        .filter(|update| !is_valid(update, &rules))
    {
        u.sort_by(|a, b| {
            if let Some(rule) = rules.get(a) {
                if rule.contains(b) {
                    return Ordering::Less;
                }
            }

            Ordering::Equal
        });

        let center_idx = u.len() / 2;
        sum += u[center_idx];
    }

    sum
}

fn parse_input(input: &[&str]) -> (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>) {
    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();

    let mut input_iter = input.iter();

    while let Some(line) = input_iter.next() {
        if *line == "" {
            break;
        }

        let (l, r) = line.split_once('|').unwrap();
        let l = l.parse::<u32>().unwrap();
        let r = r.parse::<u32>().unwrap();

        rules
            .entry(l)
            .and_modify(|predicates| predicates.push(r))
            .or_insert(vec![r]);
    }

    let mut updates = vec![];

    while let Some(line) = input_iter.next() {
        updates.push(
            line.split(',')
                .map(|val| val.parse::<u32>().unwrap())
                .collect(),
        );
    }

    (rules, updates)
}

fn is_valid(update: &Vec<u32>, rules: &HashMap<u32, Vec<u32>>) -> bool {
    for (i, val) in update.iter().enumerate() {
        if let Some(rule) = rules.get(val) {
            for n in update.iter().take(i) {
                for r in rule {
                    if n == r {
                        return false;
                    }
                }
            }
        }
    }

    return true;
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_solve_part_1() {
        let input = [
            "47|53",
            "97|13",
            "97|61",
            "97|47",
            "75|29",
            "61|13",
            "75|53",
            "29|13",
            "97|29",
            "53|29",
            "61|53",
            "97|53",
            "61|29",
            "47|13",
            "75|47",
            "97|75",
            "47|61",
            "75|61",
            "47|29",
            "75|13",
            "53|13",
            "",
            "75,47,61,53,29",
            "97,61,53,29,13",
            "75,29,13",
            "75,97,47,61,53",
            "61,13,29",
            "97,13,75,29,47",
        ];

        let expected = 143;

        let actual = solve_part_1(&input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_solve_part_2() {
        let input = [
            "47|53",
            "97|13",
            "97|61",
            "97|47",
            "75|29",
            "61|13",
            "75|53",
            "29|13",
            "97|29",
            "53|29",
            "61|53",
            "97|53",
            "61|29",
            "47|13",
            "75|47",
            "97|75",
            "47|61",
            "75|61",
            "47|29",
            "75|13",
            "53|13",
            "",
            "75,47,61,53,29",
            "97,61,53,29,13",
            "75,29,13",
            "75,97,47,61,53",
            "61,13,29",
            "97,13,75,29,47",
        ];

        let expected = 123;

        let actual = solve_part_2(&input);

        assert_eq!(actual, expected);
    }
}
