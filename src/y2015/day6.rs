use crate::grid_v1::Grid;

pub fn solve(input: &[&str]) -> String {
    let part1 = process_lights(input);
    let part2 = process_lights_v2(input);

    format!(" Part1: {} \n Part2: {}", part1, part2)
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum TokenType {
    Action,
    Turn,
    Sxy,
    Through,
    Exy,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum ActionType {
    Toggle,
    TurnOn,
    TurnOff,
    Error,
}

fn process_lights(instructions: &[&str]) -> usize {
    let mut light_grid = Grid::new(vec![false; 1000000], 1000, 1000);

    for instruction in instructions {
        let mut state = TokenType::Action;
        let mut action = ActionType::Error;
        let mut start_x = 0;
        let mut start_y = 0;
        let mut end_x = 0;
        let mut end_y = 0;

        for token in instruction.split(" ") {
            match state {
                TokenType::Action => match token {
                    "turn" => {
                        state = TokenType::Turn;
                    }
                    "toggle" => {
                        state = TokenType::Sxy;
                        action = ActionType::Toggle;
                    }
                    _ => {}
                },

                TokenType::Turn => match token {
                    "on" => {
                        state = TokenType::Sxy;
                        action = ActionType::TurnOn;
                    }
                    "off" => {
                        state = TokenType::Sxy;
                        action = ActionType::TurnOff;
                    }
                    _ => {}
                },

                TokenType::Sxy => {
                    let (x, y) = token.split_once(',').unwrap();
                    start_x = x.parse::<usize>().unwrap();
                    start_y = y.parse::<usize>().unwrap();
                    state = TokenType::Through;
                }

                TokenType::Through => {
                    state = TokenType::Exy;
                }

                TokenType::Exy => {
                    let (x, y) = token.split_once(',').unwrap();
                    end_x = x.parse::<usize>().unwrap();
                    end_y = y.parse::<usize>().unwrap();
                    state = TokenType::Through;
                }
            }
        }

        //dbg!(start_x);
        //dbg!(start_y);
        //dbg!(end_x);
        //dbg!(end_y);
        //dbg!(action);

        for x in start_x..=end_x {
            for y in start_y..=end_y {
                let idx = light_grid.xy_idx(x as i32, y as i32);
                light_grid.spaces[idx] = match action {
                    ActionType::Toggle => !light_grid.spaces[idx],
                    ActionType::TurnOn => true,
                    ActionType::TurnOff => false,
                    _ => light_grid.spaces[idx],
                }
            }
        }
    }

    light_grid.spaces.iter().filter(|state| **state).count()
}

fn process_lights_v2(instructions: &[&str]) -> u32 {
    let mut light_grid: Grid<u32> = Grid::new(vec![0; 1000000], 1000, 1000);

    for instruction in instructions {
        let mut state = TokenType::Action;
        let mut action = ActionType::Error;
        let mut start_x = 0;
        let mut start_y = 0;
        let mut end_x = 0;
        let mut end_y = 0;

        for token in instruction.split(" ") {
            match state {
                TokenType::Action => match token {
                    "turn" => {
                        state = TokenType::Turn;
                    }
                    "toggle" => {
                        state = TokenType::Sxy;
                        action = ActionType::Toggle;
                    }
                    _ => {}
                },

                TokenType::Turn => match token {
                    "on" => {
                        state = TokenType::Sxy;
                        action = ActionType::TurnOn;
                    }
                    "off" => {
                        state = TokenType::Sxy;
                        action = ActionType::TurnOff;
                    }
                    _ => {}
                },

                TokenType::Sxy => {
                    let (x, y) = token.split_once(',').unwrap();
                    start_x = x.parse::<usize>().unwrap();
                    start_y = y.parse::<usize>().unwrap();
                    state = TokenType::Through;
                }

                TokenType::Through => {
                    state = TokenType::Exy;
                }

                TokenType::Exy => {
                    let (x, y) = token.split_once(',').unwrap();
                    end_x = x.parse::<usize>().unwrap();
                    end_y = y.parse::<usize>().unwrap();
                    state = TokenType::Through;
                }
            }
        }

        //dbg!(start_x);
        //dbg!(start_y);
        //dbg!(end_x);
        //dbg!(end_y);
        //dbg!(action);

        for x in start_x..=end_x {
            for y in start_y..=end_y {
                let idx = light_grid.xy_idx(x as i32, y as i32);
                light_grid.spaces[idx] = match action {
                    ActionType::Toggle => light_grid.spaces[idx] + 2,
                    ActionType::TurnOn => light_grid.spaces[idx] + 1,
                    ActionType::TurnOff => {
                        if let Some(v) = light_grid.spaces[idx].checked_sub(1) {
                            v
                        } else {
                            0
                        }
                    }
                    _ => light_grid.spaces[idx],
                }
            }
        }
    }

    light_grid.spaces.iter().sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_process_lights() {
        let instructions = vec!["turn on 0,0 through 999,999"];
        let expected = 1000000;
        let actual = process_lights(&instructions);
        assert_eq!(actual, expected);

        let instructions = vec!["toggle 0,0 through 999,0"];
        let expected = 1000;
        let actual = process_lights(&instructions);
        assert_eq!(actual, expected);

        let instructions = vec![
            "turn on 0,0 through 999,999",
            "turn off 499,499 through 500,500",
        ];
        let expected = 1000000 - 4;
        let actual = process_lights(&instructions);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_process_lights_v2() {
        let instructions = vec!["turn on 0,0 through 999,999"];
        let expected = 1000000;
        let actual = process_lights_v2(&instructions);
        assert_eq!(actual, expected);

        let instructions = vec!["toggle 0,0 through 999,0"];
        let expected = 2000;
        let actual = process_lights_v2(&instructions);
        assert_eq!(actual, expected);

        let instructions = vec![
            "turn on 0,0 through 999,999",
            "turn off 499,499 through 500,500",
        ];
        let expected = 1000000 - 4;
        let actual = process_lights_v2(&instructions);
        assert_eq!(actual, expected);
    }
}
