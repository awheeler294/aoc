use std::{
    cmp::{max, min},
    collections::HashMap,
    convert::TryFrom,
    iter::FromIterator,
};

use anyhow::anyhow;

pub fn solve(input: &[&str]) -> String {
    let part1 = num_steps(input);
    let part2 = simultaneous_steps(input);

    format!(" Part1: {} \n Part2: {}", part1, part2)
}

fn num_steps(input: &[&str]) -> usize {
    let directions = Direction::parse_directions(input[0]);

    let graph = Node::parse_node_map(&input[2..]);

    let mut dir_idx = 0;
    let mut step_count = 0;
    let mut node = "AAA";

    loop {
        let dir = &directions[dir_idx];
        node = graph.get(node).unwrap().get(dir);

        step_count += 1;

        if node == "ZZZ" {
            return step_count;
        }

        dir_idx = (dir_idx + 1) % directions.len();
    }
}

fn simultaneous_steps(input: &[&str]) -> usize {
    let directions = Direction::parse_directions(input[0]);

    let graph = Node::parse_node_map(&input[2..]);

    let mut ghosts = graph
        .iter()
        .filter_map(|(_k, node)| {
            if node.value.ends_with("A") {
                Some(Ghost::new(node))
            } else {
                None
            }
        })
        .collect::<Vec<Ghost>>();

    let mut dir_idx = 0;
    while ghosts.iter().all(|g| g.is_at_goal()) == false {
        ghosts
            .iter_mut()
            .for_each(|g| g.move_ghost(&directions[dir_idx], &graph));
        dir_idx = (dir_idx + 1) % directions.len();
    }

    ghosts
        .iter()
        .fold(1, |lcm, ghost| least_common_multiple(lcm, ghost.steps))
}

fn least_common_multiple(a: usize, b: usize) -> usize {
    a * b / greatest_common_denominator(a, b)
}

fn greatest_common_denominator(a: usize, b: usize) -> usize {
    let (mut min, mut max) = (min(a, b), max(a, b));

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

#[derive(Debug)]
struct Ghost<'a> {
    current_node: &'a Node<'a>,
    steps: usize,
}

impl<'a> Ghost<'a> {
    fn new(starting_node: &'a Node) -> Self {
        Self {
            current_node: starting_node,
            steps: 0,
        }
    }

    fn move_ghost(&mut self, direction: &Direction, graph: &'a HashMap<&str, Node>) {
        if self.is_at_goal() == false {
            self.current_node = graph.get(self.current_node.get(direction)).unwrap();
            self.steps += 1;
        }
    }

    fn is_at_goal(&self) -> bool {
        self.current_node.value.ends_with("Z")
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn parse_directions(directions: &str) -> Vec<Self> {
        directions
            .chars()
            .map(Self::try_from)
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
    }
}

impl TryFrom<char> for Direction {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            _ => Err(anyhow!("Could not parse `{value}` as direction")),
        }
    }
}

#[derive(Debug)]
struct Node<'a> {
    value: &'a str,
    left: &'a str,
    right: &'a str,
}

impl<'a> Node<'a> {
    fn get(&self, direction: &Direction) -> &str {
        match direction {
            Direction::Left => self.left,
            Direction::Right => self.right,
        }
    }

    fn parse_node_map(input: &'a [&'a str]) -> HashMap<&'a str, Node<'a>> {
        HashMap::from_iter(input.into_iter().map(|line| {
            let node = Node::try_from(*line).expect(&format!("Error parsing `{line}`"));
            (node.value, node)
        }))
    }
}

impl<'a> TryFrom<&'a str> for Node<'a> {
    type Error = anyhow::Error;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        let (value, remainder) = input
            .split_once('=')
            .ok_or_else(|| anyhow!("Could not split `{input}` on '='"))?;

        let (left, right) = remainder
            .trim_matches(&[' ', '(', ')'] as &[_])
            .split_once(',')
            .ok_or_else(|| anyhow!("Could not split `{remainder}` on ','"))?;

        Ok(Self {
            value: value.trim(),
            left: left.trim(),
            right: right.trim(),
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_num_steps() {
        #[rustfmt::skip]
        let input = [
            "RL",
            "",
            "AAA = (BBB, CCC)",
            "BBB = (DDD, EEE)",
            "CCC = (ZZZ, GGG)",
            "DDD = (DDD, DDD)",
            "EEE = (EEE, EEE)",
            "GGG = (GGG, GGG)",
            "ZZZ = (ZZZ, ZZZ)",
        ];

        let expected = 2;
        let actual = num_steps(&input);
        assert_eq!(actual, expected);

        #[rustfmt::skip]
        let input = [
            "LLR",
            "",
            "AAA = (BBB, BBB)",
            "BBB = (AAA, ZZZ)",
            "ZZZ = (ZZZ, ZZZ)",
        ];

        let expected = 6;
        let actual = num_steps(&input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_simultaneous_steps() {
        #[rustfmt::skip]
        let input = [
            "LR",
            "",
            "11A = (11B, XXX)",
            "11B = (XXX, 11Z)",
            "11Z = (11B, XXX)",
            "22A = (22B, XXX)",
            "22B = (22C, 22C)",
            "22C = (22Z, 22Z)",
            "22Z = (22B, 22B)",
            "XXX = (XXX, XXX)",
        ];

        let expected = 6;
        let actual = simultaneous_steps(&input);
        assert_eq!(actual, expected);
    }
}
