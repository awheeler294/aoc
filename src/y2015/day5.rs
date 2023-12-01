use std::collections::HashMap;

pub fn solve(input: &[&str]) -> String {
    let mut nice_count = 0;
    for s in input {
        if is_nice(s) {
            nice_count += 1;
        }
    }

    let mut nice_count_v2 = 0;
    for s in input {
        if is_nice_v2(s) {
            nice_count_v2 += 1;
        }
    }

    format!(" Part1: {} \n Part2: {}", nice_count, nice_count_v2)
}

fn is_nice(s: &str) -> bool {
    let mut has_double_letter = false;
    let mut vowel_count = 0;

    for (i, ch) in s.chars().enumerate() {
        if let Some(next_char) = s.chars().nth(i + 1) {
            match ch {
                'a' => {
                    if next_char == 'b' {
                        return false;
                    }
                }
                'c' => {
                    if next_char == 'd' {
                        return false;
                    }
                }
                'p' => {
                    if next_char == 'q' {
                        return false;
                    }
                }
                'x' => {
                    if next_char == 'y' {
                        return false;
                    }
                }
                _ => {}
            }

            if next_char == ch {
                has_double_letter = true;
            }
        }
        if i < s.len() {}

        match ch {
            'a' | 'e' | 'i' | 'o' | 'u' => vowel_count += 1,
            _ => {}
        }
    }

    vowel_count >= 3 && has_double_letter
}

fn is_nice_v2(s: &str) -> bool {
    let mut pairs: HashMap<(char, char), Vec<usize>> = HashMap::new();

    let mut has_repeating_character = false;

    for (i, ch) in s.chars().enumerate() {
        if let Some(repeat_chr) = s.chars().nth(i + 2) {
            if repeat_chr == ch {
                has_repeating_character = true;
            }
        }

        if let Some(next_char) = s.chars().nth(i + 1) {
            let pair = (ch, next_char);
            (*pairs.entry(pair).or_insert(Vec::new())).push(i);
        }
    }

    for p in pairs.values() {
        if p.len() > 1 {
            for i in 0..p.len() {
                for j in i + 1..p.len() {
                    if p[j] - p[i] >= 2 {
                        return has_repeating_character;
                    }
                }
            }
        }
    }

    dbg!(s);
    dbg!(has_repeating_character);

    false
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_is_nice_at_least_three_vowels() {
        let s = "ugknbfddgicrmopn";
        let expected = true;
        let actual = is_nice(s);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_is_nice_double_letters() {
        let s = "aaa";
        let expected = true;
        let actual = is_nice(s);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_is_naughty_no_double_letter() {
        let s = "jchzalrnumimnmhp";
        let expected = false;
        let actual = is_nice(s);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_is_naughty_ab() {
        let s = "haegwjzuvuyypabu";
        let expected = false;
        let actual = is_nice(s);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_is_naughty_cd() {
        let s = "haegwjzuvuyypcdu";
        let expected = false;
        let actual = is_nice(s);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_is_naughty_pq() {
        let s = "haegwjzuvuyyppqu";
        let expected = false;
        let actual = is_nice(s);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_is_naughty_xy() {
        let s = "haegwjzuvuyypxyu";
        let expected = false;
        let actual = is_nice(s);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_is_naughty_only_one_vowel() {
        let s = "dvszwmarrgswjxmb";
        let expected = false;
        let actual = is_nice(s);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_is_naughty_only_two_vowels() {
        let s = "dvszwmairrgswjxmb";
        let expected = false;
        let actual = is_nice(s);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_is_nice_v2() {
        let s = "qjhvhtzxzqqjkmpb";
        let expected = true;
        let actual = is_nice_v2(s);
        assert_eq!(expected, actual);

        let s = "qjhvhtzxzqqjkmpb";
        let expected = true;
        let actual = is_nice_v2(s);
        assert_eq!(expected, actual);
    }
}
