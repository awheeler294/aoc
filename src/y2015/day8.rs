pub fn solve(input: &[&str]) -> String {
    let (part1, part2) = count_chars_in_lines(input);

    format!(" Part1: {} \n Part2: {}", part1, part2)
}

fn count_chars_in_lines(lines: &[&str]) -> (usize, usize) {
    let mut char_count = 0;
    let mut total_count = 0;
    let mut encoded_count = 0;

    for line in lines {
        total_count += line.len();
        char_count += count_chars(line);
        encoded_count += count_escape_chars(line);
    }

    dbg!(char_count);
    dbg!(total_count);
    dbg!(encoded_count);
    (total_count - char_count, encoded_count - total_count)
}

fn count_chars(line: &str) -> usize {
    let mut count = 0;

    let mut i = 1;
    loop {
        if i >= line.len() - 1 {
            break;
        }

        let token = line.chars().nth(i).unwrap();
        match token {
            '\\' => {
                i += 1;
                if let Some(ch) = line.chars().nth(i) {
                    match ch {
                        'x' => {
                            i += 2;
                            count += 1;
                        }

                        _ => {
                            count += 1;
                        }
                    }
                }
            }
            _ => {
                count += 1;
            }
        }

        i += 1;
    }

    count
}

fn count_escape_chars(line: &str) -> usize {
    let mut count = 2; // for the surrounding double quotes
    for ch in line.chars() {
        match ch {
            '\"' | '\\' => {
                count += 2;
            }
            _ => count += 1,
        }
    }

    count
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_count_chars() {
        let lines = vec!["\"\"", "\"abc\"", "\"aaa\\\"aaa\"", "\"\\x27\""];

        let expected = (12, 19);
        let actual = count_chars_in_lines(&lines);
        assert_eq!(actual, expected);

        let line = "\"\"";
        let expected = 6;
        let actual = count_escape_chars(line);
        assert_eq!(actual, expected);

        let line = "\"abc\"";
        let expected = 9;
        let actual = count_escape_chars(line);
        assert_eq!(actual, expected);

        let line = "\"aaa\\\"aaa\"";
        let expected = 16;
        let actual = count_escape_chars(line);
        assert_eq!(actual, expected);

        let line = "\"\\x27\"";
        let expected = 11;
        let actual = count_escape_chars(line);
        assert_eq!(actual, expected);
    }
}
