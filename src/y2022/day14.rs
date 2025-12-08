use crate::util::grid_v1::{Grid, GridDirections, Point};

pub fn solve(input: &[&str]) -> String {
    let part1 = drop_sand(input);
    let part2 = drop_all_sand(input);

    format!(" Part1: {} \n Part2: {}", part1, part2)
}

fn drop_sand(input: &[&str]) -> usize {
    let mut map = map_cave(input);
    let source = map.find('+').unwrap();

    let mut sand = None;

    loop {
        if sand.is_none() {
            sand = Some(Point::new(source.x, source.y));
        }

        let mut new_position = sand.unwrap();
        for (dx, dy) in [
            GridDirections::DOWN,
            GridDirections::DOWN_LEFT,
            GridDirections::DOWN_RIGHT,
        ] {
            let dest = Point::new(new_position.x + dx, new_position.y + dy);
            let dest_contents = map
                .at_point(&dest)
                .unwrap_or_else(|| panic!("Could not get map contents at {:?}", dest));
            if *dest_contents != '#' && *dest_contents != 'o' {
                new_position.x += dx;
                new_position.y += dy;
                break;
            }
        }

        if new_position == sand.unwrap() {
            sand = None;
            map.set_at_point(&new_position, 'o').unwrap();
        } else {
            sand = Some(new_position);
        }

        if new_position.y + 1 >= map.height as i32 {
            break;
        }
    }

    // dbg!(&map);

    map.spaces.iter().filter(|s| **s == 'o').count()
}

fn drop_all_sand(input: &[&str]) -> usize {
    let mut map = map_cave(input);
    let source = map.find('+').unwrap();

    let mut sand = None;

    loop {
        if sand.is_none() {
            sand = Some(Point::new(source.x, source.y));
        }

        let mut new_position = sand.unwrap();
        for (dx, dy) in [
            GridDirections::DOWN,
            GridDirections::DOWN_LEFT,
            GridDirections::DOWN_RIGHT,
        ] {
            let dest = Point::new(new_position.x + dx, new_position.y + dy);
            let dest_contents = map
                .at_point(&dest)
                .unwrap_or_else(|| panic!("Could not get map contents at {:?}", dest));
            if *dest_contents != '#' && *dest_contents != 'o' {
                new_position.x += dx;
                new_position.y += dy;
                break;
            }
        }

        if new_position == sand.unwrap() || new_position.y == (map.height - 1) as i32 {
            sand = None;
            map.set_at_point(&new_position, 'o').unwrap();
        } else {
            sand = Some(new_position);
        }

        if *map.at_point(&source).unwrap() == 'o' {
            break;
        }
    }

    // dbg!(&map);

    map.spaces.iter().filter(|s| **s == 'o').count()
}

fn map_cave(input: &[&str]) -> Grid<char> {
    let (mut x_min, mut x_max, mut y_min, mut y_max) = (i32::MAX, 0, i32::MAX, 0);
    let mut lines = Vec::new();

    for input_line in input {
        let line = parse_line(input_line);

        for point in line.iter() {
            if point.x < x_min {
                x_min = point.x;
            }
            if point.x > x_max {
                x_max = point.x;
            }
            if point.y < y_min {
                y_min = point.y;
            }
            if point.y > y_max {
                y_max = point.y;
            }
        }

        lines.push(line);
    }

    // let x_offset = x_min - 1;
    // let width = (x_max + 2 - x_offset) as usize;
    let x_offset = 0;

    let source_point = Point::new(500 - x_offset, 0);

    let width = 1000;
    let height = (y_max + 2) as usize;
    let mut map = Grid::build(width, height, '.');

    for line in lines {
        let mut line_iter = line.iter();
        let mut start = line_iter.next().unwrap();

        for end in line_iter {
            let offset_start = Point::new(start.x - x_offset, start.y);
            let offset_end = Point::new(end.x - x_offset, end.y);

            if start.x == end.x {
                map.draw_vertical_line(&offset_start, &offset_end, '#')
                    .unwrap();
            } else if start.y == end.y {
                map.draw_horizontal_line(&offset_start, &offset_end, '#')
                    .unwrap();
            } else {
                panic!("Line `{:#?}` is not vertical or horizontal!", line);
            }

            start = end;
        }
    }

    map.set_at_point(&source_point, '+').unwrap();

    // dbg!(&map);

    map
}

fn parse_line(input: &str) -> Vec<Point> {
    let mut line = Vec::new();

    for pair in input.split("->") {
        let (x, y) = pair.split_once(',').unwrap();
        let x = x
            .trim()
            .parse::<i32>()
            .unwrap_or_else(|err| panic!("Could not parse `{}` as i32: {}", x, err));
        let y = y
            .trim()
            .parse::<i32>()
            .unwrap_or_else(|err| panic!("Could not parse `{}` as i32: {}", y, err));

        line.push(Point::new(x, y));
    }

    line
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_drop_sand() {
        #[rustfmt::skip]
        let input = [
            "498,4 -> 498,6 -> 496,6",
            "503,4 -> 502,4 -> 502,9 -> 494,9",
        ];

        let actual = drop_sand(&input);
        let expected = 24;

        assert_eq!(
            actual, expected,
            "\n Got {:#?} when expecting {:#?} form calling drop_sand on {:#?}",
            actual, expected, input
        );
    }

    #[test]
    fn test_drop_all_sand() {
        #[rustfmt::skip]
        let input = [
            "498,4 -> 498,6 -> 496,6",
            "503,4 -> 502,4 -> 502,9 -> 494,9",
        ];

        let actual = drop_all_sand(&input);
        let expected = 93;

        assert_eq!(
            actual, expected,
            "\n Got {:#?} when expecting {:#?} form calling drop_sand on {:#?}",
            actual, expected, input
        );
    }

    #[test]
    fn test_parse_line() {
        let cases = [
            (
                "498,4 -> 498,6 -> 496,6",
                vec![Point::new(498, 4), Point::new(498, 6), Point::new(496, 6)],
            ),
            (
                "503,4 -> 502,4 -> 502,9 -> 494,9",
                vec![
                    Point::new(503, 4),
                    Point::new(502, 4),
                    Point::new(502, 9),
                    Point::new(494, 9),
                ],
            ),
        ];

        for (input, expected) in cases {
            let actual = parse_line(input);

            assert_eq!(
                actual, expected,
                "\n Got {:#?} when expecting {:#?} form calling parse_line on {:#?}",
                actual, expected, input
            );
        }
    }

    // #[test]
    // fn test_map_cave() {
    //     let input = [
    //         "498,4 -> 498,6 -> 496,6",
    //         "503,4 -> 502,4 -> 502,9 -> 494,9",
    //     ];
    //
    //     let expected: Vec<char> = [
    //         ".......+....",
    //         "............",
    //         "............",
    //         "............",
    //         ".....#...##.",
    //         ".....#...#..",
    //         "...###...#..",
    //         ".........#..",
    //         ".........#..",
    //         ".#########..",
    //         "............",
    //     ]
    //     .iter()
    //     .flat_map(|l| l.chars())
    //     .collect();
    //
    //     let actual = map_cave(&input).spaces;
    //
    //     assert_eq!(
    //         actual, expected,
    //         "\n Got {:#?} when expecting {:#?} form calling parse_line on {:#?}",
    //         actual, expected, input
    //     );
    // }
}
