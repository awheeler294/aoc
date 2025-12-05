use std::collections::HashSet;

pub fn solve(input: &[&str]) -> String {
    let part1 = get_duplicates(input);
    let part2 = get_common_priorities(input);

    format!(" Part1: {} \n Part2: {}", part1, part2)
}

fn get_priority(item: char) -> u32 {
    if item.is_uppercase() {
        (item as u8 - b'A' + 1 + 26).into()
    } else {
        (item as u8 - b'a' + 1).into()
    }
}

fn get_duplicates(lines: &[&str]) -> u32 {
    lines
        .iter()
        .map(|line| {
            let dup = find_dup(line);
            get_priority(dup)
        })
        .sum()
}

fn find_dup(line: &str) -> char {
    let mid = line.chars().count() / 2;
    let (a, b) = line.split_at(mid);

    let mut item_map = HashSet::new();

    for item in a.chars() {
        item_map.insert(item);
    }

    for item in b.chars() {
        if item_map.get(&item).is_some() {
            return item;
        }
    }
    panic!(
        "Reached end of line '{}' without finding any duplicates\n a: {}, b: {}",
        line, a, b
    );
}

fn get_common_priorities(lines: &[&str]) -> u32 {
    lines.chunks(3).map(find_common).map(get_priority).sum()
}

fn find_common(collections: &[&str]) -> char {
    let mut common: HashSet<char> = collections.iter().next().unwrap().chars().collect();
    for collection in collections.iter().skip(1) {
        let items = collection.chars().collect::<HashSet<char>>();
        common = common.intersection(&items).copied().collect();
    }

    common.into_iter().next().unwrap()

    // panic!(
    //     "Could not find common items in {:#?}",
    //     collections
    // );
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_duplicates() {
        #[rustfmt::skip]
        let input = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ];

        let expected = 157;
        let actual = get_duplicates(&input);
        assert_eq!(
            actual, expected,
            "Got '{actual}' when expecting '{expected}' from calling get_duplicates"
        );
    }

    #[test]
    fn test_find_dup() {
        #[rustfmt::skip]
        let cases = vec![
            ("vJrwpWtwJgWrhcsFMMfFFhFp", 'p'),
            ("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL", 'L'),
            ("PmmdzqPrVvPwwTWBwg", 'P'),
            ("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn", 'v'),
            ("ttgJtRGJQctTZtZT", 't'),
            ("CrZsJsPPZsGzwwsLwLmpwMDw", 's'),
        ];

        for (input, expected) in cases {
            let actual = find_dup(input);
            assert_eq!(
                actual, expected,
                "Got '{actual}' when expecting '{expected}' from calling find_dup on '{input}'"
            )
        }
    }

    #[test]
    fn test_get_priority() {
        assert_eq!(get_priority('a'), 1);
        assert_eq!(get_priority('z'), 26);
        assert_eq!(get_priority('A'), 27);
        assert_eq!(get_priority('Z'), 52);
    }

    #[test]
    fn test_find_common() {
        #[rustfmt::skip]
        let input = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
        ];

        let expected = 'r';
        let actual = find_common(&input);
        assert_eq!(
            actual, expected,
            "Got '{actual}' when expecting '{expected}' from calling find_common"
        );

        #[rustfmt::skip]
        let input = vec![
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ];

        let expected = 'Z';
        let actual = find_common(&input);
        assert_eq!(
            actual, expected,
            "Got '{actual}' when expecting '{expected}' from calling find_common"
        );
    }

    #[test]
    fn test_get_common_priorities() {
        #[rustfmt::skip]
        let input = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ];

        let expected = 70;
        let actual = get_common_priorities(&input);

        assert_eq!(
            actual, expected,
            "Got '{actual}' when expecting '{expected}' from calling get_common_priorities"
        );
    }
}
