use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    vec,
};

pub fn solve(input: &[&str]) -> String {
    let part1 = count_removable_blocks(input);
    let part2 = "";

    format!(" Part1: {} \n Part2: {}", part1, part2)
}

fn count_removable_blocks(input: &[&str]) -> usize {
    let blocks = input
        .into_iter()
        .map(|l| Line::parse_str(l))
        .collect::<Vec<_>>();
    let blocks = drop_blocks(&blocks);

    // dbg!(&blocks);

    let mut endpoints: HashMap<usize, Vec<&Line>> = HashMap::new();

    for block in blocks.iter() {
        endpoints
            .entry(block.end.z)
            .and_modify(|e| e.push(&block))
            .or_insert(vec![&block]);
    }

    let mut not_removable: HashSet<&Line> = HashSet::new();

    for block in blocks.iter() {
        if block.start.z > 0 {
            if let Some(below) = endpoints.get(&(block.start.z - 1)) {
                let mut supporting = HashSet::new();

                for other in below {
                    if other.is_supporting(block) {
                        supporting.insert(other);
                    }
                }

                // dbg!(&supporting);
                if supporting.len() == 1 {
                    not_removable.extend(supporting.into_iter());
                }
            }
        }
    }

    // for i in (0..blocks.len()).rev() {
    //     let block = &blocks[i];
    //     // dbg!(&block);
    //     let mut supporting = vec![];
    //
    //     for j in (0..i).rev() {
    //         let other = &blocks[j];
    //         if other.is_supporting(block) {
    //             supporting.push(other);
    //         }
    //     }
    //
    //     // dbg!(&supporting);
    //     if supporting.len() > 1 {
    //         for block in supporting.into_iter() {
    //             removable.insert(block);
    //         }
    //     }
    // }

    // dbg!(&not_removable);
    blocks.len() - not_removable.len()
}

fn drop_blocks(blocks: &Vec<Line>) -> Vec<Line> {
    let mut blocks = blocks.clone();
    blocks.sort_by_key(|line| (line.start.z, std::cmp::Reverse(line.start.z)));
    let mut dropped_blocks = vec![];

    let mut i = 0;
    while i < blocks.len() {
        let mut block = blocks[i].clone();
        // dbg!(&block);
        let mut distance = block.start.z;
        for j in (0..i).rev() {
            let other = &dropped_blocks[j];
            // dbg!(block.will_rest_on(&other));
            if block.will_rest_on(&other) {
                distance = block.start.z - other.end.z - 1;
                break;
            }
        }

        // dbg!(distance);
        block.drop(distance);
        // dbg!(&block);
        dropped_blocks.push(block);
        i += 1;
    }

    // dbg!(&dropped_blocks);
    dropped_blocks
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point3D {
    x: usize,
    y: usize,
    z: usize,
}

impl Point3D {
    fn parse_str(input: &str) -> Self {
        let mut numbers = input.split(',').map(|n| n.parse::<usize>().unwrap());

        Self {
            x: numbers.next().unwrap(),
            y: numbers.next().unwrap(),
            z: numbers.next().unwrap(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Line {
    start: Point3D,
    end: Point3D,
}

impl Line {
    fn parse_str(input: &str) -> Self {
        let mut points = input.split('~').map(Point3D::parse_str);
        let mut start = points.next().unwrap();
        let mut end = points.next().unwrap();
        if end.z < start.z {
            std::mem::swap(&mut start, &mut end);
        }

        Self { start, end }
    }

    fn is_supporting(&self, other: &Self) -> bool {
        // dbg!(self.end.z + 1 == other.start.z);
        // dbg!(self.intersect_x(other));
        // dbg!(self.intersect_y(other));
        self.end.z + 1 == other.start.z && self.intersect_x(other) && self.intersect_y(other)
    }

    fn will_rest_on(&self, other: &Self) -> bool {
        // dbg!(self.start.z > other.end.z);
        // dbg!(self.intersect_x(other));
        // dbg!(self.intersect_y(other));
        self.start.z > other.end.z && self.intersect_x(other) && self.intersect_y(other)
    }

    fn drop(&mut self, distance: usize) {
        self.start.z -= distance;
        self.end.z -= distance;
    }

    fn intersect_x(&self, other: &Self) -> bool {
        for x in min(self.start.x, self.end.x)..=max(self.start.x, self.end.x) {
            if x >= min(other.start.x, other.end.x) && x <= max(other.start.x, other.end.x) {
                // dbg!("intersect_x");
                return true;
            }
        }

        false
    }

    fn intersect_y(&self, other: &Self) -> bool {
        // dbg!((self, other));
        for y in min(self.start.y, self.end.y)..=max(self.start.y, self.end.y) {
            // dbg!(y);
            if y >= min(other.start.y, other.end.y) && y <= max(other.start.y, other.end.y) {
                // dbg!("intersect_y");
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    // 446
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_count_removable_blocks() {
        let input = [
            "1,0,1~1,2,1",
            "0,0,2~2,0,2",
            "0,2,3~2,2,3",
            "0,0,4~0,2,4",
            "2,0,5~2,2,5",
            "0,1,6~2,1,6",
            "1,1,8~1,1,9",
        ];

        let expected = 5;
        let actual = count_removable_blocks(&input);
        assert_eq!(actual, expected);

        let input = [
            "0,0,1~0,0,2", // - A
            "1,0,1~2,0,1", // - B
            "1,0,2~1,0,2", // - C
            "0,0,3~1,0,3", // - D
        ];

        let expected = 3;
        let actual = count_removable_blocks(&input);
        assert_eq!(actual, expected);

        let input = [
            "0,0,1~0,1,1", // <-- A
            "1,1,1~1,1,1", // <-- B
            "0,0,2~0,0,2", // <-- C
            "0,1,2~1,1,2", // <-- D
        ];

        let expected = 3;
        let actual = count_removable_blocks(&input);
        assert_eq!(actual, expected);

        let input = [
            "0,0,1~1,0,1", // <- A
            "0,1,1~0,1,2", // <- B
            "0,0,5~0,0,5", // <- C
            "0,0,4~0,1,4", // <- D
        ];

        let expected = 2;
        let actual = count_removable_blocks(&input);
        assert_eq!(actual, expected);

        //   X        Y
        //
        //   .B...... .BBBBB.
        // z .AAAAAA. .A.....
        #[rustfmt::skip]
        let input = [
            "5,1,1~1,1,1", // <- A
            "1,5,2~1,1,2", // <- B
        ];

        let expected = 1;
        let actual = count_removable_blocks(&input);
        assert_eq!(actual, expected);

        #[rustfmt::skip]
        let input = [
            "0,0,2~0,0,4",
            "1,0,3~2,0,3",
            "1,0,4~1,0,5",
            "0,0,6~1,0,6",
        ];

        let expected = 3;
        let actual = count_removable_blocks(&input);
        assert_eq!(actual, expected);

        let input = [
            "0,0,1~0,0,1", // <- A
            "1,1,1~1,1,1", // <- B
            "0,0,2~0,1,2", // <- C
            "0,1,3~1,1,3", // <- D
        ];

        let expected = 2;
        let actual = count_removable_blocks(&input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_drop_blocks() {
        let input = [
            "1,0,1~1,2,1", // <- A
            "0,0,2~2,0,2", // <- B
            "0,2,3~2,2,3", // <- C
            "0,0,4~0,2,4", // <- D
            "2,0,5~2,2,5", // <- E
            "0,1,6~2,1,6", // <- F
            "1,1,8~1,1,9", // <- G
        ]
        .iter()
        .map(|l| Line::parse_str(l))
        .collect::<Vec<_>>();

        let expected = [
            "1,0,0~1,2,0", // <- A
            "0,0,1~2,0,1", // <- B
            "0,2,1~2,2,1", // <- C
            "0,0,2~0,2,2", // <- D
            "2,0,2~2,2,2", // <- E
            "0,1,3~2,1,3", // <- F
            "1,1,4~1,1,5", // <- G
        ]
        .iter()
        .map(|l| Line::parse_str(l))
        .collect::<Vec<_>>();

        let actual = drop_blocks(&input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_is_supporting() {
        let blocks = [
            "1,0,0~1,2,0", // <- A
            "0,0,1~2,0,1", // <- B
            "0,2,1~2,2,1", // <- C
            "0,0,2~0,2,2", // <- D
            "2,0,2~2,2,2", // <- E
            "0,1,3~2,1,3", // <- F
            "1,1,4~1,1,5", // <- G
        ]
        .iter()
        .map(|l| Line::parse_str(l))
        .collect::<Vec<_>>();

        let expected = true;
        let actual = blocks[1].is_supporting(&blocks[3]);
        assert_eq!(actual, expected);

        let expected = true;
        let actual = blocks[2].is_supporting(&blocks[3]);
        assert_eq!(actual, expected);

        let expected = true;
        let actual = blocks[0].is_supporting(&blocks[1]);
        assert_eq!(actual, expected);

        let expected = true;
        let actual = blocks[0].is_supporting(&blocks[2]);
        assert_eq!(actual, expected);

        let expected = false;
        let actual = blocks[0].is_supporting(&blocks[3]);
        assert_eq!(actual, expected);

        let expected = false;
        let actual = blocks[0].is_supporting(&blocks[4]);
        assert_eq!(actual, expected);

        let expected = true;
        let actual = blocks[5].is_supporting(&blocks[6]);
        assert_eq!(actual, expected);
    }
}
