pub fn solve(input: &[&str]) -> String {
    let part_1 = solve_part_1(input);
    let part_2 = solve_part_2(input);

    format!(" Part1: {part_1} \n Part2: {part_2}")
}

fn solve_part_1(input: &[&str]) -> u32 {
    fn solve(input: &[&str], dial_position: i32) -> u32 {
        let count = if dial_position % 100 == 0 { 1 } else { 0 };

        // eprintln!("");
        // dbg!(dial_position);

        if input.len() == 0 {
            return count;
        }

        let n: i32 = if let Some(n) = input[0].strip_prefix('L') {
            -1 * n
                .parse::<i32>()
                .unwrap_or_else(|e| panic!("unable to parse `{}`: {e}", input[0]))
        } else if let Some(n) = input[0].strip_prefix('R') {
            n.parse::<i32>()
                .unwrap_or_else(|e| panic!("unable to parse `{}`: {e}", input[0]))
        } else {
            panic!("malformed input `{}`", input[0]);
        };

        count + solve(&input[1..], dial_position + n)
    }

    solve(input, 50)
}

fn solve_part_2(input: &[&str]) -> i32 {
    let mut position = 50;
    let mut zero_count = 0;

    for line in input {
        let distance: i32 = if let Some(n) = line.strip_prefix('L') {
            -1 * n
                .parse::<i32>()
                .unwrap_or_else(|e| panic!("unable to parse `{}`: {e}", line))
        } else if let Some(n) = line.strip_prefix('R') {
            n.parse::<i32>()
                .unwrap_or_else(|e| panic!("unable to parse `{}`: {e}", line))
        } else {
            panic!("malformed input `{}`", input[0]);
        };

        if distance >= 0 {
            zero_count += (position + distance) / 100;
        } else {
            zero_count += ((100 - position) % 100 + distance.abs()) / 100;
        }

        position = (position + distance).rem_euclid(100);
    }

    zero_count
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_solve_part_1() {
        #[rustfmt::skip]
      let input = [
         "L68",
         "L30",
         "R48",
         "L5",
         "R60",
         "L55",
         "L1",
         "L99",
         "R14",
         "L82",
      ];

        let expected = 3;

        let actual = solve_part_1(&input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_solve_part_2() {
        #[rustfmt::skip]
      let input = [
         "L68",
         "L30",
         "R48",
         "L5",
         "R60",
         "L55",
         "L1",
         "L99",
         "R14",
         "L82",
      ];

        let expected = 6;

        let actual = solve_part_2(&input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_solve_part_2_1000() {
        #[rustfmt::skip]
      let input = [
         "R1000",
      ];

        let expected = 10;

        let actual = solve_part_2(&input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_solve_part_2_1050() {
        #[rustfmt::skip]
      let input = [
         "L50",
         "R1000",
      ];

        let expected = 11;

        let actual = solve_part_2(&input);

        assert_eq!(actual, expected);
    }
}
