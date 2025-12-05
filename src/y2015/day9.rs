use std::collections::{HashMap, HashSet};

pub fn solve(input: &[&str]) -> String {
    let distance_graph = parse_distances(input);
    let part1 = traveling_santaman(&distance_graph);
    let part2 = show_off_santaman(&distance_graph);

    format!(" Part1: {} \n Part2: {}", part1, part2)
}

fn parse_distances(distances: &[&str]) -> HashMap<String, HashMap<String, u32>> {
    let mut distance_graph = HashMap::new();

    for distance in distances {
        let source = distance.split(' ').nth(0).unwrap().to_string();
        let destination = distance.split(' ').nth(2).unwrap().to_string();
        let magnitude = distance.split(' ').nth(4).unwrap().parse::<u32>().unwrap();

        (*distance_graph
            .entry(source.clone())
            .or_insert(HashMap::new()))
        .insert(destination.clone(), magnitude);
        (*distance_graph.entry(destination).or_insert(HashMap::new())).insert(source, magnitude);
    }

    distance_graph
}

fn traveling_santaman(distance_graph: &HashMap<String, HashMap<String, u32>>) -> u32 {
    traveling_santaman_rec(distance_graph, &HashSet::new(), None, 0, u32::MAX)
}

fn traveling_santaman_rec(
    distance_graph: &HashMap<String, HashMap<String, u32>>,
    visited: &HashSet<String>,
    current_location: Option<&str>,
    current_distance: u32,
    shortest_distance: u32,
) -> u32 {
    if visited.len() == distance_graph.len() {
        return current_distance;
    }

    let mut distance = u32::MAX;
    for node in distance_graph.keys() {
        if visited.get(node).is_none() {
            let mut visited = visited.clone();
            visited.insert(node.clone());

            let distance_to = match current_location {
                Some(cl) => *distance_graph.get(cl).unwrap().get(node).unwrap(),
                None => 0,
            };

            let new_distance = current_distance + distance_to;
            if new_distance > shortest_distance {
                return new_distance;
            }

            let distance_from_node = traveling_santaman_rec(
                distance_graph,
                &visited,
                Some(&node),
                new_distance,
                shortest_distance,
            );

            if distance_from_node < distance {
                distance = distance_from_node;
            }
        }
    }

    distance
}

fn show_off_santaman(distance_graph: &HashMap<String, HashMap<String, u32>>) -> u32 {
    show_off_santaman_rec(distance_graph, &HashSet::new(), None, 0, 0)
}

fn show_off_santaman_rec(
    distance_graph: &HashMap<String, HashMap<String, u32>>,
    visited: &HashSet<String>,
    current_location: Option<&str>,
    current_distance: u32,
    longest_distance: u32,
) -> u32 {
    if visited.len() == distance_graph.len() {
        return current_distance;
    }

    let mut distance = 0;
    for node in distance_graph.keys() {
        if visited.get(node).is_none() {
            let mut visited = visited.clone();
            visited.insert(node.clone());

            let distance_to = match current_location {
                Some(cl) => *distance_graph.get(cl).unwrap().get(node).unwrap(),
                None => 0,
            };

            let new_distance = current_distance + distance_to;
            if new_distance < longest_distance {
                return new_distance;
            }

            let distance_from_node = show_off_santaman_rec(
                distance_graph,
                &visited,
                Some(&node),
                new_distance,
                longest_distance,
            );

            if distance_from_node > distance {
                distance = distance_from_node;
            }
        }
    }

    distance
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_distances() {
        #[rustfmt::skip]
        let distances = vec![
            "London to Dublin = 464",
            "London to Belfast = 518",
            "Dublin to Belfast = 141",
        ];

        let expected = HashMap::from([
            (
                "London".to_string(),
                HashMap::from([("Belfast".to_string(), 518), ("Dublin".to_string(), 464)]),
            ),
            (
                "Dublin".to_string(),
                HashMap::from([("Belfast".to_string(), 141), ("London".to_string(), 464)]),
            ),
            (
                "Belfast".to_string(),
                HashMap::from([("Dublin".to_string(), 141), ("London".to_string(), 518)]),
            ),
        ]);

        let actual = parse_distances(&distances);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_traveling_santaman() {
        #[rustfmt::skip]
        let input = vec![
            "London to Dublin = 464",
            "London to Belfast = 518",
            "Dublin to Belfast = 141",
        ];

        let distances = parse_distances(&input);

        let expected = 605;
        let actual = traveling_santaman(&distances);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_show_off_santaman() {
        #[rustfmt::skip]
        let input = vec![
            "London to Dublin = 464",
            "London to Belfast = 518",
            "Dublin to Belfast = 141",
        ];

        let distances = parse_distances(&input);

        let expected = 982;
        let actual = show_off_santaman(&distances);

        assert_eq!(expected, actual);
    }
}
