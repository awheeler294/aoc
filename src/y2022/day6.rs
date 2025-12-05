use std::collections::HashMap;

pub fn solve(input: &[&str]) -> String {
    let part1 = find_start_of_packet_marker(input[0]).unwrap();
    let part2 = find_start_of_message_marker(input[0]).unwrap();

    format!(" Part1: {} \n Part2: {}", part1, part2)
}

fn find_start_of_message_marker(datastream: &str) -> Option<usize> {
    find_marker(datastream, 14)
}

fn find_start_of_packet_marker(datastream: &str) -> Option<usize> {
    find_marker(datastream, 4)
}

fn find_marker(datastream: &str, marker_len: usize) -> Option<usize> {
    let mut char_counts: HashMap<char, usize> = HashMap::new();

    for (i, ch) in datastream.chars().enumerate() {
        if i > marker_len - 1 {
            let mut dup_count = 0;
            for count in char_counts.values() {
                if *count > 1 {
                    dup_count += 1;
                }
            }
            if dup_count == 0 {
                return Some(i);
            }

            let to_remove = datastream.chars().nth(i - marker_len).unwrap();
            char_counts.entry(to_remove).and_modify(|count| *count -= 1);
        }

        char_counts
            .entry(ch)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    None
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_find_start_of_packet_marker() {
        #[rustfmt::skip]
        let cases = vec![
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", Some(7)),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", Some(5)),
            ("nppdvjthqldpwncqszvftbrmjlhg", Some(6)),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", Some(10)),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", Some(11)),
        ];

        for (input, expected) in cases {
            let actual = find_start_of_packet_marker(input);

            assert_eq!(
                actual, expected,
                "\nGot `{:#?}` when expecting `{:#?}` from calling find_start_of_packet_marker on `{input}`",
                actual, expected
            );
        }
    }

    #[test]
    fn test_find_start_of_message_marker() {
        #[rustfmt::skip]
        let cases = vec![
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", Some(19)),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", Some(23)),
            ("nppdvjthqldpwncqszvftbrmjlhg", Some(23)),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", Some(29)),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", Some(26)),
        ];

        for (input, expected) in cases {
            let actual = find_start_of_message_marker(input);

            assert_eq!(
                actual, expected,
                "\nGot `{:#?}` when expecting `{:#?}` from calling find_start_of_message_marker on `{input}`",
                actual, expected
            );
        }
    }
}
