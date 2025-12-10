use anyhow::{Result, anyhow};

const WINNING_POINTS: u32 = 6;
const DRAW_POINTS: u32 = 3;
const LOOSING_POINTS: u32 = 0;

#[derive(Copy, Clone, PartialEq, Debug)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    fn from_chr(ch: char) -> Result<RPS> {
        match ch {
            'A' | 'X' => Ok(RPS::Rock),
            'B' | 'Y' => Ok(RPS::Paper),
            'C' | 'Z' => Ok(RPS::Scissors),
            _ => Err(anyhow!("Could not convert '{}' to an RPS", ch)),
        }
    }

    fn shape_score(shape: RPS) -> u32 {
        match shape {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }

    fn score_round(a: RPS, b: RPS) -> u32 {
        if a == b {
            return DRAW_POINTS;
        }

        match a {
            RPS::Rock => match b {
                RPS::Scissors => WINNING_POINTS,
                _ => LOOSING_POINTS,
            },
            RPS::Paper => match b {
                RPS::Rock => WINNING_POINTS,
                _ => LOOSING_POINTS,
            },
            RPS::Scissors => match b {
                RPS::Paper => WINNING_POINTS,
                _ => LOOSING_POINTS,
            },
        }
    }

    fn get_shape(other: RPS, outcome: char) -> Result<RPS> {
        match outcome {
            // Loose
            'X' => match other {
                RPS::Rock => Ok(RPS::Scissors),
                RPS::Paper => Ok(RPS::Rock),
                RPS::Scissors => Ok(RPS::Paper),
            },
            // Draw
            'Y' => Ok(other),
            // Win
            'Z' => match other {
                RPS::Rock => Ok(RPS::Paper),
                RPS::Paper => Ok(RPS::Scissors),
                RPS::Scissors => Ok(RPS::Rock),
            },
            _ => Err(anyhow!("get_shape: unknown outcome '{}'", outcome)),
        }
    }
}

pub fn solve(input: &[&str]) -> String {
    let part1 = score_rps1(input);
    let part2 = score_rps2(input);

    format!(" Part1: {} \n Part2: {}", part1, part2)
}

fn score_rps1(lines: &[&str]) -> u32 {
    lines
        .iter()
        .map(|line| {
            let (l, r) = line.split_once(' ').unwrap();
            let ls = RPS::from_chr(l.chars().next().unwrap()).unwrap();
            let rs = RPS::from_chr(r.chars().next().unwrap()).unwrap();
            let shape_score = RPS::shape_score(rs);
            let round_score = RPS::score_round(rs, ls);

            shape_score + round_score
        })
        .sum::<u32>()
}

fn score_rps2(lines: &[&str]) -> u32 {
    lines
        .iter()
        .map(|line| {
            let (l, r) = line.split_once(' ').unwrap();
            let ls = RPS::from_chr(l.chars().next().unwrap()).unwrap();
            let rs = RPS::get_shape(ls, r.chars().next().unwrap()).unwrap();
            let shape_score = RPS::shape_score(rs);
            let round_score = RPS::score_round(rs, ls);

            // dbg!(l);
            // dbg!(r);
            // dbg!(ls);
            // dbg!(rs);
            // dbg!(shape_score);
            // dbg!(round_score);
            // dbg!(shape_score + round_score);
            shape_score + round_score
        })
        .sum::<u32>()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_score_rps1() {
        #[rustfmt::skip]
        let input = vec![
            "A Y", 
            "B X", 
            "C Z"
        ];

        let actual = score_rps1(&input);
        let expected = 15;

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_shape() {
        assert_eq!(RPS::get_shape(RPS::Rock, 'X').unwrap(), RPS::Scissors);
        assert_eq!(RPS::get_shape(RPS::Paper, 'X').unwrap(), RPS::Rock);
        assert_eq!(RPS::get_shape(RPS::Scissors, 'X').unwrap(), RPS::Paper);

        assert_eq!(RPS::get_shape(RPS::Rock, 'Y').unwrap(), RPS::Rock);
        assert_eq!(RPS::get_shape(RPS::Paper, 'Y').unwrap(), RPS::Paper);
        assert_eq!(RPS::get_shape(RPS::Scissors, 'Y').unwrap(), RPS::Scissors);

        assert_eq!(RPS::get_shape(RPS::Rock, 'Z').unwrap(), RPS::Paper);
        assert_eq!(RPS::get_shape(RPS::Paper, 'Z').unwrap(), RPS::Scissors);
        assert_eq!(RPS::get_shape(RPS::Scissors, 'Z').unwrap(), RPS::Rock);
    }

    #[test]
    fn test_score_rps2() {
        #[rustfmt::skip]
        let input = vec![
            "A Y", 
            "B X", 
            "C Z"
        ];

        let actual = score_rps2(&input);
        let expected = 12;

        assert_eq!(actual, expected);
    }
}
