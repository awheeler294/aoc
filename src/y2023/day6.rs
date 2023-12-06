use std::{ops::Range, usize};

pub fn solve(input: &[&str]) -> String {
    let part1 = record_beating(&Race::parse_races(input));
    let part2 = record_beating(&vec![Race::parse_race(input)]);

    format!(" Part1: {} \n Part2: {}", part1, part2)
}

fn record_beating(races: &Vec<Race>) -> usize {
    races.iter().map(|race| race.beating_count()).product()
}

struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    fn parse_race(race_data: &[&str]) -> Self {
        let race_data = race_data
            .iter()
            .map(|r| {
                r.split_whitespace()
                    .skip(1)
                    .map(|v| v.trim())
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap()
            })
            .collect::<Vec<_>>();

        Self {
            time: race_data[0],
            distance: race_data[1],
        }
    }

    fn parse_races(race_data: &[&str]) -> Vec<Self> {
        let race_data = race_data
            .iter()
            .map(|r| {
                r.split_whitespace()
                    .skip(1)
                    .map(|v| v.trim().parse().unwrap())
                    .collect::<Vec<usize>>()
            })
            .collect::<Vec<_>>();

        race_data[0]
            .iter()
            .zip(race_data[1].iter())
            .map(|(t, d)| Self {
                time: *t,
                distance: *d,
            })
            .collect::<Vec<_>>()
    }

    fn beating_count(&self) -> usize {
        calculate_bounds(self.time as f64, self.distance as f64).len()
    }
}

fn calculate_bounds(time: f64, distance: f64) -> Range<usize> {
    // use quadratic equation to find upper and lower solutions to the current record
    let lower_bound = ((-1.0 * time + (time * time - 4.0 * distance).powf(0.5)) / -2.0).ceil();
    let upper_bound = ((-1.0 * time - (time * time - 4.0 * distance).powf(0.5)) / -2.0).floor();

    // move upper and lower bounds so they beat (don't overlap with) the current record
    let lower = {
        let mut lower = lower_bound;
        while lower * (time - lower) <= distance {
            lower += 1.0;
        }

        lower as usize
    };

    let upper = {
        let mut upper = upper_bound;
        while upper * (time - upper) <= distance {
            upper -= 1.0;
        }

        upper as usize
    };

    lower..upper + 1
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_calculate_bounds() {
        let time = 7.0;
        let distance = 9.0;

        let expected = 4;
        let bounds = calculate_bounds(time, distance);
        dbg!(&bounds);
        assert_eq!(bounds.len(), expected);

        let time = 15.0;
        let distance = 40.0;

        let expected = 8;
        let bounds = calculate_bounds(time, distance);
        dbg!(&bounds);
        assert_eq!(bounds.len(), expected);

        let time = 30.0;
        let distance = 200.0;

        let expected = 9;
        let bounds = calculate_bounds(time, distance);
        dbg!(&bounds);
        assert_eq!(bounds.len(), expected);
    }

    #[test]
    fn test_beating_count() {
        let race = Race {
            time: 7,
            distance: 9,
        };

        let expected = 4;
        let actual = race.beating_count();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_record_beating() {
        #[rustfmt::skip]
        let race_data = [
            "Time:      7  15   30",
            "Distance:  9  40  200",
        ];

        let expected = 288;
        let actual = record_beating(&Race::parse_races(&race_data));

        assert_eq!(actual, expected);

        let expected = 71503;
        let actual = record_beating(&vec![Race::parse_race(&race_data)]);

        assert_eq!(actual, expected);
    }
}
