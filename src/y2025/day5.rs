use core::panic;
use std::collections::HashSet;

pub fn solve(input: &[&str]) -> String {
    let part_1 = solve_part_1(input);
    let part_2 = solve_part_2(input);

    format!(" Part1: {part_1} \n Part2: {part_2}")
}

fn solve_part_1(input: &[&str]) -> usize {
    let (ranges, ids) = {
        let mut lines = input.iter();

        let mut ranges = vec![];

        while let Some(line) = lines.next() {
            if *line == "" {
                break;
            }

            let (start, end) = line
                .split_once('-')
                .unwrap_or_else(|| panic!("malformed line {}", line));
            let range = start
                .parse::<u64>()
                .unwrap_or_else(|e| panic!("could not parse `{}`: {}", start, e))
                ..=end
                    .parse::<u64>()
                    .unwrap_or_else(|e| panic!("could not parse `{}`: {}", end, e));
            ranges.push(range);
        }

        let mut ids = HashSet::new();
        while let Some(line) = lines.next() {
            let id = line
                .parse::<u64>()
                .unwrap_or_else(|e| panic!("could not parse `{}`: {e}", line));
            ids.insert(id);
        }

        (ranges, ids)
    };

    let mut fresh = 0;

    for id in ids {
        for range in ranges.iter() {
            if range.contains(&id) {
                fresh += 1;
                break;
            }
        }
    }

    fresh
}

#[derive(Debug)]
struct IdRange {
    start: u64,
    end: u64,
}

impl IdRange {
    fn new(start: u64, end: u64) -> Self {
        Self { start, end }
    }

    fn contains(&self, val: u64) -> bool {
        val >= self.start && val <= self.end
    }

    fn length(&self) -> u64 {
        (self.end - self.start) + 1
    }
}

fn solve_part_2(input: &[&str]) -> u64 {
    let mut ranges = vec![];

    for line in input {
        if *line == "" {
            break;
        }

        let (start, end) = line
            .split_once('-')
            .unwrap_or_else(|| panic!("malformed line {}", line));
        let range = IdRange::new(
            start
                .parse::<u64>()
                .unwrap_or_else(|e| panic!("could not parse `{}`: {}", start, e)),
            end.parse::<u64>()
                .unwrap_or_else(|e| panic!("could not parse `{}`: {}", end, e)),
        );
        ranges.push(range);
    }

    ranges.sort_by_key(|r| r.start);

    let mut merged_ranges: Vec<IdRange> = vec![];

    for range in ranges {
        if let Some(last_merged) = merged_ranges.last_mut() {
            if range.start > last_merged.end + 1 {
                merged_ranges.push(range);
            }
            else if (last_merged.contains(range.start) && !last_merged.contains(range.end))
                || (range.start == last_merged.end + 1)
            {
                last_merged.end = range.end;
            }
        } else {
            merged_ranges.push(range);
        }
    }

    // dbg!(&merged_ranges);

    let mut count = 0;

    for range in merged_ranges {
        count += range.length();
    }

    count
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_solve_part_1() {
        #[rustfmt::skip]
        let input = [
            "3-5",
            "10-14", 
            "16-20", 
            "12-18", 
            "", 
            "1", 
            "5", 
            "8", 
            "11", 
            "17", 
            "32",
        ];

        let expected = 3;

        let actual = solve_part_1(&input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_solve_part_1_full_range() {
        #[rustfmt::skip]
        let input = [
            "3-5", 
            "10-14", 
            "16-20", 
            "12-18", 
            "", 
            "1", 
            "2", 
            "3", 
            "4", 
            "5", 
            "6", 
            "7", 
            "8", 
            "9",
            "10", 
            "11", 
            "12", 
            "13", 
            "14", 
            "15", 
            "16", 
            "17", 
            "18", 
            "19", 
            "20", 
            "21", 
            "22", 
            "23",
            "24", 
            "25", 
            "26", 
            "27", 
            "28", 
            "29", 
            "30", 
            "31", 
            "32",
        ];

        let expected = 14;

        let actual = solve_part_1(&input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_solve_part_2() {
        #[rustfmt::skip]
        let input = [
            "3-5",
            "10-14", 
            "16-20", 
            "12-18", 
            "", 
            "1", 
            "5", 
            "8", 
            "11", 
            "17", 
            "32",
        ];

        let expected = 14;

        let actual = solve_part_2(&input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_solve_part_2_extended() {
        for (input, expected) in [
            (["3-5", "10-14", "16-20", "12-18", "", "1"], 14),
            (["1-1", "3-4", "4-8", "5-6", "", "1"], 7),
        ] {
            let actual = solve_part_2(&input);

            assert_eq!(actual, expected);
        }
    }
}
