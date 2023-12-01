pub fn solve(input: &[&str]) -> String {
    let part1 = count_floors(input[0]);
    let part2 = first_in_basement(input[0]);

    format!(" Part1: {} \n Part2: {}", part1, part2.unwrap())
}

fn count_floors(input: &str) -> i32 {
    let mut floor_num = 0;
    for ch in input.chars() {
        match ch {
            '(' => floor_num += 1,
            ')' => floor_num -= 1,
            _ => {}
        };
    }

    floor_num
}

fn first_in_basement(input: &str) -> Option<usize> {
    let mut floor_num = 0;
    for (i, ch) in input.chars().enumerate() {
        match ch {
            '(' => floor_num += 1,
            ')' => floor_num -= 1,
            _ => {}
        };
        if floor_num < 0 {
            return Some(i + 1);
        }
    }

    None
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_count_floors() {
        let input = "";
        let expected = 0;
        let actual = count_floors(input);

        assert_eq!(actual, expected);

        let input = "(";
        let expected = 1;
        let actual = count_floors(input);

        assert_eq!(actual, expected);

        let input = ")";
        let expected = -1;
        let actual = count_floors(input);

        assert_eq!(actual, expected);

        let input = "(())";
        let expected = 0;
        let actual = count_floors(input);

        assert_eq!(actual, expected);

        let input = "()()";
        let expected = 0;
        let actual = count_floors(input);

        assert_eq!(actual, expected);

        let input = "(((";
        let expected = 3;
        let actual = count_floors(input);

        assert_eq!(actual, expected);

        let input = "(()(()(";
        let expected = 3;
        let actual = count_floors(input);

        assert_eq!(actual, expected);

        let input = "))(((((";
        let expected = 3;
        let actual = count_floors(input);

        assert_eq!(actual, expected);

        let input = "())";
        let expected = -1;
        let actual = count_floors(input);

        assert_eq!(actual, expected);

        let input = "))(";
        let expected = -1;
        let actual = count_floors(input);

        assert_eq!(actual, expected);

        let input = ")))";
        let expected = -3;
        let actual = count_floors(input);

        assert_eq!(actual, expected);

        let input = ")())())";
        let expected = -3;
        let actual = count_floors(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_first_in_basement() {
        let input = "";
        let expected = None;
        let actual = first_in_basement(input);

        assert_eq!(actual, expected);

        let input = ")";
        let expected = Some(1);
        let actual = first_in_basement(input);

        assert_eq!(actual, expected);

        let input = "()())";
        let expected = Some(5);
        let actual = first_in_basement(input);

        assert_eq!(actual, expected);
    }
}
