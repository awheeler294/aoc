use crate::util::grid::{Grid, GridDirection, Point};

pub fn solve(input: &[&str]) -> String {
    let part_1 = solve_part_1(input);
    let part_2 = solve_part_2(input);

    format!(" Part1: {part_1} \n Part2: {part_2}")
}

fn solve_part_1(input: &[&str]) -> usize {
    let grid = Grid::parse_char(input);

    let mut total_rolls = 0;
    let mut inaccessible = 0;

    for (idx, val) in grid.iter().enumerate() {
        if *val == '@' {
            total_rolls += 1;

            let point = grid.idx_point(idx);

            let mut adj_count = 0;

            for direction in GridDirection::all() {
                if let Some(adjacent) = grid.get_direction(&point, direction) {
                    if *adjacent == '@' {
                        adj_count += 1;
                    }
                }

                if adj_count >= 4 {
                    inaccessible += 1;
                    break;
                }
            }
        }
    }

    total_rolls - inaccessible
}

fn solve_part_2(input: &[&str]) -> usize {
    let mut grid = Grid::parse_char(input);

    let mut total = 0;

    loop {
        let accessible = find_accessible(&grid);

        // for p in accessible.iter() {
        //    grid[p] = 'x';
        // }
        // dbg!(&grid);

        if accessible.len() == 0 {
            break;
        }

        total += accessible.len();

        for p in accessible.iter() {
            grid[p] = '.';
        }
    }

    total
}

fn find_accessible(grid: &Grid<char>) -> Vec<Point> {
    let mut accessible = vec![];

    for (idx, val) in grid.iter().enumerate() {
        if *val == '@' {
            let point = grid.idx_point(idx);

            let mut adj_count = 0;

            for direction in GridDirection::all() {
                if let Some(adjacent) = grid.get_direction(&point, direction) {
                    if *adjacent == '@' {
                        adj_count += 1;
                    }
                }

                if adj_count >= 4 {
                    break;
                }
            }

            if adj_count < 4 {
                accessible.push(point);
            }
        }
    }

    accessible
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_solve_part_1() {
        #[rustfmt::skip]
      let input = [
         "..@@.@@@@.",
         "@@@.@.@.@@",
         "@@@@@.@.@@",
         "@.@@@@..@.",
         "@@.@@@@.@@",
         ".@@@@@@@.@",
         ".@.@.@.@@@",
         "@.@@@.@@@@",
         ".@@@@@@@@.",
         "@.@.@@@.@.",
      ];

        let expected = 13;

        let actual = solve_part_1(&input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_solve_part_2() {
        #[rustfmt::skip]
      let input = [
         "..@@.@@@@.",
         "@@@.@.@.@@",
         "@@@@@.@.@@",
         "@.@@@@..@.",
         "@@.@@@@.@@",
         ".@@@@@@@.@",
         ".@.@.@.@@@",
         "@.@@@.@@@@",
         ".@@@@@@@@.",
         "@.@.@@@.@.",
      ];

        let expected = 43;

        let actual = solve_part_2(&input);

        assert_eq!(actual, expected);
    }
}
