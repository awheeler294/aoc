const PROHIBITED_CHARS: [char; 3] = ['i', 'o', 'l'];
const REQUIRED_PAIR_COUNT: usize = 2;
const REQUIRED_STRAIGHT_SIZE: usize = 3;
const REQUIRED_PASSWORD_LENGTH: usize = 8;

pub fn solve(input: &[&str]) -> String {
    let current_password = input[0];
    let part1 = generate_next_password(current_password);
    let part2 = generate_next_password(&part1);

    format!(" Part1: {} \n Part2: {}", part1, part2)
}

fn generate_next_password(current_password: &str) -> String {
    let mut next_password = increment_str(current_password);

    loop {
        if validate_password(&next_password) {
            break;
        } else {
            next_password = increment_str(&next_password);
        }
    }

    next_password
}

fn ch_to_num(ch: &char) -> usize {
    match ch {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        'h' => 7,
        'i' => 8,
        'j' => 9,
        'k' => 10,
        'l' => 11,
        'm' => 12,
        'n' => 13,
        'o' => 14,
        'p' => 15,
        'q' => 16,
        'r' => 17,
        's' => 18,
        't' => 19,
        'u' => 20,
        'v' => 21,
        'w' => 22,
        'x' => 23,
        'y' => 24,
        'z' => 25,
        _ => panic!("unknown character {}", ch),
    }
}

fn increment_str(s: &str) -> String {
    let mut nums = Vec::new();
    let mut carry = 1;

    for ch in s.chars().rev() {
        let n = carry + ch_to_num(&ch);
        nums.push(n % 26);
        carry = n / 26;
    }

    nums.iter()
        .rev()
        .map(|n| match n {
            0 => 'a',
            1 => 'b',
            2 => 'c',
            3 => 'd',
            4 => 'e',
            5 => 'f',
            6 => 'g',
            7 => 'h',
            8 => 'i',
            9 => 'j',
            10 => 'k',
            11 => 'l',
            12 => 'm',
            13 => 'n',
            14 => 'o',
            15 => 'p',
            16 => 'q',
            17 => 'r',
            18 => 's',
            19 => 't',
            20 => 'u',
            21 => 'v',
            22 => 'w',
            23 => 'x',
            24 => 'y',
            25 => 'z',
            _ => panic!("cannot convert {} to character", n),
        })
        .collect()
}

fn validate_password(password: &str) -> bool {
    // password length
    if password.len() != REQUIRED_PASSWORD_LENGTH {
        return false;
    }

    let mut pairs_count = 0;
    let mut next_pair_idx = 1;

    let mut straight_value = 0;
    let mut straight_size = 0;

    for (i, ch) in password.chars().enumerate() {
        // password must not contain prohibited characters
        if PROHIBITED_CHARS.contains(&ch) {
            return false;
        }

        if i == next_pair_idx {
            let previous_ch = password.chars().nth(i - 1).unwrap();
            if previous_ch == ch {
                pairs_count += 1;
                next_pair_idx = i + 2;
            } else {
                next_pair_idx += 1;
            }
        }

        let ch_value = ch_to_num(&ch);
        if ch_value == straight_value + 1 {
            straight_value = ch_value;
            straight_size += 1;
        } else {
            if straight_size < REQUIRED_STRAIGHT_SIZE {
                straight_value = ch_value;
                straight_size = 1;
            }
        }
    }

    // Passwords must contain at least two different, non-overlapping pairs
    // of letters, like aa, bb, or zz.
    if pairs_count < REQUIRED_PAIR_COUNT {
        return false;
    }

    // Passwords must include one increasing straight of at least three
    // letters, like abc, bcd, cde, and so on, up to xyz. They cannot skip
    // letters; abd doesn't count.
    if straight_size < REQUIRED_STRAIGHT_SIZE {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_increment_string() {
        let s = "xx";
        let expected = "xy".to_string();
        let actual = increment_str(s);
        assert_eq!(actual, expected);

        let s = "xy";
        let expected = "xz".to_string();
        let actual = increment_str(s);
        assert_eq!(actual, expected);

        let s = "xz";
        let expected = "ya".to_string();
        let actual = increment_str(s);
        assert_eq!(actual, expected);

        let s = "ya";
        let expected = "yb".to_string();
        let actual = increment_str(s);
        assert_eq!(actual, expected);

        let s = "zz";
        let expected = "aa".to_string();
        let actual = increment_str(s);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_validate_password() {
        let contains_l = "hajklmmn";
        let contains_i = "hajkimmn";
        let contains_o = "hajkommn";

        assert_eq!(validate_password(contains_l), false);
        assert_eq!(validate_password(contains_i), false);
        assert_eq!(validate_password(contains_o), false);

        let not_eight_characters = "hajkmmn";
        assert_eq!(validate_password(not_eight_characters), false);

        let no_pairs = "abcdfgab";
        assert_eq!(validate_password(no_pairs), false);

        let only_one_pair = "abcdffab";
        assert_eq!(validate_password(only_one_pair), false);

        let overlapping_pairs = "abcdfffb";
        assert_eq!(validate_password(overlapping_pairs), false);

        let valid_password = "abcdffaa";
        assert_eq!(validate_password(valid_password), true);

        let valid_password = "ghjaabcc";
        assert_eq!(validate_password(valid_password), true);
    }

    #[test]
    fn test_generate_next_password() {
        let current_password = "abcdefgh";
        let expected = "abcdffaa".to_string();
        let actual = generate_next_password(current_password);
        assert_eq!(actual, expected);

        //let current_password = "ghijklmn";
        //let expected = "ghjaabcc".to_string();
        //let actual = generate_next_password(current_password);
        //assert_eq!(actual, expected);
    }
}
