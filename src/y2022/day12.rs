use std::{cmp::Reverse, collections::HashMap};

use crate::util::grid_v1::{Grid, GridDirections, Point};

pub fn solve(input: &[&str]) -> String {
    let part1 = Map::parse_map(input).path_len();
    let part2 = Map::parse_map(input).shortest_path_len();

    format!(" Part1: {} \n Part2: {}", part1, part2)
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct MapNode {
    point: Point,
    f_score: i32,
    g_score: i32,
    h_score: i32,
    previous: Option<Point>,
}

#[derive(Debug)]
struct Map {
    tiles: Grid<char>,
    start: Point,
    end: Point,
}

impl Map {
    fn parse_map(input: &[&str]) -> Self {
        let mut tiles = Grid::new(
            input.iter().flat_map(|line| line.chars()).collect(),
            input[0].len(),
            input.len(),
        );
        let mut start = Point::new(0, 0);
        let mut end = Point::new(0, 0);

        for (i, ch) in tiles.spaces.iter_mut().enumerate() {
            if *ch == 'S' {
                *ch = 'a';

                (start.x, start.y) = Grid::<char>::idx_xy_static(i, tiles.width);
            }
            if *ch == 'E' {
                *ch = 'z';

                (end.x, end.y) = Grid::<char>::idx_xy_static(i, tiles.width);
            }
        }

        Map { tiles, start, end }
    }

    fn path_len(&self) -> usize {
        let path = self.search(self.start).unwrap();

        self.tiles.print_path(&path);

        path.len() - 1
    }

    fn shortest_path_len(&self) -> usize {
        let mut shortest = (usize::MAX, vec![]);

        for (i, val) in self.tiles.spaces.iter().enumerate() {
            if *val == 'a' {
                let start = self.tiles.idx_point(i);
                if let Some(path) = self.search(start) {
                    let len = path.len() - 1;
                    if len < shortest.0 {
                        shortest.0 = len;
                        shortest.1 = path;
                    }
                }
            }
        }

        self.tiles.print_path(&shortest.1);

        shortest.0
    }

    fn search(&self, start: Point) -> Option<Vec<Point>> {
        let dest = self.end;

        let mut visited: HashMap<Point, MapNode> = HashMap::new();
        let mut unvisited = Vec::new();

        let start_node = MapNode {
            point: start,
            f_score: i32::MAX,
            g_score: 0,
            // h_score: (dest.x - start.x).abs() + (dest.y - start.y).abs(),
            h_score: 0,
            previous: None,
        };
        unvisited.push(start_node);

        let mut found_destination = false;
        while !found_destination {
            unvisited.sort_by_key(|n| Reverse(n.f_score));
            if let Some(node) = unvisited.pop() {
                let height = *self.tiles.at_point(&node.point).unwrap() as u32;

                for (dx, dy) in [
                    GridDirections::RIGHT,
                    GridDirections::UP,
                    GridDirections::LEFT,
                    GridDirections::DOWN,
                ] {
                    let neighbor = Point::new(node.point.x + dx, node.point.y + dy);

                    if let Some(neighbor_height) = self.tiles.at_point(&neighbor) {
                        if *neighbor_height as u32 <= height + 1 && visited.get(&neighbor).is_none()
                        {
                            // let h = (dest.x - node.point.x).abs() + (dest.y - node.point.y).abs();
                            let h = 0;
                            let g = node.g_score + 1;
                            let f = g + h;

                            if let Some(duplicate) =
                                unvisited.iter_mut().find(|n| n.point == neighbor)
                            {
                                if f < duplicate.f_score {
                                    duplicate.f_score = f;
                                    duplicate.g_score = g;
                                    duplicate.previous = Some(node.point);
                                }
                            } else {
                                unvisited.push(MapNode {
                                    point: neighbor,
                                    f_score: f,
                                    g_score: g,
                                    h_score: h,
                                    previous: Some(node.point),
                                });
                            }

                            if neighbor == dest {
                                visited.insert(
                                    neighbor,
                                    MapNode {
                                        point: neighbor,
                                        f_score: 0,
                                        g_score: node.g_score + 1,
                                        h_score: 0,
                                        previous: Some(node.point),
                                    },
                                );
                                found_destination = true;
                                break;
                            }
                        }
                    }
                }

                visited.insert(node.point, node);
            } else {
                return None;
            }
        }

        let mut dest_node = visited.get(&dest).unwrap();
        let mut route = vec![dest_node.point];

        loop {
            let prev = visited.get(&dest_node.previous.unwrap()).unwrap();
            route.push(prev.point);
            dest_node = prev;
            if dest_node.point == start {
                break;
            }
        }

        Some(route)
    }
}

// fn find_path(input: &[&str]) -> usize {
//     let map = Map::parse_map(input);
//
//     let path = search(map.start, map.end, &map, &HashSet::new()).unwrap();
//
//     // let mut grid = Grid::new(
//     //     vec!['.'; map.tiles.width * map.tiles.height],
//     //     map.tiles.width,
//     //     map.tiles.height,
//     // );
//     //
//     // for point in &path {
//     //     grid.set_at(point.x as usize, point.y as usize, '#')
//     //         .unwrap();
//     //     dbg!(&grid);
//     // }
//
//     path.len() - 1
// }
//
// fn search(start: Point, dest: Point, map: &Map, visited: &HashSet<Point>) -> Option<Vec<Point>> {
//     // eprintln!("");
//     // eprintln!("search(): ");
//     // dbg!(&start);
//     // dbg!(&dest);
//     // dbg!(&map);
//     // dbg!(&visited);
//     let mut visited = visited.clone();
//     visited.insert(start);
//
//     let mut grid = Grid::new(
//         vec!['.'; map.tiles.width * map.tiles.height],
//         map.tiles.width,
//         map.tiles.height,
//     );
//
//     for point in &visited {
//         grid.set_at(point.x as usize, point.y as usize, '#')
//             .unwrap();
//         grid.set_at(start.x as usize, start.y as usize, 'S')
//             .unwrap();
//         grid.set_at(dest.x as usize, dest.y as usize, 'E').unwrap();
//     }
//     dbg!(&grid);
//
//     if start == dest {
//         return Some(vec![dest]);
//     }
//
//     let height = *map.tiles.at_point(&start).unwrap() as u32;
//
//     let neighbors = {
//         let mut neighbors = Vec::new();
//         for (dx, dy) in [
//             GridDirections::RIGHT,
//             GridDirections::LEFT,
//             GridDirections::UP,
//             GridDirections::DOWN,
//         ] {
//             let neighbor = Point::new(start.x + dx, start.y + dy);
//             if let Some(neighbor_height) = map.tiles.at_point(&neighbor) {
//                 if *neighbor_height as u32 <= height + 1 && visited.get(&neighbor).is_none() {
//                     let h = (dest.x - start.x).abs() + (dest.y - start.y).abs();
//                     // + (*neighbor_height as i32 - height as i32).abs();
//                     neighbors.push((neighbor, h));
//                 }
//             }
//         }
//
//         neighbors.sort_by(|a, b| a.1.cmp(&b.1));
//
//         neighbors
//     };
//
//     // dbg!(&neighbors);
//
//     let mut routes = Vec::new();
//     for (neighbor, _) in neighbors {
//         eprintln!("Searching for route from {:?}", neighbor);
//         if let Some(mut route) = search(neighbor, dest, map, &visited) {
//             route.push(start);
//             routes.push(route);
//             eprintln!("Found route from {:?}", neighbor);
//         } else {
//             eprintln!("No route from {:?}", neighbor);
//         }
//     }
//
//     routes.sort_by_key(|v| Reverse(v.len()));
//     // eprintln!("Possible Routes: ");
//     // dbg!(&routes);
//     // eprintln!();
//
//     routes.pop()
// }

#[cfg(test)]
mod tests {

    use super::*;

    // #[test]
    // fn test_find_path() {
    //     let input = ["Sabqponm", "abcryxxl", "accszExk", "acctuvwj", "abdefghi"];
    //
    //     let actual = find_path(&input);
    //     let expected = 31;
    //
    //     assert_eq!(
    //         actual, expected,
    //         "\n Got {actual} when expecting {expected} from calling find_path on {:#?}",
    //         input
    //     );
    // }

    #[test]
    fn test_map_find_path() {
        #[rustfmt::skip]
        let input = [
            "Sabqponm", 
            "abcryxxl", 
            "accszExk", 
            "acctuvwj", 
            "abdefghi"
        ];

        let actual = Map::parse_map(&input).path_len();
        let expected = 31;

        assert_eq!(
            actual, expected,
            "\n Got {actual} when expecting {expected} from calling find_path on {:#?}",
            input
        );
    }

    #[test]
    fn test_map_find_shortest_path() {
        #[rustfmt::skip]
        let input = [
            "Sabqponm", 
            "abcryxxl", 
            "accszExk", 
            "acctuvwj", 
            "abdefghi"
        ];

        let actual = Map::parse_map(&input).shortest_path_len();
        let expected = 29;

        assert_eq!(
            actual, expected,
            "\n Got {actual} when expecting {expected} from calling find_path on {:#?}",
            input
        );
    }
}
