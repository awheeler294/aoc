use regex::Regex;

pub fn solve(input: &[&str]) -> String {
    let part1 = solve_part_1(input);
    let part2 = solve_part_2(input);

    format!(" Part1: {} \n Part2: {}", part1, part2)
}

fn solve_part_1(input: &[&str]) -> usize {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let input = input.join(" ");

    let mut sum = 0;

    for (_, [lhs, rhs]) in re.captures_iter(&input).map(|c| c.extract()) {
        let lhs = lhs.parse::<usize>().unwrap();
        let rhs = rhs.parse::<usize>().unwrap();
        sum += lhs * rhs;
    }

    sum
}

fn solve_part_2(input: &[&str]) -> usize {
    let re =
        Regex::new(r"(?:mul\((?<lhs>\d+),(?<rhs>\d+)\))|(?<do>do\(\))|(?<dont>don't\(\))").unwrap();
    let input = input.join(" ");

    let mut enabled = true;
    let mut sum = 0;

    for cap in re.captures_iter(&input) {
        if cap.name("do").is_some() {
            enabled = true;
        } else if cap.name("dont").is_some() {
            enabled = false;
        } else if enabled {
            let lhs = cap["lhs"].parse::<usize>().unwrap();
            let rhs = cap["rhs"].parse::<usize>().unwrap();

            sum += lhs * rhs;
        }
    }

    sum
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_solve_part_1() {
        let input = ["xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"];

        let expected = 161;

        let actual = solve_part_1(&input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_solve_part_2() {
        let input = ["xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"];

        let expected = 48;

        let actual = solve_part_2(&input);

        assert_eq!(actual, expected);
    }
}
