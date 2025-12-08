use std::{
    collections::{BinaryHeap, HashMap},
    hash::Hash,
    io::stdout,
    thread,
    time::Duration,
    usize,
};

use log_update::LogUpdate;

use crate::util::grid::{Grid, GridDirection, Point};

pub fn solve(input: &[&str]) -> String {
    let part1 = min_path_cost(input);
    let part2 = "";

    format!(" Part1: {} \n Part2: {}", part1, part2)
}

fn min_path_cost(input: &[&str]) -> usize {
    let grid = Grid::parse(input, |c| c.to_digit(10).unwrap() as usize);

    // let path = min_path(Crucible::new(), &grid, &HashSet::new()).unwrap();
    let path = min_path(&grid);

    let mut total_cost = 0;
    let mut path_grid = Grid::parse_char(input);
    for c in path.iter().skip(1) {
        total_cost += grid[&c.position];
        path_grid[&c.position] = match c.direction {
            GridDirection::Up => '^',
            GridDirection::Down => 'v',
            GridDirection::Left => '<',
            GridDirection::Right => '>',
            _ => unreachable!(),
        };
    }
    dbg!(path_grid);

    total_cost
}

fn min_path(weights: &Grid<usize>) -> Vec<Crucible> {
    let goal = Point::new(weights.width - 1, weights.height - 1);

    let mut came_from: HashMap<Crucible, Option<Crucible>> = HashMap::new();
    let mut costs: HashMap<Point, Crucible> = HashMap::new();

    let crucible = Crucible::new();
    came_from.insert(crucible, None);
    costs.insert(crucible.position, crucible);

    let mut open = BinaryHeap::new();
    open.push(crucible);

    let mut log_update = LogUpdate::new(stdout()).unwrap();
    while let Some(q) = open.pop() {
        // dbg!(&open);
        // dbg!(&q);

        let mut debug_grid = Grid::new(weights.width, weights.height, '.');
        debug_grid[(0, 0)] = 'S';
        debug_grid[&goal] = 'G';
        for (c, _) in came_from.iter() {
            debug_grid[&c.position] = match c.direction {
                GridDirection::Up => '^',
                GridDirection::Down => 'v',
                GridDirection::Left => '<',
                GridDirection::Right => '>',
                _ => unreachable!(),
            };
        }
        debug_grid[&q.position] = '#';

        for next in q.possible_moves(weights) {
            // dbg!(&next);
            debug_grid[&next.position] = 'N';

            // let c = costs.get(&next.position);
            // dbg!(c);
            if let Some(c) = costs.get(&next.position) {
                if next.direction == c.direction && next.cost >= c.cost {
                    // dbg!("continue");
                    continue;
                }
            }

            costs
                .entry(next.position)
                .and_modify(|c| *c = next)
                .or_insert(next);
            // dbg!(&costs);

            came_from
                .entry(next)
                .and_modify(|v| *v = Some(q))
                .or_insert(Some(q));

            // if next.position == goal {
            //     // dbg!("goal");
            //     break;
            // }

            // dbg!("push");
            open.push(next);
        }

        log_update.render(&format!("{debug_grid}")).unwrap();
        thread::sleep(Duration::from_millis(1000 / 10));
        // thread::sleep(Duration::from_millis(1000));
    }

    // dbg!(&came_from);
    let path_end = came_from
        .iter()
        .filter_map(|(c, _)| if c.position == goal { Some(c) } else { None })
        .last()
        .unwrap();
    let mut path = vec![*path_end];
    loop {
        let current = path.last().unwrap();
        let parent = came_from.get(&current).unwrap();
        if parent.is_none() {
            break;
        }
        path.push(parent.unwrap());
    }

    // dbg!(&path);

    path.into_iter().rev().collect()
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct Crucible {
    steps: usize,
    direction: GridDirection,
    position: Point,
    cost: usize,
    h: usize,
    f: usize,
}

impl Hash for Crucible {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.position.hash(state)
    }
}

impl PartialOrd for Crucible {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.cost.cmp(&self.cost))
    }
}

impl Ord for Crucible {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.f.cmp(&self.f)
    }
}

impl Crucible {
    fn new() -> Self {
        Self {
            steps: 0,
            direction: GridDirection::Right,
            position: Point { x: 0, y: 0 },
            cost: 0,
            h: 0,
            f: 0,
        }
    }

    fn possible_moves(&self, weights: &Grid<usize>) -> Vec<Crucible> {
        match self.direction {
            GridDirection::Up => [GridDirection::Up, GridDirection::Right, GridDirection::Left],
            GridDirection::Down => [
                GridDirection::Down,
                GridDirection::Right,
                GridDirection::Left,
            ],
            GridDirection::Left => [GridDirection::Left, GridDirection::Up, GridDirection::Down],
            GridDirection::Right => [GridDirection::Right, GridDirection::Down, GridDirection::Up],
            _ => unreachable!(),
        }
        .iter()
        .filter_map(|d| {
            if let Some(weight) = weights.get_direction(&self.position, *d) {
                self.genetate_move(
                    *d,
                    *weight,
                    Point {
                        x: weights.width - 1,
                        y: weights.height - 1,
                    },
                )
            } else {
                None
            }
        })
        .collect()
    }

    fn genetate_move(&self, direction: GridDirection, weight: usize, goal: Point) -> Option<Self> {
        if direction == self.direction && self.steps >= 3 {
            return None;
        }

        if direction == GridDirection::Up && self.position.y == 0 {
            return None;
        }

        if direction == GridDirection::Left && self.position.x == 0 {
            return None;
        }

        let steps = {
            if direction == self.direction {
                self.steps + 1
            } else {
                1
            }
        };

        let position = match direction {
            GridDirection::Up => Point {
                x: self.position.x,
                y: self.position.y - 1,
            },
            GridDirection::Down => Point {
                x: self.position.x,
                y: self.position.y + 1,
            },
            GridDirection::Left => Point {
                x: self.position.x - 1,
                y: self.position.y,
            },
            GridDirection::Right => Point {
                x: self.position.x + 1,
                y: self.position.y,
            },
            _ => unreachable!(),
        };

        let cost = self.cost + weight;
        let h = position.manhattan_distance(goal);
        let f = h + cost;

        Some(Self {
            position,
            direction,
            steps,
            cost,
            h,
            f,
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    #[ignore]
    fn test_min_path_cost() {
        #[rustfmt::skip]
        let input = [
            "2413432311323",
            "3215453535623",
            "3255245654254",
            "3446585845452",
            "4546657867536",
            "1438598798454",
            "4457876987766",
            "3637877979653",
            "4654967986887",
            "4564679986453",
            "1224686865563",
            "2546548887735",
            "4322674655533",
        ];

        let expected = 102;
        let actual = min_path_cost(&input);

        assert_eq!(actual, expected);
    }

    #[test]
    #[ignore]
    fn test_min_path_cost2() {
        let input = ["11111", "91191", "99991"];

        let expected = 8;
        let actual = min_path_cost(&input);

        assert_eq!(actual, expected);
    }
}
