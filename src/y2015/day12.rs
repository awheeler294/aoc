use std::str::FromStr;

pub fn solve(input: &[&str]) -> String {
    let json = input[0];
    let part1 = sum_str(json);
    let part2 = sum_json(json, "red");

    format!(" Part1: {} \n Part2: {}", part1, part2)
}

fn sum_str(s: &str) -> i32 {
    let mut sum = 0;

    let mut acc = 0;
    let mut multiplier = 1;

    for ch in s.chars().rev() {
        if ch == '-' {
            acc *= -1;
        } else {
            if let Some(n) = ch.to_digit(10) {
                acc += multiplier * n as i32;
                multiplier *= 10;
            } else {
                sum += acc;
                acc = 0;
                multiplier = 1;
            }
        }
    }

    sum += acc;

    sum
}

fn sum_json(json: &str, exclude: &str) -> i32 {
    let mut sum = 0;
    let is_object = json.chars().nth(0).unwrap() == '{';

    let mut brace_count = 0;
    let mut quote_count = 0;
    let mut nums = Vec::new();
    let mut sign = 1;

    for (i, ch) in json[1..].chars().enumerate() {
        if brace_count == 0 {
            match ch {
                '[' | '{' => {
                    brace_count += 1;
                    sum += sum_json(&json[i + 1..], exclude);
                }

                ']' | '}' => return sum + nums_to_int(&nums, sign),

                '"' => {
                    if quote_count == 0 {
                        let string = parse_string_from_json(&json[i + 1..]);
                        if is_object && string == exclude {
                            return 0;
                        }
                        quote_count += 1;
                    } else {
                        quote_count -= 1;
                    }
                }

                _ => {
                    if ch == '-' {
                        sign = -1;
                    } else {
                        if let Some(n) = ch.to_digit(10) {
                            nums.push(n as i32);
                        } else {
                            if nums.len() > 0 {
                                sum += nums_to_int(&nums, sign);
                                nums.truncate(0);
                            }
                            sign = 1;
                        }
                    }
                }
            }
        } else {
            match ch {
                '[' | '{' => {
                    brace_count += 1;
                }

                ']' | '}' => {
                    brace_count -= 1;
                }

                _ => (),
            }
        }
    }

    sum
}

fn nums_to_int(nums: &[i32], sign: i32) -> i32 {
    let mut acc = 0;
    let mut multiplier = sign;
    for n in nums.iter().rev() {
        acc += n * multiplier;
        multiplier *= 10;
    }
    acc
}

fn parse_string_from_json(json: &str) -> String {
    let mut end_idx = 0;
    for (i, ch) in json[1..].chars().enumerate() {
        if ch == '"' {
            end_idx = i + 1;
            break;
        }
    }

    String::from_str(&json[1..end_idx]).unwrap()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_sum_str() {
        let s = "[1,2,3]";
        let expected = 6;
        let actual = sum_str(s);
        assert_eq!(actual, expected);

        let s = r#"{"a":2,"b":4}"#;
        let expected = 6;
        let actual = sum_str(s);
        assert_eq!(actual, expected);

        let s = r#"[[[3]]]"#;
        let expected = 3;
        let actual = sum_str(s);
        assert_eq!(actual, expected);

        let s = r#"{"a":{"b":4},"c":-1}"#;
        let expected = 3;
        let actual = sum_str(s);
        assert_eq!(actual, expected);

        let s = r#"{"a":[-1,1]}"#;
        let expected = 0;
        let actual = sum_str(s);
        assert_eq!(actual, expected);

        let s = r#"[-1,{"a":1}]"#;
        let expected = 0;
        let actual = sum_str(s);
        assert_eq!(actual, expected);

        let s = r#"[]"#;
        let expected = 0;
        let actual = sum_str(s);
        assert_eq!(actual, expected);

        let s = r#"{}"#;
        let expected = 0;
        let actual = sum_str(s);
        assert_eq!(actual, expected);

        let s = r#"[-14,22,313]"#;
        let expected = 321;
        let actual = sum_str(s);
        assert_eq!(actual, expected);

        let s = r#"[-14,22,-313]"#;
        let expected = -305;
        let actual = sum_str(s);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_sum_json() {
        let exclude = "red";

        let json = r#"[1,2,3]"#;
        let expected = 6;
        let actual = sum_json(json, exclude);
        assert_eq!(actual, expected);

        let json = r#"[1,{"c":"red","b":2},3]"#;
        let expected = 4;
        let actual = sum_json(json, exclude);
        assert_eq!(actual, expected);

        let json = r#"{"d":"red","e":[1,2,3,4],"f":5}"#;
        let expected = 0;
        let actual = sum_json(json, exclude);

        assert_eq!(actual, expected);
        let json = r#"[1,"red",5]"#;
        let expected = 6;
        let actual = sum_json(json, exclude);
        assert_eq!(actual, expected);
    }
}
