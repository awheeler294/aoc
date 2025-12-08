use crate::util::grid_v1::{Grid, GridDirections};

pub fn solve(input: &[&str]) -> String {
    let part1 = count_visible(input);
    let part2 = highest_senic_score(input);

    format!(" Part1: {} \n Part2: {}", part1, part2)
}

fn find_visible_trees(trees: &[&str]) -> Grid<usize> {
    let tree_grid: Grid<i32> = Grid::from_lines(trees);

    let mut visibility_grid = Grid::new(
        vec![0; tree_grid.width * tree_grid.height],
        tree_grid.width,
        tree_grid.height,
    );

    for x in 0..tree_grid.width {
        let mut highest = -1;
        for y in 0..tree_grid.height {
            let tree_height = tree_grid.at(x, y).unwrap();

            if *tree_height > highest {
                highest = *tree_height;
                visibility_grid.set_at(x, y, 1).unwrap();
            }
        }
    }

    for y in 0..tree_grid.width {
        let mut highest = -1;
        for x in 0..tree_grid.height {
            let tree_height = tree_grid.at(x, y).unwrap();

            if *tree_height > highest {
                highest = *tree_height;
                visibility_grid.set_at(x, y, 1).unwrap();
            }
        }
    }

    for x in (0..tree_grid.width).rev() {
        let mut highest = -1;
        for y in (0..tree_grid.height).rev() {
            let tree_height = tree_grid.at(x, y).unwrap();

            if *tree_height > highest {
                highest = *tree_height;
                visibility_grid.set_at(x, y, 1).unwrap();
            }
        }
    }

    for y in (0..tree_grid.width).rev() {
        let mut highest = -1;
        for x in (0..tree_grid.height).rev() {
            let tree_height = tree_grid.at(x, y).unwrap();

            if *tree_height > highest {
                highest = *tree_height;
                visibility_grid.set_at(x, y, 1).unwrap();
            }
        }
    }

    visibility_grid
}

fn highest_senic_score(trees: &[&str]) -> i32 {
    let tree_grid = Grid::from_lines(trees);

    let mut best_score = 0;

    for x in 1..tree_grid.width - 1 {
        for y in 1..tree_grid.height - 1 {
            let mut viewing_distances = vec![];

            let tree_height = *tree_grid.at(x, y).unwrap();

            for (dx, dy) in [
                GridDirections::UP,
                GridDirections::LEFT,
                GridDirections::DOWN,
                GridDirections::RIGHT,
            ] {
                let mut viewing_distance = 0;
                let (mut sx, mut sy) = (x as i32 + dx, y as i32 + dy);

                while tree_grid.is_in_bounds(sx, sy) {
                    viewing_distance += 1;
                    if *tree_grid.at(sx as usize, sy as usize).unwrap() < tree_height {
                        (sx, sy) = (sx + dx, sy + dy);
                    } else {
                        break;
                    }
                }

                viewing_distances.push(viewing_distance);
            }

            let senic_score = viewing_distances.iter().product();
            if senic_score > best_score {
                best_score = senic_score;
            }
        }
    }

    best_score
}

fn count_visible(input: &[&str]) -> usize {
    let visibility_grid = find_visible_trees(input);

    visibility_grid.spaces.iter().sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_highest_senic_score() {
        #[rustfmt::skip]
        let input = vec![
            "30373",
            "25512",
            "65332",
            "33549",
            "35390",
        ];

        let actual = highest_senic_score(&input);
        let expected = 8;

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_solve() {
        #[rustfmt::skip]
        let input = vec![
            "30373",
            "25512",
            "65332",
            "33549",
            "35390",
        ];

        let actual = count_visible(&input);
        let expected = 21;

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_find_visible_trees() {
        #[rustfmt::skip]
        let input = vec![
            "30373",
            "25512",
            "65332",
            "33549",
            "35390",
        ];

        #[rustfmt::skip]
        let spaces = vec![
            1, 1, 1, 1, 1,
            1, 1, 1, 0, 1,
            1, 1, 0, 1, 1,
            1, 0, 1, 0, 1,
            1, 1, 1, 1, 1,
        ];

        let expected = Grid::new(spaces, input.len(), input.len());
        let actual = find_visible_trees(&input);

        assert_eq!(
            actual, expected,
            "Got {:#?} when expecting {:#?}",
            actual, expected
        );
    }

    #[test]
    fn test_repeated_zeros() {
        #[rustfmt::skip]
        let input = vec![
            "30373",
            "00012",
            "65332",
            "33549",
            "35390",
        ];

        #[rustfmt::skip]
        let spaces = vec![
            1, 1, 1, 1, 1,
            1, 0, 0, 1, 1,
            1, 1, 0, 1, 1,
            1, 0, 1, 0, 1,
            1, 1, 1, 1, 1,
        ];

        let expected = Grid::new(spaces, input.len(), input.len());
        let actual = find_visible_trees(&input);

        assert_eq!(
            actual, expected,
            "Got {:#?} when expecting {:#?}",
            actual, expected
        );
    }

    // #[test]
    // fn test_find_visible_trees() {
    //     #[rustfmt::skip]
    //     let input = vec![
    //         "30373",
    //         "25512",
    //         "65332",
    //         "33549",
    //         "35390",
    //     ];
    //
    //     #[rustfmt::skip]
    //     let spaces = vec![
    //         true, true, true, true, true,
    //         true, true, true, false,true,
    //         true, true, false,true, true,
    //         true, false,true, false,true,
    //         true, true, true, true, true,
    //     ];
    //
    //     let expected = Grid::new(spaces, input.len(), input.len());
    //     let actual = find_visible_trees(&input);
    //
    //     assert_eq!(
    //         actual, expected,
    //         "Got {:#?} when expecting {:#?}",
    //         actual, expected
    //     );
    // }
    //
    // #[test]
    // fn test_repeated_zeros() {
    //     #[rustfmt::skip]
    //     let input = vec![
    //         "30373",
    //         "00012",
    //         "65332",
    //         "33549",
    //         "35390",
    //     ];
    //
    //     #[rustfmt::skip]
    //     let spaces = vec![
    //         true, true, true, true, true,
    //         true, true, true, true, true,
    //         true, true, false,true, true,
    //         true, false,true, false,true,
    //         true, true, true, true, true,
    //     ];
    //
    //     let expected = Grid::new(spaces, input.len(), input.len());
    //     let actual = find_visible_trees(&input);
    //
    //     assert_eq!(
    //         actual, expected,
    //         "Got {:#?} when expecting {:#?}",
    //         actual, expected
    //     );
    // }
}
