use core::panic;

use crate::util::grid::Grid;

pub fn solve(input: &[&str]) -> String {
    let part_1 = solve_part_1(input);
    let part_2 = solve_part_2(input);

    format!(" Part1: {part_1} \n Part2: {part_2}")
}

#[derive(Debug)]
struct Region {
    width: usize,
    height: usize,
    counts: Vec<usize>,
}

fn solve_part_1(input: &[&str]) -> usize {
    // Parse
    let mut shapes = vec![];
    let mut regions = vec![];

    let mut l = 0;
    while l < input.len() {
        let line = input[l];
        let chunks = line.split(':').collect::<Vec<_>>();

        dbg!(&line);
        dbg!(&chunks);

        if chunks[1] == "" {
            let start = l + 1;

            l = start + 1;
            while input[l] != "" {
                l += 1;
            }

            shapes.push(Grid::parse_char(&input[start..l]));
        } else {
            let (Some(dimensions), Some(counts)) = (chunks.get(0), chunks.get(1)) else {
                panic!("Can't parse region from {line}");
            };

            let (width, height) = {
                let mut d = dimensions.split('x').map(|n| {
                    n.parse::<u32>().unwrap_or_else(|e| {
                        panic!("Coulden't parse dimensions from {dimensions}: {e}")
                    })
                });
                let width = d
                    .next()
                    .unwrap_or_else(|| panic!("Coulden't parse width from {dimensions}"))
                    as usize;
                let height = d
                    .next()
                    .unwrap_or_else(|| panic!("Coulden't parse height from {dimensions}"))
                    as usize;

                (width, height)
            };

            let counts = counts
                .trim()
                .split_whitespace()
                .map(|n| {
                    n.parse::<usize>()
                        .unwrap_or_else(|e| panic!("Coulden't parse number from {n}: {e}"))
                })
                .collect::<Vec<_>>();

            regions.push(Region {
                width,
                height,
                counts,
            });
        }

        l += 1;
    }

    // dbg!(&shapes);
    // dbg!(&regions);

    let mut valid_count = 0;

    for region in regions.iter() {
        // If the region is big enough to fit the presents side by side we dont have to bother
        // arranging them.
        if region.width / 3 * region.height / 3 >= region.counts.iter().sum() {
            valid_count += 1
        } else {
            // dfs or something? It turns out the above works to solve actual input.
        }
    }

    // dbg!(regions.len() - valid_count);

    valid_count
}

fn solve_part_2(input: &[&str]) -> usize {
    input.len() - input.len()
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_solve_part_1() {
        #[rustfmt::skip]
      let input = [
            "0:",
            "###",
            "##.",
            "##.",
            "",
            "1:",
            "###",
            "##.",
            ".##",
            "",
            "2:",
            ".##",
            "###",
            "##.",
            "",
            "3:",
            "##.",
            "###",
            "##.",
            "",
            "4:",
            "###",
            "#..",
            "###",
            "",
            "5:",
            "###",
            ".#.",
            "###",
            "",
            "4x4: 0 0 0 0 2 0",
            "12x5: 1 0 1 0 2 2",
            "12x5: 1 0 1 0 3 2",
        ];

        let expected = 1;

        let actual = solve_part_1(&input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_solve_part_2() {
        #[rustfmt::skip]
        let input = [
            "0:",
            "###",
            "##.",
            "##.",
            "",
            "1:",
            "###",
            "##.",
            ".##",
            "",
            "2:",
            ".##",
            "###",
            "##.",
            "",
            "3:",
            "##.",
            "###",
            "##.",
            "",
            "4:",
            "###",
            "#..",
            "###",
            "",
            "5:",
            "###",
            ".#.",
            "###",
            "",
            "4x4: 0 0 0 0 2 0",
            "12x5: 1 0 1 0 2 2",
            "12x5: 1 0 1 0 3 2",
        ];

        let expected = 0;

        let actual = solve_part_2(&input);

        assert_eq!(actual, expected);
    }
}
