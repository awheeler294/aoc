pub fn solve(input: &[&str]) -> String {
    let dimensions = input
        .iter()
        .map(|ds| {
            ds.split('x')
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let part1 = calculate_wrapping_paper_amount(&dimensions);
    let part2 = calculate_ribbon_amount(&dimensions);

    format!(" Part1: {} \n Part2: {}", part1, part2)
}

fn calculate_wrapping_paper_amount(box_dimensions: &[Vec<usize>]) -> usize {
    let mut total = 0;

    for d in box_dimensions {
        let mut dimensions = d.clone();
        dimensions.sort();
        let (l, w, h) = (dimensions[0], dimensions[1], dimensions[2]);

        let area = 3 * l * w + 2 * w * h + 2 * h * l;

        total += area;
    }

    total
}

fn calculate_ribbon_amount(box_dimensions: &[Vec<usize>]) -> usize {
    let mut total = 0;

    for d in box_dimensions {
        let mut dimensions = d.clone();
        dimensions.sort();
        let (l, w, h) = (dimensions[0], dimensions[1], dimensions[2]);

        let wrap_amount = 2 * l + 2 * w;
        let bow_amount = l * w * h;

        total += wrap_amount + bow_amount;
    }

    total
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_calculate_wrapping_paper_amount() {
        let dimensions = vec![vec![2, 3, 4]];
        let expected = 58;
        let actual = calculate_wrapping_paper_amount(&dimensions);
        assert_eq!(expected, actual);

        let dimensions = vec![vec![1, 1, 10]];
        let expected = 43;
        let actual = calculate_wrapping_paper_amount(&dimensions);
        assert_eq!(expected, actual);

        let dimensions = vec![vec![4, 3, 2]];
        let expected = 58;
        let actual = calculate_wrapping_paper_amount(&dimensions);
        assert_eq!(expected, actual);

        let dimensions = vec![vec![1, 10, 1]];
        let expected = 43;
        let actual = calculate_wrapping_paper_amount(&dimensions);
        assert_eq!(expected, actual);

        let dimensions = vec![vec![1, 10, 1], vec![4, 3, 2]];
        let expected = 101;
        let actual = calculate_wrapping_paper_amount(&dimensions);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calculate_ribbon_amount() {
        let dimensions = vec![vec![2, 3, 4]];
        let expected = 34;
        let actual = calculate_ribbon_amount(&dimensions);
        assert_eq!(expected, actual);

        let dimensions = vec![vec![1, 1, 10]];
        let expected = 14;
        let actual = calculate_ribbon_amount(&dimensions);
        assert_eq!(expected, actual);

        let dimensions = vec![vec![4, 3, 2]];
        let expected = 34;
        let actual = calculate_ribbon_amount(&dimensions);
        assert_eq!(expected, actual);

        let dimensions = vec![vec![1, 10, 1]];
        let expected = 14;
        let actual = calculate_ribbon_amount(&dimensions);
        assert_eq!(expected, actual);

        let dimensions = vec![vec![1, 10, 1], vec![4, 3, 2]];
        let expected = 48;
        let actual = calculate_ribbon_amount(&dimensions);
        assert_eq!(expected, actual);
    }
}
