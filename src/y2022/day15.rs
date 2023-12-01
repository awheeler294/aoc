use crate::grid::Point;

pub fn solve(input: &[&str]) -> String {
    let part1 = count_exclusions(input, 2_000_000);
    let part2 = tuning_frequency(input, 4_000_000);

    format!(" Part1: {} \n Part2: {}", part1, part2)
}

#[derive(Debug, PartialEq, Eq)]
struct Sensor {
    position: Point,
    beacon: Point,
    radius: i32,
}

impl Sensor {
    fn new(position: Point, beacon_position: Point) -> Self {
        let radius = position.manhattan_distance(beacon_position);

        Self {
            position,
            beacon: beacon_position,
            radius,
        }
    }

    fn parse_sensor(input: &str) -> Self {
        let mut tokens = input.split_whitespace();

        let sensor_x = tokens
            .nth(2)
            .unwrap()
            .replace(',', "")
            .chars()
            .skip(2)
            .collect::<String>()
            .parse()
            .unwrap();

        let sensor_y = tokens
            .next()
            .unwrap()
            .replace(':', "")
            .chars()
            .skip(2)
            .collect::<String>()
            .parse()
            .unwrap();

        let beacon_x = tokens
            .nth(4)
            .unwrap()
            .replace(',', "")
            .chars()
            .skip(2)
            .collect::<String>()
            .parse()
            .unwrap();

        let beacon_y = tokens
            .next()
            .unwrap()
            .chars()
            .skip(2)
            .collect::<String>()
            .parse()
            .unwrap();

        Self::new(
            Point::new(sensor_x, sensor_y),
            Point::new(beacon_x, beacon_y),
        )
    }

    fn get_x_endpoint(&self, y: i32) -> Option<i32> {
        if (self.position.y - y).abs() > self.radius {
            return None;
        }

        Some(self.position.x + self.radius - (self.position.y - y).abs())
    }
}

fn count_exclusions(input: &[&str], y: i32) -> i32 {
    let sensors = input
        .iter()
        .map(|l| Sensor::parse_sensor(l))
        .collect::<Vec<Sensor>>();

    let mut min_x = i32::MAX;
    let mut max_x = 0;
    for sensor in sensors.iter() {
        let sensor_min_x = sensor.position.x - sensor.radius;
        if sensor_min_x < min_x {
            min_x = sensor_min_x;
        }

        let sensor_max_x = sensor.position.x + sensor.radius;
        if sensor_max_x > max_x {
            max_x = sensor_max_x;
        }
    }

    let mut exclusion_count = 0;
    dbg!(max_x);
    // let mut exclusions = vec![];

    for x in min_x..=max_x {
        let position = Point::new(x, y);

        // let mut excluded = false;

        for sensor in sensors.iter() {
            if sensor.position.manhattan_distance(position) <= sensor.radius
                && position != sensor.beacon
            {
                exclusion_count += 1;
                // excluded = true;
                break;
            }
        }

        // if excluded {
        // exclusions.push('#');
        // } else {
        // exclusions.push('.');
        // }
    }

    // dbg!(exclusions);

    exclusion_count
}

fn tuning_frequency(input: &[&str], search_max: i32) -> u64 {
    let sensors = input
        .iter()
        .map(|l| Sensor::parse_sensor(l))
        .collect::<Vec<Sensor>>();

    let mut beacon_position = None;

    for y in 0..=search_max {
        if y % 100_000 == 0 {
            dbg!(y);
        }
        let mut y_culled_sensors = sensors
            .iter()
            .filter(|s| y <= s.position.y + s.radius && y >= s.position.y - s.radius)
            .collect::<Vec<&Sensor>>();

        y_culled_sensors.sort_by(|a, b| a.position.x.cmp(&b.position.x));
        let mut x = 0;
        while x <= search_max {
            let position = Point::new(x, y);
            // print!("x: {}, ", x);
            let mut coverage = false;

            for sensor in y_culled_sensors.iter() {
                if sensor.position.manhattan_distance(position) <= sensor.radius {
                    let sensor_endpoint = sensor.get_x_endpoint(y).unwrap();
                    if sensor_endpoint > x {
                        x = sensor_endpoint;
                    }

                    coverage = true;
                }
            }

            if !coverage {
                beacon_position = Some(Point::new(x, y));
                break;
            }

            x += 1;
        }

        if beacon_position.is_some() {
            break;
        }
        // println!();
    }

    let beacon_position = beacon_position.unwrap();

    dbg!(&beacon_position);

    (beacon_position.x as u64 * 4_000_000) + beacon_position.y as u64
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_count_exclusions() {
        let input = [
            "Sensor at x=2, y=18: closest beacon is at x=-2, y=15",
            "Sensor at x=9, y=16: closest beacon is at x=10, y=16",
            "Sensor at x=13, y=2: closest beacon is at x=15, y=3",
            "Sensor at x=12, y=14: closest beacon is at x=10, y=16",
            "Sensor at x=10, y=20: closest beacon is at x=10, y=16",
            "Sensor at x=14, y=17: closest beacon is at x=10, y=16",
            "Sensor at x=8, y=7: closest beacon is at x=2, y=10",
            "Sensor at x=2, y=0: closest beacon is at x=2, y=10",
            "Sensor at x=0, y=11: closest beacon is at x=2, y=10",
            "Sensor at x=20, y=14: closest beacon is at x=25, y=17",
            "Sensor at x=17, y=20: closest beacon is at x=21, y=22",
            "Sensor at x=16, y=7: closest beacon is at x=15, y=3",
            "Sensor at x=14, y=3: closest beacon is at x=15, y=3",
            "Sensor at x=20, y=1: closest beacon is at x=15, y=3",
        ];

        let expected = 26;
        let actual = count_exclusions(&input, 10);

        assert_eq!(
            actual, expected,
            "\n Got {:#?} when expecting {:#?} from calling exclusion_count on {:#?}",
            actual, expected, input
        );
    }

    #[test]
    fn test_tuning_frequency() {
        let input = [
            "Sensor at x=2, y=18: closest beacon is at x=-2, y=15",
            "Sensor at x=9, y=16: closest beacon is at x=10, y=16",
            "Sensor at x=13, y=2: closest beacon is at x=15, y=3",
            "Sensor at x=12, y=14: closest beacon is at x=10, y=16",
            "Sensor at x=10, y=20: closest beacon is at x=10, y=16",
            "Sensor at x=14, y=17: closest beacon is at x=10, y=16",
            "Sensor at x=8, y=7: closest beacon is at x=2, y=10",
            "Sensor at x=2, y=0: closest beacon is at x=2, y=10",
            "Sensor at x=0, y=11: closest beacon is at x=2, y=10",
            "Sensor at x=20, y=14: closest beacon is at x=25, y=17",
            "Sensor at x=17, y=20: closest beacon is at x=21, y=22",
            "Sensor at x=16, y=7: closest beacon is at x=15, y=3",
            "Sensor at x=14, y=3: closest beacon is at x=15, y=3",
            "Sensor at x=20, y=1: closest beacon is at x=15, y=3",
        ];

        let expected = 56_000_011;
        let actual = tuning_frequency(&input, 20);

        assert_eq!(
            actual, expected,
            "\n Got {:#?} when expecting {:#?} from calling exclusion_count on {:#?}",
            actual, expected, input
        );
    }

    #[test]
    fn test_get_x_endpoint() {
        let cases = [
            (
                Sensor::new(Point::new(2, 18), Point::new(-2, 15)),
                18,
                2 + 7,
            ),
            (
                Sensor::new(Point::new(2, 18), Point::new(-2, 15)),
                17,
                2 + 6,
            ),
            (
                Sensor::new(Point::new(2, 18), Point::new(-2, 15)),
                19,
                2 + 6,
            ),
            (
                Sensor::new(Point::new(2, 18), Point::new(-2, 15)),
                16,
                2 + 5,
            ),
            (
                Sensor::new(Point::new(2, 18), Point::new(-2, 15)),
                20,
                2 + 5,
            ),
            // (Sensor::new(Point::new(9, 16), Point::new(0, 16)), 9),
            // (Sensor::new(Point::new(13, 2), Point::new(15, 3)), 3),
            // (Sensor::new(Point::new(12, 14), Point::new(10, 16)), 4),
            // (Sensor::new(Point::new(10, 20), Point::new(10, 16)), 4),
            // (Sensor::new(Point::new(14, 17), Point::new(10, 16)), 5),
            // (Sensor::new(Point::new(8, 7), Point::new(2, 10)), 9),
            // (Sensor::new(Point::new(2, 0), Point::new(2, 10)), 10),
            // (Sensor::new(Point::new(0, 11), Point::new(2, 10)), 3),
            // (Sensor::new(Point::new(20, 14), Point::new(25, 17)), 8),
            // (Sensor::new(Point::new(17, 20), Point::new(21, 22)), 6),
            // (Sensor::new(Point::new(16, 7), Point::new(15, 3)), 5),
            // (Sensor::new(Point::new(14, 3), Point::new(15, 3)), 1),
            // (Sensor::new(Point::new(20, 1), Point::new(15, 3)), 7),
        ];

        for (sensor, y, expected) in cases {
            let actual = sensor.get_x_endpoint(y).unwrap();
            assert_eq!(actual, expected, "\n Got `{actual}` when expecting `{expected}` from calling get_x_endpoint on {:#?} with y = `{:#?}`", sensor, y);
        }
    }

    #[test]
    fn test_parse_sensors() {
        let cases = [
            (
                "Sensor at x=2, y=18: closest beacon is at x=-2, y=15",
                Sensor {
                    position: Point { x: 2, y: 18 },
                    beacon: Point { x: -2, y: 15 },
                    radius: 7,
                },
            ),
            (
                "Sensor at x=9, y=16: closest beacon is at x=10, y=16",
                Sensor {
                    position: Point { x: 9, y: 16 },
                    beacon: Point { x: 10, y: 16 },
                    radius: 1,
                },
            ),
            (
                "Sensor at x=13, y=2: closest beacon is at x=15, y=3",
                Sensor {
                    position: Point { x: 13, y: 2 },
                    beacon: Point { x: 15, y: 3 },
                    radius: 3,
                },
            ),
            (
                "Sensor at x=12, y=14: closest beacon is at x=10, y=16",
                Sensor {
                    position: Point { x: 12, y: 14 },
                    beacon: Point { x: 10, y: 16 },
                    radius: 4,
                },
            ),
            (
                "Sensor at x=10, y=20: closest beacon is at x=10, y=16",
                Sensor {
                    position: Point { x: 10, y: 20 },
                    beacon: Point { x: 10, y: 16 },
                    radius: 4,
                },
            ),
            (
                "Sensor at x=14, y=17: closest beacon is at x=10, y=16",
                Sensor {
                    position: Point { x: 14, y: 17 },
                    beacon: Point { x: 10, y: 16 },
                    radius: 5,
                },
            ),
            (
                "Sensor at x=8, y=7: closest beacon is at x=2, y=10",
                Sensor {
                    position: Point { x: 8, y: 7 },
                    beacon: Point { x: 2, y: 10 },
                    radius: 9,
                },
            ),
            (
                "Sensor at x=2, y=0: closest beacon is at x=2, y=10",
                Sensor {
                    position: Point { x: 2, y: 0 },
                    beacon: Point { x: 2, y: 10 },
                    radius: 10,
                },
            ),
            (
                "Sensor at x=0, y=11: closest beacon is at x=2, y=10",
                Sensor {
                    position: Point { x: 0, y: 11 },
                    beacon: Point { x: 2, y: 10 },
                    radius: 3,
                },
            ),
            (
                "Sensor at x=20, y=14: closest beacon is at x=25, y=17",
                Sensor {
                    position: Point { x: 20, y: 14 },
                    beacon: Point { x: 25, y: 17 },
                    radius: 8,
                },
            ),
            (
                "Sensor at x=17, y=20: closest beacon is at x=21, y=22",
                Sensor {
                    position: Point { x: 17, y: 20 },
                    beacon: Point { x: 21, y: 22 },
                    radius: 6,
                },
            ),
            (
                "Sensor at x=16, y=7: closest beacon is at x=15, y=3",
                Sensor {
                    position: Point { x: 16, y: 7 },
                    beacon: Point { x: 15, y: 3 },
                    radius: 5,
                },
            ),
            (
                "Sensor at x=14, y=3: closest beacon is at x=15, y=3",
                Sensor {
                    position: Point { x: 14, y: 3 },
                    beacon: Point { x: 15, y: 3 },
                    radius: 1,
                },
            ),
            (
                "Sensor at x=20, y=1: closest beacon is at x=15, y=3",
                Sensor {
                    position: Point { x: 20, y: 1 },
                    beacon: Point { x: 15, y: 3 },
                    radius: 7,
                },
            ),
        ];

        for (input, expected) in cases {
            let actual = Sensor::parse_sensor(input);

            assert_eq!(
                actual, expected,
                "\n Got {:#?} when expecting {:#?} from calling Sensor::parse_sensor on {:#?}",
                actual, expected, input
            );
        }
    }
}
