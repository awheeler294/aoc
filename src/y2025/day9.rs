use std::collections::HashSet;
use std::cmp::{min, max};

use crate::util::grid::Point;


pub fn solve(input: &[&str]) -> String {
    let part_1 = solve_part_1(input);
    let part_2 = solve_part_2(input);

    format!(" Part1: {part_1} \n Part2: {part_2}")
}

fn solve_part_1(input: &[&str]) -> usize {
    let points = input
        .into_iter()
        .map(|line|{
            let mut nums = line
                .split(',')
                .map(|n| n.parse::<usize>().unwrap_or_else(|e| panic!("could not parse number from {}: {}", line, e)));
            let x = nums.next().unwrap_or_else(|| panic!("could not find x value in {}", line));
            let y = nums.next().unwrap_or_else(|| panic!("could not find y value in {}", line));

            Point::new(x, y)
        })
        .collect::<Vec<_>>();

    let mut max_area = 0;
    // let mut max_points = None;

    for i in 0..points.len() {
        for j in i+1..points.len() {
            let a = points[i];
            let b = points[j];

            let area = (a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1);
            max_area = max(max_area, area);
        }
    }

    // dbg!(max_points);


    max_area
    
}

fn solve_part_2(input: &[&str]) -> usize {
    let points = input
        .into_iter()
        .map(|line|{
            let mut nums = line
                .split(',')
                .map(|n| n.parse::<usize>().unwrap_or_else(|e| panic!("could not parse number from {}: {}", line, e)));
            let x = nums.next().unwrap_or_else(|| panic!("could not find x value in {}", line));
            let y = nums.next().unwrap_or_else(|| panic!("could not find y value in {}", line));

            Point::new(x, y)
        })
        .collect::<Vec<_>>();


    // let width = 14;
    // let height = 9;
    // let mut grid = Grid::new(width, height, '.');


    let edges = {
        let mut edges: HashSet<Point> = HashSet::new();
        for i in 0..points.len() {
            let j = if i+1 < points.len() { i + 1 } else { 0 };
            let a = points[i];
            let b = points[j];

            if a.x == b.x {
                for y in min(a.y, b.y)..=max(a.y, b.y) {
                    edges.insert(Point { x: a.x, y });
                    // grid[&Point { x: a.x, y }] = '#';
                }
            } else {
                for x in min(a.x, b.x)..=max(a.x, b.x) {
                    edges.insert(Point { x, y: a.y });
                    // grid[&Point { x, y: a.y }] = '#';
                }
            }

            // grid[&a] = 'R';
            // grid[&b] = 'R';
            // dbg!(&grid);
        }

        edges
    };

    let mut max_area = 0;
    // let mut max_points = None;

    for i in 0..points.len() {
        for j in i+1..points.len() {
            let a = points[i];
            let b = points[j];

            let area = (a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1);
            
            if area > max_area {
                let c = Point{ x: a.x, y: b.y };
                let d = Point{ x: b.x, y: a.y };

                // let mut g = grid.clone();
                // g[&a] = 'a';
                // g[&b] = 'b';
                // g[&c] = 'c';
                // g[&d] = 'd';
                //
                // dbg!(g);
                // dbg!(area);

                if (edges.contains(&c) || is_inside(&c, &points)) && (edges.contains(&d) || is_inside(&d, &points)) {
                    
                    let mut is_valid = true;
                    
                    if is_valid {
                        for y in min(a.y, c.y)..max(a.y, c.y) {
                            let p = Point{ x: a.x, y: y };
                            if !edges.contains(&p) && !is_inside(&p, &points) {
                                is_valid = false;
                                break;
                            }
                        }
                    }
                    
                    if is_valid {
                        for x in min(c.x, b.x)..max(c.x, b.x) {
                            let p = Point{ x: x, y: b.y };
                            if !edges.contains(&p) && !is_inside(&p, &points) {
                                is_valid = false;
                                break;
                            }
                        }
                    }
                    
                    if is_valid {
                        for y in min(b.y, d.y)..max(b.y, d.y) {
                            let p = Point{ x: b.x, y: y };
                            if !edges.contains(&p) && !is_inside(&p, &points) {
                                is_valid = false;
                                break;
                            }
                        }
                    }
                    
                    if is_valid {
                        for x in min(d.x, a.x)..max(d.x, a.x) {
                            let p = Point{ x: x, y: d.y };
                            if !edges.contains(&p) && !is_inside(&p, &points) {
                                is_valid = false;
                                break;
                            }
                        }
                    }

                    if is_valid {
                        max_area = area;
                        // max_points = Some((a, b));
                    }
                }
            }
        }
    }

    // dbg!(&max_points);
    
    // ################# visulaization ################

    // {
    //     use crate::util::grid::Grid;
    //     // normalize points
    //     let normalized = points
    //         .iter()
    //         .map(|p| Point::new(p.x / 1000, p.y / 1000))
    //         .collect::<Vec<_>>();
    //
    //     let mut max_x = 0;
    //     let mut min_x = usize::min_value();
    //
    //     let mut max_y = 0;
    //     let mut min_y = usize::max_value();
    //
    //
    //     for p in normalized.iter() {
    //         if p.x > max_x {
    //             max_x = p.x;
    //         } else if p.x < min_x {
    //             min_x = p.x;
    //         }
    //
    //         if p.y > max_y {
    //             max_y = p.y;
    //         } else if p.y < min_y {
    //             min_y = p.y;
    //         }
    //
    //     }
    //
    //     let width = max_x + 2;
    //     let height = max_y + 2;
    //
    //     dbg!(&normalized);
    //     dbg!((min_x, min_y));
    //     dbg!((max_x, max_y));
    //     dbg!((width, height));
    //
    //     let mut grid = Grid::new(width, height, '.');
    //     for i in 0..normalized.len() {
    //         let j = if i+1 < normalized.len() { i + 1 } else { 0 };
    //         let a = normalized[i];
    //         let b = normalized[j];
    //
    //         if a.x == b.x {
    //             grid.draw_vertical_line(&a, &b, '#').unwrap();
    //         } else {
    //             grid.draw_horizontal_line(&a, &b, '#').unwrap();
    //         }
    //
    //         grid[&a] = '#';
    //         grid[&b] = '#';
    //     }
    //
    //     let (a, b) = (
    //         Point{ x: max_points.unwrap().0.x / 1000, y: max_points.unwrap().0.y / 1000 },
    //         Point{ x: max_points.unwrap().1.x / 1000, y: max_points.unwrap().1.y / 1000 },
    //     );
    //
    //     grid[&a] = 'a';
    //     grid[&b] = 'b';
    //
    //     dbg!(&grid);
    // }

    // ################# visulaization ################


    max_area
}

// https://www.geeksforgeeks.org/dsa/how-to-check-if-a-given-point-lies-inside-a-polygon/
fn is_inside(p: &Point, polygon: &[Point]) -> bool {
    let num_vertices = polygon.len();
    let (x, y) = (p.x, p.y);
    let mut inside = false;

    // Store the first point in the polygon and initialize the second point
    let mut p1 = polygon[0];

    // Loop through each edge in the polygon
    for i in 1..num_vertices + 1 {
        // Get the next point in the polygon
        let p2 = polygon[i % num_vertices];

        // Check if the point is above the minimum y coordinate of the edge
        if y > min(p1.y, p2.y) {
            // Check if the point is below the maximum y coordinate of the edge
            if y <= max(p1.y, p2.y) {
                // Check if the point is to the left of the maximum x coordinate of the edge
                if x <= max(p1.x, p2.x) {
                    // Calculate the x-intersection of the line connecting the point to the edge
                    let x_intersection = (y.abs_diff(p1.y)) * (p2.x.abs_diff(p1.x)) / (p2.y.abs_diff(p1.y)) + p1.x;

                    // Check if the point is on the same line as the edge or to the left of the x-intersection
                    if p1.x == p2.x || x <= x_intersection {
                        // Flip the inside flag
                        inside = !inside;
                    }
                }
            }
        }

        // Store the current point as the first point for the next iteration
        p1 = p2
    }

    // Return the value of the inside flag

    inside
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_solve_part_1() {
        #[rustfmt::skip]
        let input = [
            "7,1",
            "11,1",
            "11,7",
            "9,7",
            "9,5",
            "2,5",
            "2,3",
            "7,3",
        ];

        let expected = 50;

        let actual = solve_part_1(&input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_solve_part_2() {
        #[rustfmt::skip]
      let input = [
            "7,1",
            "11,1",
            "11,7",
            "9,7",
            "9,5",
            "2,5",
            "2,3",
            "7,3",
        ];

        let expected = 24;

        let actual = solve_part_2(&input);

        assert_eq!(actual, expected);
    }
}
