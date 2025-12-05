use std::{str::FromStr, usize};

pub fn solve(input: &[&str]) -> String {
    let part1 = total_combinations(&input);
    let part2 = total_combinations_unfolded(&input);

    format!(" Part1: {} \n Part2: {}", part1, part2)
}

fn total_combinations_unfolded(input: &[&str]) -> usize {
    input
        .iter()
        .map(|line| {
            let (conditions, counts) = line.split_once(' ').unwrap();
            let mut unfolded = String::from_str(&conditions).unwrap();
            unfolded.push('?');
            unfolded = unfolded.repeat(5);
            let _ = unfolded.pop();

            let counts = counts
                .split(',')
                .map(|d| d.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            count_arrangements(&unfolded, &counts.repeat(5))
        })
        .sum()
}

fn total_combinations(input: &[&str]) -> usize {
    input
        .iter()
        .map(|line| {
            let (conditions, counts) = line.split_once(' ').unwrap();
            let counts = counts
                .split(',')
                .map(|d| d.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            (conditions, counts)
        })
        .map(|(conditions, counts)| count_arrangements(conditions, &counts))
        .sum()
}

// copied from https://github.com/akaritakai/AdventOfCode2023/blob/128d0f06c2d3cb67bfad63a599aea92e4bbf7ccf/src/day12.rs#L54
fn count_arrangements(line: &str, counts: &[usize]) -> usize {
    let line = line.as_bytes();
    let n = line.len();
    let m = counts.len();
    let mut dp = &mut vec![vec![0; n + 1]; m + 1];
    let mut next_dp = &mut vec![vec![0; n + 1]; m + 1];

    dp[m][0] = 1;
    dp[m - 1][counts[m - 1]] = 1;

    for pos in (0..n).rev() {
        for group in 0..=m {
            let max_count = if group == m { 0 } else { counts[group] };
            for count in 0..=max_count {
                next_dp[group][count] = 0;
                if matches!(line[pos], b'#' | b'?') {
                    next_dp[group][count] += dp[group][count + 1];
                }
            }
            if matches!(line[pos], b'.' | b'?') {
                next_dp[group][0] += dp[group][0];
                if group < m {
                    next_dp[group][max_count] += dp[group + 1][0];
                }
            }
        }
        std::mem::swap(&mut dp, &mut next_dp);
    }

    dp[0][0]
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_total_combinations() {
        #[rustfmt::skip]
        let input = [
            "???.### 1,1,3",
            ".??..??...?##. 1,1,3",
            "?#?#?#?#?#?#?#? 1,3,1,6",
            "????.#...#... 4,1,1",
            "????.######..#####. 1,6,5",
            "?###???????? 3,2,1",
        ];

        let expected = 21;
        let actual = total_combinations(&input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_total_combinations_unfolded() {
        #[rustfmt::skip]
        let input = [
            "???.### 1,1,3",
            ".??..??...?##. 1,1,3",
            "?#?#?#?#?#?#?#? 1,3,1,6",
            "????.#...#... 4,1,1",
            "????.######..#####. 1,6,5",
            "?###???????? 3,2,1",
        ];

        let expected = 525152;
        let actual = total_combinations_unfolded(&input);
        assert_eq!(actual, expected);
    }
}
