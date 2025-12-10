#[derive(Debug, PartialEq, Eq)]
struct CraneInstruction {
    source: usize,
    destination: usize,
    amount: usize,
}

#[derive(Debug, PartialEq, Eq)]
struct Crane {
    stacks: Vec<Vec<char>>,
}

impl Crane {
    fn process_instruction(&mut self, instruction: &CraneInstruction) {
        for _ in 0..instruction.amount {
            let to_move = self.stacks[instruction.source - 1].pop().unwrap();
            self.stacks[instruction.destination - 1].push(to_move);
        }
    }

    fn process_instructions(&mut self, instructions: &[CraneInstruction]) {
        for instruction in instructions {
            self.process_instruction(instruction);
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct CrateMover9001 {
    stacks: Vec<Vec<char>>,
}

impl CrateMover9001 {
    fn process_instruction(&mut self, instruction: &CraneInstruction) {
        let mut to_move = Vec::new();

        for _ in 0..instruction.amount {
            to_move.push(self.stacks[instruction.source - 1].pop().unwrap());
        }

        for item in to_move.into_iter().rev() {
            self.stacks[instruction.destination - 1].push(item);
        }
    }

    fn process_instructions(&mut self, instructions: &[CraneInstruction]) {
        for instruction in instructions {
            self.process_instruction(instruction);
        }
    }
}

pub fn solve(input: &[&str]) -> String {
    let part1 = get_crane_results(input);
    let part2 = get_crate_mover_9001_results(input);

    format!(" Part1: {} \n Part2: {}", part1, part2)
}

fn get_crane_results(input: &[&str]) -> String {
    let (stacks, instructions) = parse_crane_data(input);
    let mut crane = Crane { stacks };
    crane.process_instructions(&instructions);

    let mut results = String::new();
    for stack in crane.stacks {
        results.push(*stack.last().unwrap_or(&'\0'));
    }

    results
}

fn get_crate_mover_9001_results(input: &[&str]) -> String {
    let (stacks, instructions) = parse_crane_data(input);
    let mut crane = CrateMover9001 { stacks };
    crane.process_instructions(&instructions);

    let mut results = String::new();
    for stack in crane.stacks {
        results.push(*stack.last().unwrap_or(&'\0'));
    }

    results
}

fn parse_crane_data(input: &[&str]) -> (Vec<Vec<char>>, Vec<CraneInstruction>) {
    let split_idx = (|| {
        for (i, line) in input.iter().enumerate() {
            if line.is_empty() {
                return i;
            }
        }
        panic!(
            "parse_crane_data got to the end of it's input without finding a blank line. There must be a blank line between the stack data and the crane data. input was: {:#?}",
            input
        );
    })();

    let stacks = parse_stacks(&input[..split_idx]);
    let crane_instructions = parse_crane_instructions(&input[split_idx + 1..]);

    (stacks, crane_instructions)
}

fn parse_stacks(stack_data: &[&str]) -> Vec<Vec<char>> {
    let mut stacks = Vec::new();

    for ch in stack_data.iter().last().unwrap().chars() {
        if ch.is_ascii_digit() {
            stacks.push(Vec::new());
        }
    }

    for line in stack_data.iter().rev().skip(1) {
        let mut stack_idx = 0;
        for (i, ch) in line.chars().enumerate() {
            if ch.is_alphabetic() {
                stacks[stack_idx].push(ch);
            }
            if i > 0 && i % 4 == 0 {
                stack_idx += 1;
            }
        }
    }

    stacks
}

fn parse_crane_instructions(instruction_data: &[&str]) -> Vec<CraneInstruction> {
    let mut instructions = Vec::new();

    for line in instruction_data {
        let mut words = line.split_whitespace();

        let amount = words.nth(1).unwrap().parse::<usize>().unwrap();
        let source = words.nth(1).unwrap().parse::<usize>().unwrap();
        let destination = words.nth(1).unwrap().parse::<usize>().unwrap();

        instructions.push(CraneInstruction {
            amount,
            source,
            destination,
        });
    }

    instructions
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_crate_mover_9001_results() {
        let input = vec![
            "    [D]    ",
            "[N] [C]    ",
            "[Z] [M] [P]",
            " 1   2   3 ",
            "",
            "move 1 from 2 to 1",
            "move 3 from 1 to 3",
            "move 2 from 2 to 1",
            "move 1 from 1 to 2",
        ];

        let expected = String::from("MCD");

        let actual = get_crate_mover_9001_results(&input);

        assert_eq!(
            actual, expected,
            "\nGot `{:#?}` when expecting `{:#?}` from calling get_crane_results on `{:#?}`",
            actual, expected, input
        );
    }

    #[test]
    fn test_crate_mover_9001_process_instruction() {
        let stacks = vec![vec!['Z', 'N', 'D'], vec!['M', 'C'], vec!['P']];
        let mut crane = CrateMover9001 {
            stacks: stacks.clone(),
        };

        let instruction = CraneInstruction {
            amount: 3,
            source: 1,
            destination: 3,
        };

        let expected = vec![vec![], vec!['M', 'C'], vec!['P', 'Z', 'N', 'D']];

        crane.process_instruction(&instruction);
        let actual = crane.stacks;

        assert_eq!(
            actual, expected,
            "\nGot `{:#?}` when expecting `{:#?}` from processing `{:#?}` on `{:#?}`",
            actual, expected, instruction, stacks
        );
    }

    #[test]
    fn test_get_crane_results() {
        let input = vec![
            "    [D]    ",
            "[N] [C]    ",
            "[Z] [M] [P]",
            " 1   2   3 ",
            "",
            "move 1 from 2 to 1",
            "move 3 from 1 to 3",
            "move 2 from 2 to 1",
            "move 1 from 1 to 2",
        ];

        let expected = String::from("CMZ");

        let actual = get_crane_results(&input);

        assert_eq!(
            actual, expected,
            "\nGot `{:#?}` when expecting `{:#?}` from calling get_crane_results on `{:#?}`",
            actual, expected, input
        );
    }

    #[test]
    fn test_parse_crane_data() {
        let input = vec![
            "    [D]    ",
            "[N] [C]    ",
            "[Z] [M] [P]",
            " 1   2   3 ",
            "",
            "move 1 from 2 to 1",
            "move 3 from 1 to 3",
            "move 2 from 2 to 1",
            "move 1 from 1 to 2",
        ];

        let expected = (
            vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']],
            vec![
                CraneInstruction {
                    amount: 1,
                    source: 2,
                    destination: 1,
                },
                CraneInstruction {
                    amount: 3,
                    source: 1,
                    destination: 3,
                },
                CraneInstruction {
                    amount: 2,
                    source: 2,
                    destination: 1,
                },
                CraneInstruction {
                    amount: 1,
                    source: 1,
                    destination: 2,
                },
            ],
        );

        let actual = parse_crane_data(&input);

        assert_eq!(
            actual, expected,
            "\nGot `{:#?}` when expecting `{:#?}` from calling parse_crane_data on `{:#?}`",
            actual, expected, input
        );
    }

    #[test]
    fn test_parse_crane_instructions() {
        let instruction_data = vec![
            "move 1 from 2 to 1",
            "move 3 from 1 to 3",
            "move 2 from 2 to 1",
            "move 1 from 1 to 2",
        ];

        let expected = vec![
            CraneInstruction {
                amount: 1,
                source: 2,
                destination: 1,
            },
            CraneInstruction {
                amount: 3,
                source: 1,
                destination: 3,
            },
            CraneInstruction {
                amount: 2,
                source: 2,
                destination: 1,
            },
            CraneInstruction {
                amount: 1,
                source: 1,
                destination: 2,
            },
        ];

        let actual = parse_crane_instructions(&instruction_data);

        assert_eq!(
            actual, expected,
            "\nGot `{:#?}` when expecting `{:#?}` from calling parse_crane_instructions on `{:#?}`",
            actual, expected, instruction_data
        );
    }

    #[test]
    fn test_parse_stacks() {
        #[rustfmt::skip]
        let cases = vec![
            (
                vec![
                    "    [D]    ", 
                    "[N] [C]    ", 
                    "[Z] [M] [P]", 
                    " 1   2   3 "
                ],
                vec![
                    vec!['Z', 'N'], 
                    vec!['M', 'C', 'D'], 
                    vec!['P']
                ],
            ),
            (
                vec![
                    "    [M]             [Z]     [V]    ",
                    "    [Z]     [P]     [L]     [Z] [J]",
                    "[S] [D]     [W]     [W]     [H] [Q]",
                    "[P] [V] [N] [D]     [P]     [C] [V]",
                    "[H] [B] [J] [V] [B] [M]     [N] [P]",
                    "[V] [F] [L] [Z] [C] [S] [P] [S] [G]",
                    "[F] [J] [M] [G] [R] [R] [H] [R] [L]",
                    "[G] [G] [G] [N] [V] [V] [T] [Q] [F]",
                    " 1   2   3   4   5   6   7   8   9 ",
                ],
                vec![
                    vec!['G', 'F', 'V', 'H', 'P', 'S'],
                    vec!['G', 'J', 'F', 'B', 'V', 'D', 'Z', 'M'],
                    vec!['G', 'M', 'L', 'J', 'N'],
                    vec!['N', 'G', 'Z', 'V', 'D', 'W', 'P'],
                    vec!['V', 'R', 'C', 'B'],
                    vec!['V', 'R', 'S', 'M', 'P', 'W', 'L', 'Z'],
                    vec!['T', 'H', 'P'],
                    vec!['Q', 'R', 'S', 'N', 'C', 'H', 'Z', 'V'],
                    vec!['F', 'L', 'G', 'P', 'V', 'Q', 'J'],
                ],
            ),
        ];

        for (stack_data, expected) in cases {
            let actual = parse_stacks(&stack_data);

            assert_eq!(
                actual, expected,
                "\nGot `{:#?}` when expecting `{:#?}` from calling parse_stacks on `{:#?}`",
                actual, expected, stack_data
            );
        }
    }
}
