use core::panic;
use std::{collections::{HashMap, HashSet}, usize};

use crate::util::point::Point3;

pub fn solve(input: &[&str]) -> String {
    let part_1 = solve_part_1(1000, input);
    let part_2 = solve_part_2(input);

    format!(" Part1: {part_1} \n Part2: {part_2}")
}

fn solve_part_1(n: usize, input: &[&str]) -> usize {
    
    // parse input into points
    let points: Vec<Point3> = input
        .iter()
        .map(|line| {
            let mut nums = line.split(',');
            let x = nums
                .next()
                .unwrap_or_else(|| panic!("no x in line: {}", line))
                .parse::<usize>()
                .unwrap_or_else(|e| panic!("could not parse x from {}: {}", line, e));
            let y = nums
                .next()
                .unwrap_or_else(|| panic!("no y in line: {}", line))
                .parse::<usize>()
                .unwrap_or_else(|e| panic!("could not parse y from {}: {}", line, e));
            let z = nums
                .next()
                .unwrap_or_else(|| panic!("no z in line: {}", line))
                .parse::<usize>()
                .unwrap_or_else(|e| panic!("could not parse z from {}: {}", line, e));

            Point3::new(x, y, z)
        })
        .collect::<Vec<_>>();

    // calculate distance between all point pairs and sort
    let distances = {
        let mut distances = vec![];
        for i in 0..points.len() {
            for j in i+1..points.len() {
                let distance = points[i].euclidean_distance(points[j]);
                distances.push((distance, &points[i], &points[j]));
            }
        }

        distances.sort_by(|a, b| a.0.total_cmp(&b.0));

        distances
    };

    // dbg!(&distances);

    // create circuits out of the first n point pairs
    let mut point_to_circuit: HashMap<&Point3, usize> = HashMap::new();
    let mut next_circuit_id = 0;

    for i in 0..n {
        if i >= distances.len() {
            break;
        }

        let (_d, a, b) = distances[i];
        
        if let Some(a_cid) = point_to_circuit.get(a) && let Some(b_cid) = point_to_circuit.get(b) {

            if *a_cid != *b_cid {

                let to_merge = b_cid.clone();
                let merged_cid = a_cid.clone();
                
                for (_, cid) in point_to_circuit.iter_mut() {
                    if *cid == to_merge { 
                        *cid = merged_cid;
                    }
                }
            }

        } else if let Some(cid) = point_to_circuit.get(a) {
        
            point_to_circuit.insert(b, *cid);

        } else if let Some(cid) = point_to_circuit.get(b) {
            
            point_to_circuit.insert(a, *cid);

        } else {
            
            point_to_circuit.insert(a, next_circuit_id);
            point_to_circuit.insert(b, next_circuit_id);
            next_circuit_id += 1;

        }
    }

    // dbg!(&point_to_circuit);

    // transform point-to-circuit into circuit-to-point representation
    let mut circuits: HashMap<usize, Vec<&Point3>> = HashMap::new();

    for (point, cid) in point_to_circuit {
        circuits.entry(cid).and_modify(|points| (*points).push(point)).or_insert(vec![point]);
    }

    // dbg!(&circuits);

    // calculate sizes of circuits and sort
    let circuit_sizes = {
        let mut sorted_circuits = circuits
            .iter()
            .map(|(_cid, points)| points.len())
            .collect::<Vec<_>>();
        sorted_circuits.sort_by(|a, b| b.cmp(a));
        sorted_circuits
    };

    // dbg!(&circuit_sizes);

    // result
    circuit_sizes.iter().take(3).product()
}

fn solve_part_2(input: &[&str]) -> usize {
    
    // parse input into points
    let points: Vec<Point3> = input
        .iter()
        .map(|line| {
            let mut nums = line.split(',');
            let x = nums
                .next()
                .unwrap_or_else(|| panic!("no x in line: {}", line))
                .parse::<usize>()
                .unwrap_or_else(|e| panic!("could not parse x from {}: {}", line, e));
            let y = nums
                .next()
                .unwrap_or_else(|| panic!("no y in line: {}", line))
                .parse::<usize>()
                .unwrap_or_else(|e| panic!("could not parse y from {}: {}", line, e));
            let z = nums
                .next()
                .unwrap_or_else(|| panic!("no z in line: {}", line))
                .parse::<usize>()
                .unwrap_or_else(|e| panic!("could not parse z from {}: {}", line, e));

            Point3::new(x, y, z)
        })
        .collect::<Vec<_>>();

    // calculate distance between all point pairs and sort
    let distances = {
        let mut distances = vec![];
        for i in 0..points.len() {
            for j in i+1..points.len() {
                let distance = points[i].euclidean_distance(points[j]);
                distances.push((distance, &points[i], &points[j]));
            }
        }

        distances.sort_by(|a, b| a.0.total_cmp(&b.0));

        distances
    };

    // dbg!(&distances);

    // create circuits out of the first n point pairs
    let mut point_to_circuit: HashMap<&Point3, usize> = HashMap::new();
    let mut circuits: HashMap<usize, HashSet<&Point3>> = HashMap::new();
    let mut next_circuit_id = 0;
    let mut result = 0;

    for i in 0..distances.len() {

        let (_d, a, b) = distances[i];
        
        if let Some(a_cid) = point_to_circuit.get(a) && let Some(b_cid) = point_to_circuit.get(b) {

            if *a_cid != *b_cid {

                let to_merge = b_cid.clone();
                let merged_cid = a_cid.clone();
                
                for (_, cid) in point_to_circuit.iter_mut() {
                    if *cid == to_merge { 
                        *cid = merged_cid;
                    }
                }

                if let Some(points_to_merge) = circuits.remove(&to_merge) {
                    circuits.entry(merged_cid).and_modify(|points|(*points).extend(points_to_merge.into_iter()));
                }
            }

        } else if let Some(cid) = point_to_circuit.get(a) {
        
            circuits.entry(*cid).and_modify(|points| { (*points).insert(b); });
            point_to_circuit.insert(b, *cid);

        } else if let Some(cid) = point_to_circuit.get(b) {
            
            circuits.entry(*cid).and_modify(|points| { (*points).insert(a); });
            point_to_circuit.insert(a, *cid);

        } else {
            
            circuits.insert(next_circuit_id, HashSet::from([a, b]));
            point_to_circuit.insert(a, next_circuit_id);
            point_to_circuit.insert(b, next_circuit_id);
            next_circuit_id += 1;

        }

        // we found the last connection if there is one circuit that contains all the points
        if circuits.len() == 1 && circuits.iter().last().unwrap().1.len() == points.len() {
            result = a.x * b.x;
            break;
        }
    }

    // dbg!(&point_to_circuit);
    // dbg!(&circuits);

    result
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_solve_part_1() {
        #[rustfmt::skip]
      let input = [
            "162,817,812",
            "57,618,57",
            "906,360,560",
            "592,479,940",
            "352,342,300",
            "466,668,158",
            "542,29,236",
            "431,825,988",
            "739,650,466",
            "52,470,668",
            "216,146,977",
            "819,987,18",
            "117,168,530",
            "805,96,715",
            "346,949,466",
            "970,615,88",
            "941,993,340",
            "862,61,35",
            "984,92,344",
            "425,690,689",
        ];

        let expected = 40;

        let actual = solve_part_1(10, &input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_solve_part_2() {
        #[rustfmt::skip]
      let input = [
            "162,817,812",
            "57,618,57",
            "906,360,560",
            "592,479,940",
            "352,342,300",
            "466,668,158",
            "542,29,236",
            "431,825,988",
            "739,650,466",
            "52,470,668",
            "216,146,977",
            "819,987,18",
            "117,168,530",
            "805,96,715",
            "346,949,466",
            "970,615,88",
            "941,993,340",
            "862,61,35",
            "984,92,344",
            "425,690,689",
        ];

        let expected = 25272;

        let actual = solve_part_2(&input);

        assert_eq!(actual, expected);
    }
}
