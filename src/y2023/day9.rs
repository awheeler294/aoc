pub fn solve(input: &[&str]) -> String {
    let part1 = extrapolated_values(&input);
    let part2 = extrapolated_values_backward(&input);

    format!(" Part1: {} \n Part2: {}", part1, part2)
}

fn extrapolated_values_backward(input: &[&str]) -> isize {
    input
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<isize>().unwrap())
                .rev()
                .collect::<Vec<_>>()
        })
        .map(|l| extrapolate_next_value(&l))
        .sum()
}

fn extrapolated_values(input: &[&str]) -> isize {
    input
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<isize>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|l| extrapolate_next_value(&l))
        .sum()
}

fn extrapolate_next_value(values: &[isize]) -> isize {
    if values.len() == 1 {
        return values[0];
    }

    let diffs = values[..]
        .windows(2)
        .map(|w| w[1] - w[0])
        .collect::<Vec<_>>();

    let last = values.last().unwrap();

    if diffs.iter().all(|n| *n == 0) {
        return *last;
    }

    last + extrapolate_next_value(&diffs)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_extrapolate_next_value() {
        let cases = [
            ([0, 3, 6, 9, 12, 15], 18),
            ([1, 3, 6, 10, 15, 21], 28),
            ([10, 13, 16, 21, 30, 45], 68),
            ([45, 30, 21, 16, 13, 10], 5),
        ];

        for (values, expected) in cases {
            let actual = extrapolate_next_value(&values);
            assert_eq!(
                actual, expected,
                "Got {actual} when expecting {expected} from calling extrapolate_next_value on {values:?}"
            );
        }
    }

    #[test]
    fn test_extrapolated_values() {
        #[rustfmt::skip]
        let input = [
            "0 3 6 9 12 15", 
            "1 3 6 10 15 21", 
            "10 13 16 21 30 45"
        ];

        let expected = 114;
        let actual = extrapolated_values(&input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_extrapolated_values_backwards() {
        #[rustfmt::skip]
        let input = [
            "0 3 6 9 12 15", 
            "1 3 6 10 15 21", 
            "10 13 16 21 30 45"
        ];

        let expected = 2;
        let actual = extrapolated_values_backward(&input);
        assert_eq!(actual, expected);
    }
}
