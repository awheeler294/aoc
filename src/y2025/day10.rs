use core::panic;
use std::collections::{HashSet, VecDeque};
use z3::{Optimize, SatResult, ast::Int};

pub fn solve(input: &[&str]) -> String {
    let machines = {
        let mut machines = vec![];

        for line in input {
            machines.push(Machine::try_from(*line).unwrap_or_else(|e| panic!("{}", e)));
        }

        machines
    };

    let part_1 = solve_part_1(&machines);
    let part_2 = solve_part_2(&machines);

    format!(" Part1: {part_1} \n Part2: {part_2}")
}

fn solve_part_1(machines: &[Machine]) -> u32 {
    // dbg!(&machines);

    machines.iter().map(|m| m.calculate_fewest_presses()).sum()
}

fn solve_part_2(machines: &[Machine]) -> u64 {
    machines.iter().map(|m| m.configure_joltage()).sum()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Machine {
    target_state: u32,
    buttons: Vec<u32>,
    target_joltages: Vec<u32>,
    button_indexes: Vec<Vec<usize>>,
}

impl Machine {
    fn calculate_fewest_presses(&self) -> u32 {
        let mut visited = HashSet::new();

        let mut to_visit = VecDeque::from_iter(self.buttons.iter().map(|v| (1, *v)));

        while !to_visit.is_empty() {
            let (count, state) = to_visit.pop_front().unwrap();

            if state == self.target_state {
                return count;
            }

            for button in self.buttons.iter() {
                let new_state = state ^ button;
                if !visited.contains(&new_state) {
                    to_visit.push_back((count + 1, new_state));
                    visited.insert(new_state);
                }
            }
        }

        panic!("Could not find solution for {self:#?}")
    }

    fn configure_joltage(&self) -> u64 {
        let joltages = &self.target_joltages;
        let buttons = self.button_indexes.clone();

        let optimezer = Optimize::new();
        let total_presses = Int::fresh_const("total_presses");

        let button_presses = (0..buttons.len())
            .map(|i| Int::fresh_const(&format!("button_{i}")))
            .collect::<Vec<_>>();

        button_presses
            .iter()
            .for_each(|b| optimezer.assert(&b.ge(0)));

        for (pos, &target) in joltages.iter().enumerate() {
            let mut terms = Vec::new();

            for (i, btn) in buttons.iter().enumerate() {
                if btn.contains(&pos) {
                    terms.push(button_presses[i].clone());
                }
            }
            let sum = Int::add(&terms.iter().collect::<Vec<&Int>>());
            optimezer.assert(&sum.eq(Int::from_u64(target as u64)));
        }

        optimezer.assert(&total_presses.eq(Int::add(&button_presses)));
        optimezer.minimize(&total_presses);

        match optimezer.check(&[]) {
            SatResult::Sat => optimezer
                .get_model()
                .unwrap()
                .eval(&total_presses, true)
                .and_then(|t| t.as_u64())
                .unwrap(),
            _ => panic!("Could not find solution for {self:#?}"),
        }
    }
}

impl TryFrom<&str> for Machine {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut target_state = 0;
        let mut buttons = vec![];
        let mut target_joltages = vec![];
        let mut button_indexes = vec![];

        let mut bit_width = 0;

        for token in value.split_whitespace() {
            let mut chars = token.chars();

            match chars.next() {
                Some('[') => {
                    while let Some(ch) = chars.next() {
                        match ch {
                            '.' => {
                                target_state <<= 1;
                                bit_width += 1;
                            }

                            '#' => {
                                target_state = target_state << 1 | 1;
                                bit_width += 1;
                            }

                            ']' => {
                                continue;
                            }

                            _ => {
                                return Err(format!(
                                    "Parse error, could not parse {ch} as target state when parsing {value}"
                                ));
                            }
                        }
                    }
                }

                Some('(') => {
                    let mut acc = 0;
                    let mut nums = vec![];

                    while let Some(ch) = chars.next() {
                        match ch {
                            ',' => {
                                nums.push(acc as usize);
                                acc = 0;
                            }

                            ')' => {
                                nums.push(acc as usize);

                                let mut bits = vec![0; bit_width];
                                for idx in nums.iter() {
                                    bits[*idx] = 1;
                                }

                                let mut n = 0;
                                for bit in bits {
                                    n = n << 1 | bit;
                                }

                                buttons.push(n);
                                button_indexes.push(nums.clone());
                                // buttons.push(button.iter().fold(0, |acc, n| acc | (1 << n)));
                            }

                            _ => {
                                let digit = ch.to_digit(10).ok_or_else(|| {
                                    format!("Could not parse digit from {ch} when parsing {value}")
                                })?;
                                acc *= 10;
                                acc += digit;
                            }
                        }
                    }
                }

                Some('{') => {
                    let mut acc = 0;

                    while let Some(ch) = chars.next() {
                        match ch {
                            ',' => {
                                target_joltages.push(acc);
                                acc = 0;
                            }

                            '}' => {
                                target_joltages.push(acc);
                                acc = 0;
                            }
                            _ => {
                                let digit = ch.to_digit(10).ok_or_else(|| {
                                    format!("Could not parse digit from {ch} when parsing {value}")
                                })?;
                                acc *= 10;
                                acc += digit;
                            }
                        }
                    }
                }

                _ => {
                    return Err(format!(
                        "Parse error, could not parse {token} when parsing {value}"
                    ));
                }
            }
        }

        Ok(Self {
            target_state,
            buttons,
            target_joltages,
            button_indexes,
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_solve_part_1() {
        #[rustfmt::skip]
        let machines = [
            "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}",
            "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}",
            "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
        ].iter().map(|&line| Machine::try_from(line).unwrap()).collect::<Vec<_>>();

        let expected = 7;

        let actual = solve_part_1(&machines);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_solve_part_2() {
        #[rustfmt::skip]
        let machines = [
            "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}",
            "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}",
            "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
        ].iter().map(|&line| Machine::try_from(line).unwrap()).collect::<Vec<_>>();

        let expected = 33;

        let actual = solve_part_2(&machines);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_machine_from_str() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let actual = Machine::try_from(input).unwrap();
        let expected = Machine {
            target_state: 0b0110,
            //             (3)     (1,3)  (2)      (2,3)   (0,2)   (0,1)
            buttons: vec![0b0001, 0b0101, 0b0010, 0b0011, 0b1010, 0b1100],
            target_joltages: vec![3, 5, 4, 7],
            button_indexes: vec![
                vec![3],
                vec![1, 3],
                vec![2],
                vec![2, 3],
                vec![0, 2],
                vec![0, 1],
            ],
        };

        // eprintln!("{:04b}, ", expected.target_state);
        // eprintln!("{:04b}, ", actual.target_state);
        // for n in expected.buttons.iter() {
        //     eprint!("{n:04b}, ");
        // }
        // eprintln!();
        // for n in actual.buttons.iter() {
        //     eprint!("{n:04b}, ");
        // }
        // eprintln!();

        assert_eq!(actual, expected);

        let input = "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}";
        let actual = Machine::try_from(input).unwrap();
        let expected = Machine {
            target_state: 0b00010,
            //           (0,2,3,4) (2,3)    (0,4)    (0,1,2)  (1,2,3,4)
            buttons: vec![0b10111, 0b00110, 0b10001, 0b11100, 0b01111],
            target_joltages: vec![7, 5, 12, 7, 2],
            button_indexes: vec![
                vec![0, 2, 3, 4],
                vec![2, 3],
                vec![0, 4],
                vec![0, 1, 2],
                vec![1, 2, 3, 4],
            ],
        };

        // eprintln!("{:05b}, ", expected.target_state);
        // eprintln!("{:05b}, ", actual.target_state);
        // for n in expected.buttons.iter() {
        //     eprint!("{n:05b}, ");
        // }
        // eprintln!();
        // for n in actual.buttons.iter() {
        //     eprint!("{n:05b}, ");
        // }
        // eprintln!();

        assert_eq!(actual, expected);
    }
}
