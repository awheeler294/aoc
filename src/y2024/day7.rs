pub fn solve(input: &[&str]) -> String {
    let part1 = solve_part_1(input);
    let part2 = solve_part_2(input);

    format!(" Part1: {} \n Part2: {}", part1, part2)
}

fn solve_part_1(input: &[&str]) -> u64 {
    let mut total = 0;

    for line in input {
        let (value, rest) = line.split_once(':').unwrap();
        let value = value.parse::<u64>().unwrap();
        let operands = rest
            .split(' ')
            .skip(1)
            .map(|o| o.parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        if can_solve(value, &operands) {
            total += value;
        }
    }

    total
}

fn solve_part_2(input: &[&str]) -> u64 {
    fn can_solve_2(value: u64, acc: u64, operands: &[u64]) -> bool {
        // dbg!(acc);
        // dbg!(&operands);
        if acc == value && operands.len() == 0 {
            return true;
        }

        if acc > value {
            return false;
        }

        if let Some(next) = operands.iter().next() {
            // dbg!("+");
            if can_solve_2(value, acc + next, &operands[1..]) {
                return true;
            }

            // dbg!("*");
            if can_solve_2(value, acc * next, &operands[1..]) {
                return true;
            };

            // dbg!("||");
            if can_solve_2(value, concat_nums(acc, *next), &operands[1..]) {
                return true;
            };
        }

        false
    }

    let mut total = 0;

    for line in input {
        let (value, rest) = line.split_once(':').unwrap();
        let value = value.parse::<u64>().unwrap();
        let operands = rest
            .split(' ')
            .skip(1)
            .map(|o| o.parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        // dbg!(value);
        // dbg!(&operands);

        if can_solve_2(value, operands[0], &operands[1..]) {
            // println!("{value} true");
            total += value;
        }
    }

    total
}

fn can_solve(value: u64, operands: &[u64]) -> bool {
    can_solve_rec(value, operands[0], &operands[1..])
}

fn can_solve_rec(value: u64, acc: u64, operands: &[u64]) -> bool {
    if acc == value {
        return true;
    }

    if let Some(next) = operands.iter().next() {
        if can_solve_rec(value, acc + next, &operands[1..]) {
            return true;
        }

        if can_solve_rec(value, acc * next, &operands[1..]) {
            return true;
        };
    }

    false
}

fn concat_nums(a: u64, b: u64) -> u64 {
    // dbg!(a);
    // dbg!(b);
    // dbg!(b / 10 + 1);
    let mut e = 0;
    while 10_u64.pow(e) <= b {
        e += 1;
    }

    a * 10_u64.pow(e) + b
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_solve_part_1() {
        #[rustfmt::skip]
      let input = [
         "190: 10 19",
         "3267: 81 40 27",
         "83: 17 5",
         "156: 15 6",
         "7290: 6 8 6 15",
         "161011: 16 10 13",
         "192: 17 8 14",
         "21037: 9 7 18 13",
         "292: 11 6 16 20",
      ];

        let expected = 3749;

        let actual = solve_part_1(&input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_solve_part_2() {
        #[rustfmt::skip]
      let input = [
         "190: 10 19",
         "3267: 81 40 27",
         "83: 17 5",
         "156: 15 6",
         "7290: 6 8 6 15",
         "161011: 16 10 13",
         "192: 17 8 14",
         "21037: 9 7 18 13",
         "292: 11 6 16 20",
      ];

        let expected = 11387;

        let actual = solve_part_2(&input);

        assert_eq!(actual, expected);

        let input = ["924: 4 2 42 57 2 23"];

        // ['*', '||', '+', '+', '+']

        let expected = 924;

        let actual = solve_part_2(&input);

        assert_eq!(actual, expected);

        let input = ["125692: 11 4 89 28 2"];

        let expected = 0;

        let actual = solve_part_2(&input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_concat_nums() {
        for (a, b, expected) in [(8, 42, 842), (1, 100, 1100)] {
            let actual = concat_nums(a, b);

            assert_eq!(actual, expected);
        }
    }
}
