use crate::grid::Grid;

pub fn solve(input: &[&str]) -> String {
    let part1 = sum_signal_strengths(input);
    let part2 = draw_crt(input);

    format!(" Part1: {} \n Part2: {:#?}", part1, part2)
}

fn run_instructions(instructions: &[&str]) -> Vec<i32> {
    let mut x = 1;
    let mut cycles = vec![x];

    for instruction in instructions {
        if instruction.starts_with("noop") {
            cycles.push(x);
        } else if instruction.starts_with("addx") {
            let (_, value) = instruction.split_once(' ').unwrap();
            let value = value.parse::<i32>().unwrap_or_else(|e| {
                panic!(
                    "Could not parse `{}` from `{}` as i32: {}",
                    value, instruction, e
                )
            });

            cycles.push(x);
            cycles.push(x);
            x += value;
        }
    }

    cycles.push(x);

    cycles
}

fn sum_signal_strengths(instructions: &[&str]) -> i32 {
    let x_values = run_instructions(instructions);

    x_values
        .into_iter()
        .enumerate()
        .map(|(i, x)| x * i as i32)
        .skip(20)
        .step_by(40)
        .sum()
}

fn draw_crt(instructions: &[&str]) -> Grid<char> {
    let crt_width = 40;
    let crt_height = 6;

    let mut pixels = vec!['.'; crt_width * crt_height];

    let x_values = run_instructions(instructions);

    for (clock, x) in x_values.iter().skip(1).enumerate() {
        let sprite = [*x - 1, *x, *x + 1];
        if sprite.contains(&(clock as i32 % 40)) {
            pixels[clock] = '#';
        }
    }

    Grid::new(pixels, crt_width, crt_height)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[rustfmt::skip]
    static LONG_INSTRUCTIONS: [&str; 146] = [
        "addx 15",
        "addx -11",
        "addx 6",
        "addx -3",
        "addx 5",
        "addx -1",
        "addx -8",
        "addx 13",
        "addx 4",
        "noop ",
        "addx -1",
        "addx 5",
        "addx -1",
        "addx 5",
        "addx -1",
        "addx 5",
        "addx -1",
        "addx 5",
        "addx -1",
        "addx -35",
        "addx 1",
        "addx 24",
        "addx -19",
        "addx 1",
        "addx 16",
        "addx -11",
        "noop",
        "noop",
        "addx 21",
        "addx -15",
        "noop",
        "noop",
        "addx -3",
        "addx 9",
        "addx 1",
        "addx -3",
        "addx 8",
        "addx 1",
        "addx 5",
        "noop",
        "noop",
        "noop",
        "noop",
        "noop",
        "addx -36",
        "noop",
        "addx 1",
        "addx 7",
        "noop",
        "noop",
        "noop",
        "addx 2",
        "addx 6",
        "noop",
        "noop",
        "noop",
        "noop",
        "noop",
        "addx 1",
        "noop",
        "noop",
        "addx 7",
        "addx 1",
        "noop",
        "addx -13",
        "addx 13",
        "addx 7",
        "noop",
        "addx 1",
        "addx -33",
        "noop",
        "noop",
        "noop",
        "addx 2",
        "noop",
        "noop",
        "noop",
        "addx 8",
        "noop",
        "addx -1",
        "addx 2",
        "addx 1",
        "noop",
        "addx 17",
        "addx -9",
        "addx 1",
        "addx 1",
        "addx -3",
        "addx 11",
        "noop",
        "noop",
        "addx 1",
        "noop",
        "addx 1",
        "noop",
        "noop",
        "addx -13",
        "addx -19",
        "addx 1",
        "addx 3",
        "addx 26",
        "addx -30",
        "addx 12",
        "addx -1",
        "addx 3",
        "addx 1",
        "noop",
        "noop",
        "noop",
        "addx -9",
        "addx 18",
        "addx 1",
        "addx 2",
        "noop",
        "noop",
        "addx 9",
        "noop",
        "noop",
        "noop",
        "addx -1",
        "addx 2",
        "addx -37",
        "addx 1",
        "addx 3",
        "noop",
        "addx 15",
        "addx -21",
        "addx 22",
        "addx -6",
        "addx 1",
        "noop",
        "addx 2",
        "addx 1",
        "noop",
        "addx -10",
        "noop",
        "noop",
        "addx 20",
        "addx 1",
        "addx 2",
        "addx 2",
        "addx -6",
        "addx -11",
        "noop",
        "noop",
        "noop",
    ];

    #[test]
    fn test_run_instructions() {
        #[rustfmt::skip]
        let instructions = vec![
            "noop", 
            "addx 3", 
            "addx -5"
        ];

        let x_values = run_instructions(&instructions);
        for (i, val) in x_values.iter().enumerate() {
            println!("  {i}: {val}");
        }

        let results = vec![
            (x_values[0], 1),
            (x_values[1], 1),
            (x_values[2], 1),
            (x_values[3], 1),
            (x_values[4], 4),
            (x_values[5], 4),
            (x_values[6], -1),
        ];

        for (i, (actual, expected)) in results.iter().enumerate() {
            assert_eq!(
                actual, expected,
                "\n Got `{actual}` when expecting `{expected}` at cycle `{i}`"
            )
        }
    }

    #[test]
    fn test_run_instructions_long() {
        let results = run_instructions(&LONG_INSTRUCTIONS);
        for (i, val) in results.iter().enumerate() {
            println!("  {i}: {val}");
        }
        assert_eq!(results[20], 21);
        assert_eq!(results[60], 19);
        assert_eq!(results[100], 18);
        assert_eq!(results[140], 21);
        assert_eq!(results[180], 16);
        assert_eq!(results[220], 18);
    }

    #[test]
    fn test_sum_signal_strengths() {
        let expected = 13140;
        let actual = sum_signal_strengths(&LONG_INSTRUCTIONS);

        assert_eq!(
            actual, expected,
            "\n Got `{actual}` when expecting `{expected}` from calling sum_signal_strengths"
        );
    }

    // ##..##..##..##..##..##..##..##..##..##..
    // ###...###...###...###...###...###...###.
    // ####....####....####....####....####....
    // #####.....#####.....#####.....#####.....
    // ######......######......######......####
    // #######.......#######.......#######.....
    #[test]
    fn test_draw_crt() {
        #[rustfmt::skip]
        let expected = Grid::new(vec![
'#','#','.','.','#','#','.','.','#','#','.','.','#','#','.','.','#','#','.','.','#','#','.','.','#','#','.','.','#','#','.','.','#','#','.','.','#','#','.','.',
'#','#','#','.','.','.','#','#','#','.','.','.','#','#','#','.','.','.','#','#','#','.','.','.','#','#','#','.','.','.','#','#','#','.','.','.','#','#','#','.',
'#','#','#','#','.','.','.','.','#','#','#','#','.','.','.','.','#','#','#','#','.','.','.','.','#','#','#','#','.','.','.','.','#','#','#','#','.','.','.','.',
'#','#','#','#','#','.','.','.','.','.','#','#','#','#','#','.','.','.','.','.','#','#','#','#','#','.','.','.','.','.','#','#','#','#','#','.','.','.','.','.',
'#','#','#','#','#','#','.','.','.','.','.','.','#','#','#','#','#','#','.','.','.','.','.','.','#','#','#','#','#','#','.','.','.','.','.','.','#','#','#','#',
'#','#','#','#','#','#','#','.','.','.','.','.','.','.','#','#','#','#','#','#','#','.','.','.','.','.','.','.','#','#','#','#','#','#','#','.','.','.','.','.',
        ], 40, 6);
        let actual = draw_crt(&LONG_INSTRUCTIONS);

        assert_eq!(
            actual, expected,
            "\n Got `{:#?}` when expecting `{:#?}` from calling draw_crt.",
            actual, expected
        );
    }
}
