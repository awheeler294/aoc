pub fn solve(input: &[&str]) -> String {
    let part1 = solve_part_1(input);
    let part2 = solve_part_2(input);

    format!(" Part1: {} \n Part2: {}", part1, part2)
}

fn solve_part_1(input: &[&str]) -> usize {
    input.len()
}

fn solve_part_2(input: &[&str]) -> usize {
    input.len()
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_solve_part_1() {
        let input = [];

        let expected = 0;

        let actual = solve_part_1(&input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_solve_part_2() {
        let input = [];

        let expected = 0;

        let actual = solve_part_2(&input);

        assert_eq!(actual, expected);
    }
}
