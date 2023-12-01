use std::collections::VecDeque;

pub fn solve(input: &[&str]) -> String {
    let part1 = part1(input);
    let part2 = part2(input);

    format!(" Part1: {} \n Part2: {}", part1, part2)
}

fn part1(input: &[&str]) -> u32 {
    input
        .iter()
        .map(|s| {
            s.chars()
                .filter_map(|ch| ch.to_digit(10))
                .collect::<VecDeque<u32>>()
        })
        .map(|digits| {
            let first = digits.front().expect("Line had no first digit");
            let last = digits.back().expect("Line had no last digit");

            first * 10 + last
        })
        .sum()
}

fn part2(input: &[&str]) -> u32 {
    input
        .iter()
        .map(|d| parse_first_and_last_digit(&d))
        .map(|digits| digits.0 * 10 + digits.1)
        .sum()
}

fn parse_first_and_last_digit(line: &str) -> (u32, u32) {
    
    let first = 'scope: {
        let mut end = 1;
        loop {
            if let Some(digit) = parse_word_digit(&line[0..end]) {
                break 'scope digit;
            }

            let mut sweep = 0;
            while end - sweep >= 3 {
                if let Some(digit) = parse_word_digit(&line[sweep..end]) {
                    break 'scope digit;
                }

                sweep += 1;
            }

            end += 1;
        }
    };

    let last = 'scope: {
        let (mut start, end) = (line.len() - 1, line.len());
        loop {
            if let Some(digit) = parse_word_digit(&line[start..end]) {
                break 'scope digit;
            }

            let mut sweep = end;
            while sweep - start >= 3 {
                if let Some(digit) = parse_word_digit(&line[start..sweep]) {
                    break 'scope digit;
                }

                sweep -= 1;
            }

            start -= 1;
        }
    };

    (first, last)

}

fn parse_word_digit(input: &str) -> Option<u32> {
    if let Some(digit) = input.chars().last().unwrap().to_digit(10) {
        return Some(digit);
    }
    if let Some(digit) = input.chars().nth(0).unwrap().to_digit(10) {
        return Some(digit);
    }

    match input {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None,
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part1() {
        let input = ["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];

        let expected = 142;
        let actual = part1(&input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_first_last_1() {
        let input = "two1nine";
        let expected = (2, 9);
        let actual = parse_first_and_last_digit(&input);
       assert_eq!(actual, expected);
    }

    #[test]
    fn test_first_last_2() {
        let input = "eightwothree";
        let expected = (8, 3);
        let actual = parse_first_and_last_digit(&input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_first_last_3() {
        let input = "abcone2threexyz";
        let expected = (1, 3);
        let actual = parse_first_and_last_digit(&input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_first_last_4() {
        let input = "xtwone3four";
        let expected = (2, 4);
        let actual = parse_first_and_last_digit(&input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_first_last_5() {
        let input = "4nineeightseven2";
        let expected = (4, 2);
        let actual = parse_first_and_last_digit(&input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_first_last_6() {
        let input = "zoneight234";
        let expected = (1, 4);
        let actual = parse_first_and_last_digit(&input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_first_last_7() {
        let input = "7pqrstsixteen";
        let expected = (7, 6);
        let actual = parse_first_and_last_digit(&input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let input = [
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ];

        let expected = 281;
        let actual = part2(&input);

        assert_eq!(actual, expected);
    }

}
