use std::collections::HashSet;

use crate::grid_v1::Point;

pub fn solve(input: &[&str]) -> String {
    let part1 = count_tail_visited(input, 2);
    let part2 = count_tail_visited(input, 10);

    format!(" Part1: {} \n Part2: {}", part1, part2)
}

fn count_tail_visited(motions: &[&str], rope_length: usize) -> usize {
    let mut rope = vec![Point::new(0, 0); rope_length];
    let mut visited = HashSet::new();
    // let mut v_grid = Grid::new(vec!['.'; 6 * 6], 6, 6);

    visited.insert(*rope.last().unwrap());
    // v_grid
    //     .set_at(tail.x.try_into().unwrap(), tail.y.try_into().unwrap(), '#')
    //     .unwrap();

    for motion in motions {
        let (direction, magnitude) = motion.split_once(' ').unwrap();
        let magnitude = magnitude.parse::<i32>().unwrap();
        let (dx, dy) = match direction {
            "L" => (-1, 0),
            "R" => (1, 0),
            "U" => (0, 1),
            "D" => (0, -1),
            _ => panic!("Could not parse direction from `{:#?}`", motion),
        };

        // dbg!(&head);
        // dbg!(&tail);
        // dbg!(&motion);
        for _ in 0..magnitude {
            rope.first_mut().unwrap().x += dx;
            rope.first_mut().unwrap().y += dy;

            for i in 1..rope.len() {
                let head = *rope.get_mut(i - 1).unwrap();
                let tail = rope.get_mut(i).unwrap();
                loop {
                    // println!();
                    // dbg!(&head);
                    // dbg!(&tail);
                    let x_diff = head.x - tail.x;
                    let y_diff = head.y - tail.y;
                    // dbg!((x_diff, y_diff));
                    if x_diff.abs() <= 1 && y_diff.abs() <= 1 {
                        break;
                    }
                    if tail.x != head.x {
                        if tail.x < head.x {
                            tail.x += 1;
                        } else {
                            tail.x -= 1;
                        }
                    }
                    if tail.y != head.y {
                        if tail.y < head.y {
                            tail.y += 1;
                        } else {
                            tail.y -= 1;
                        }
                    }

                    // if x_diff.abs() > 1 && y_diff.abs() > 0 {
                    //     tail.x += dx;
                    //     tail.y = head.y;
                    // } else if x_diff.abs() > 0 && y_diff.abs() > 1 {
                    //     tail.x = head.x;
                    //     tail.y += dy;
                    // } else {
                    //     if x_diff.abs() > 1 {
                    //         tail.x += dx;
                    //     }
                    //     if y_diff.abs() > 1 {
                    //         tail.y += dy;
                    //     }
                    // }
                }
            }

            // let x_offset: i32 = 11;
            // let y_offset: i32 = 15;
            // let mut position_grid = Grid::new(vec!['.'; 40 * 40], 40, 40);
            // position_grid
            //     .set_at(x_offset as usize, y_offset as usize, 's')
            //     .unwrap();
            //
            // for (i, point) in rope.iter().enumerate() {
            //     position_grid
            //         .set_at(
            //             (point.x + x_offset).try_into().unwrap(),
            //             (point.y + y_offset).try_into().unwrap(),
            //             char::from_digit(i as u32, 10).unwrap(),
            //         )
            //         .unwrap();
            // }
            //
            // position_grid
            //     .set_at(
            //         (rope.last().unwrap().x + x_offset).try_into().unwrap(),
            //         (rope.last().unwrap().y + y_offset).try_into().unwrap(),
            //         'T',
            //     )
            //     .unwrap();
            // position_grid
            //     .set_at(
            //         (rope.first().unwrap().x + x_offset).try_into().unwrap(),
            //         (rope.first().unwrap().y + y_offset).try_into().unwrap(),
            //         'H',
            //     )
            //     .unwrap();
            //
            // dbg!(position_grid);

            visited.insert(*rope.last().unwrap());
            // dbg!(&tail);
            // v_grid
            //     .set_at(tail.x.try_into().unwrap(), tail.y.try_into().unwrap(), '#')
            //     .unwrap();
        }
    }

    // dbg!(v_grid);
    visited.len()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_count_tail_visited() {
        #[rustfmt::skip]
        let motions = vec![
            "R 4",
            "U 4",
            "L 3",
            "D 1",
            "R 4",
            "D 1",
            "L 5",
            "R 2",
        ];

        let expected = 13;
        let actual = count_tail_visited(&motions, 2);
        assert_eq!(actual, expected, "\nGot `{actual}` when expecting `{expected}` from calling count_tail_visited on `{:#?}`.", motions);
    }

    #[test]
    fn test_count_tail_visited_lond() {
        #[rustfmt::skip]
        let motions = vec![
            "R 5",
            "U 8",
            "L 8",
            "D 3",
            "R 17",
            "D 10",
            "L 25",
            "U 20",
        ];

        let expected = 36;
        let actual = count_tail_visited(&motions, 10);
        assert_eq!(actual, expected, "\nGot `{actual}` when expecting `{expected}` from calling count_tail_visited on `{:#?}`.", motions);
    }
}
