use regex::Regex;

use crate::grid::GridDirection;
use std::i64;

pub fn solve(input: &[&str]) -> String {
    let part1 = dug_area(&input);
    let part2 = dug_area2(&input);

    format!(" Part1: {} \n Part2: {}", part1, part2)
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct IPoint {
    pub x: i64,
    pub y: i64,
}

impl IPoint {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

type DigPlan = Vec<(GridDirection, i64)>;

fn dug_area(dig_plan: &[&str]) -> usize {
    calculate_area(parse_dig_plan(dig_plan))
}

fn dug_area2(dig_plan: &[&str]) -> usize {
    calculate_area(parse_dig_plan2(dig_plan))
}

fn parse_dig_plan(input: &[&str]) -> DigPlan {
    let parse_re = Regex::new(
        r"(?P<direction>[UDLR])\s(?P<distance>[0-9]+)\s\(\#(?P<color_code>[a-f0-9]{6})\)",
    )
    .unwrap();

    input
        .iter()
        .map(|line| {
            let captures = parse_re.captures(line).unwrap();

            let direction = match &captures["direction"] {
                "U" => GridDirection::Up,
                "D" => GridDirection::Down,
                "L" => GridDirection::Left,
                "R" => GridDirection::Right,
                _ => unreachable!(),
            };
            let distance = &captures["distance"];
            let _color = &captures["color_code"];

            let distance = distance.parse::<i64>().unwrap();

            (direction, distance)
        })
        .collect::<Vec<_>>()
}

fn parse_dig_plan2(input: &[&str]) -> DigPlan {
    let parse_re = Regex::new(r"\#(?P<distance>[a-f0-9]{5})(?P<direction>[a-f0-9])").unwrap();

    input
        .iter()
        .map(|line| {
            let captures = parse_re.captures(line).unwrap();

            let direction = match &captures["direction"] {
                "0" => GridDirection::Right,
                "1" => GridDirection::Down,
                "2" => GridDirection::Left,
                "3" => GridDirection::Up,
                _ => unreachable!(),
            };
            let distance = &captures["distance"];

            let distance = i64::from_str_radix(distance, 16).unwrap();

            (direction, distance)
        })
        .collect::<Vec<_>>()
}

fn calculate_area(dig_plan: DigPlan) -> usize {
    let mut startpoint = IPoint::new(0, 0);
    let points = {
        let mut lines = Vec::new();

        for (direction, distance) in dig_plan {
            // dbg!(position);
            // dbg!((direction, distance));
            let endpoint = match direction {
                GridDirection::Up => IPoint::new(startpoint.x, startpoint.y - distance),

                GridDirection::Down => IPoint::new(startpoint.x, startpoint.y + distance),

                GridDirection::Left => IPoint::new(startpoint.x - distance, startpoint.y),

                GridDirection::Right => IPoint::new(startpoint.x + distance, startpoint.y),
                _ => unreachable!(),
            };

            lines.push(endpoint);

            startpoint = endpoint;
        }
        lines
    };

    // dbg!(&lines);

    let mut xcc = 0;
    let mut ycc = 0;
    for i in 1..points.len() - 1 {
        let curr = points[i];
        let next = points[i - 1];

        xcc += curr.x * next.y;
        ycc += curr.y * next.x;
    }

    // dbg!(xcc);
    // dbg!(ycc);

    let inside_area = (xcc - ycc).abs() / 2;

    // dbg!(inside_area);

    let perimeter = {
        let mut p = 0;
        for i in 1..points.len() {
            p += (points[i].x - points[i - 1].x).abs() + (points[i].y - points[i - 1].y).abs();
        }
        p += (points.last().unwrap().x - points[0].x).abs()
            + (points.last().unwrap().y - points[0].y).abs();
        p
    };

    // dbg!(perimeter);

    (inside_area + (perimeter / 2) + 1) as usize
}

#[cfg(test)]
mod tests {

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_dug_area() {
        let dig_plan = [
            "R 6 (#70c710)",
            "D 5 (#0dc571)",
            "L 2 (#5713f0)",
            "D 2 (#d2c081)",
            "R 2 (#59c680)",
            "D 2 (#411b91)",
            "L 5 (#8ceee2)",
            "U 2 (#caa173)",
            "L 1 (#1b58a2)",
            "U 2 (#caa171)",
            "R 2 (#7807d2)",
            "U 3 (#a77fa3)",
            "L 2 (#015232)",
            "U 2 (#7a21e3)",
        ];

        let expected = 62;
        let actual = dug_area(&dig_plan);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_dug_area2() {
        let dig_plan = [
            "R 6 (#70c710)",
            "D 5 (#0dc571)",
            "L 2 (#5713f0)",
            "D 2 (#d2c081)",
            "R 2 (#59c680)",
            "D 2 (#411b91)",
            "L 5 (#8ceee2)",
            "U 2 (#caa173)",
            "L 1 (#1b58a2)",
            "U 2 (#caa171)",
            "R 2 (#7807d2)",
            "U 3 (#a77fa3)",
            "L 2 (#015232)",
            "U 2 (#7a21e3)",
        ];

        let expected = 952408144115;
        let actual = dug_area2(&dig_plan);

        assert_eq!(actual, expected);
    }
}
