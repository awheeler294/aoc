pub fn solve(input: &[&str]) -> String {
    let part1 = count_contained_intervals(input);
    let part2 = count_overlapping_intervals(input);

    format!(" Part1: {} \n Part2: {}", part1, part2)
}

fn count_contained_intervals(input: &[&str]) -> u32 {
    input
        .iter()
        .map(|intervals| match are_intervals_fully_contained(intervals) {
            Some(res) => u32::from(res),
            _ => 0,
        })
        .sum()
}

fn count_overlapping_intervals(input: &[&str]) -> u32 {
    input
        .iter()
        .map(|intervals| match do_intervals_overlap(intervals) {
            Some(res) => u32::from(res),
            _ => 0,
        })
        .sum()
}

fn are_intervals_fully_contained(intervals: &str) -> Option<bool> {
    let vals = intervals
        .split(&['-', ','])
        .map(|v| v.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let (a_start, a_end, b_start, b_end) = (vals[0], vals[1], vals[2], vals[3]);

    Some((a_start <= b_start && a_end >= b_end) || (b_start <= a_start && b_end >= a_end))
}

fn do_intervals_overlap(intervals: &str) -> Option<bool> {
    let vals = intervals
        .split(&['-', ','])
        .map(|v| v.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let (a_start, a_end, b_start, b_end) = (vals[0], vals[1], vals[2], vals[3]);

    Some(
        (a_start >= b_start && a_start <= b_end)
            || (a_end >= b_start && a_end <= b_end)
            || (b_start >= a_start && b_start <= a_end)
            || (b_end >= a_start && b_end <= a_end),
    )
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_do_intervals_overlap() {
        #[rustfmt::skip]
        let cases = vec![
            ("2-4,6-8", false),
            ("2-3,4-5", false),
            ("5-7,7-9", true),
            ("2-8,3-7", true),
            ("6-6,4-6", true),
            ("2-6,4-8", true),
            ("4-4,4-8", true),
            ("1-1,1-1", true),
            ("4-4,4-6", true),
            ("3-7,2-8", true),
            ("0-0,0-0", true),
            ("74-75,18-75", true),
            ("76-76,18-75", false),
        ];

        for (input, expected) in cases {
            let actual = do_intervals_overlap(input).unwrap();
            assert_eq!(actual, expected, "Got '{actual}' when expecting '{expected}' when calling do_intervals_overlap on '{input}'");
        }
    }

    #[test]
    fn test_are_intervals_fully_contained() {
        #[rustfmt::skip]
        let cases = vec![
            ("2-4,6-8", false),
            ("2-3,4-5", false),
            ("5-7,7-9", false),
            ("2-8,3-7", true),
            ("6-6,4-6", true),
            ("2-6,4-8", false),
            ("4-4,4-8", true),
            ("1-1,1-1", true),
            ("4-4,4-6", true),
            ("3-7,2-8", true),
            ("0-0,0-0", true),
            ("74-75,18-75", true),
            ("76-76,18-75", false),
        ];

        for (input, expected) in cases {
            let actual = are_intervals_fully_contained(input).unwrap();
            assert_eq!(actual, expected, "Got '{actual}' when expecting '{expected}' when calling are_intervals_fully_contained on '{input}'");
        }
    }

    #[test]
    fn test_count_contained_intervals() {
        #[rustfmt::skip]
        let intervals = vec![
            "2-4,6-8",
            "2-3,4-5",
            "5-7,7-9",
            "2-8,3-7",
            "6-6,4-6",
            "2-6,4-8",
        ];

        let expected = 2;
        let actual = count_contained_intervals(&intervals);
        assert_eq!(
            actual, expected,
            "Got {actual} when expecting {expected} when calling count_contained_intervals"
        );
    }
}
