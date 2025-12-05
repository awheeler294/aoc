use std::collections::{HashMap, HashSet};

pub fn solve(input: &[&str]) -> String {
    let happiness_graph = create_happiness_graph(input);
    dbg!(&happiness_graph);
    let part1 = maximize_happiness(&happiness_graph);
    let part2 = "";

    format!(" Part1: {} \n Part2: {}", part1, part2)
}

fn create_happiness_graph(input: &[&str]) -> HashMap<String, HashMap<String, i32>> {
    let mut graph = HashMap::new();

    for line in input {
        let person = line.split(' ').nth(0).unwrap().to_string();
        let sign = match line.split(' ').nth(2).unwrap() {
            "gain" => 1,
            "lose" => -1,
            _ => panic!("cant parse sign from line: {}", line),
        };
        let amount = line.split(' ').nth(3).unwrap().parse::<i32>().unwrap();
        let other_person = line
            .split(' ')
            .nth(10)
            .unwrap()
            .strip_suffix('.')
            .unwrap()
            .to_string();

        (*graph.entry(person).or_insert(HashMap::new())).insert(other_person, amount * sign);
    }

    graph
}

fn maximize_happiness(happiness_graph: &HashMap<String, HashMap<String, i32>>) -> i32 {
    maximize_happiness_rec(happiness_graph, &HashSet::new(), None, 0, 0)
}

fn maximize_happiness_rec(
    distance_graph: &HashMap<String, HashMap<String, i32>>,
    visited: &HashSet<String>,
    current_location: Option<&str>,
    current_distance: i32,
    longest_distance: i32,
) -> i32 {
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

            let distance_from_node = maximize_happiness_rec(
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
    fn test_create_happiness_graph() {
        #[rustfmt::skip]
        let input = vec![
            "Alice would gain 54 happiness units by sitting next to Bob.",
            "Alice would lose 79 happiness units by sitting next to Carol.",
            "Alice would lose 2 happiness units by sitting next to David.",
            "Bob would gain 83 happiness units by sitting next to Alice.",
            "Bob would lose 7 happiness units by sitting next to Carol.",
            "Bob would lose 63 happiness units by sitting next to David.",
            "Carol would lose 62 happiness units by sitting next to Alice.",
            "Carol would gain 60 happiness units by sitting next to Bob.",
            "Carol would gain 55 happiness units by sitting next to David.",
            "David would gain 46 happiness units by sitting next to Alice.",
            "David would lose 7 happiness units by sitting next to Bob.",
            "David would gain 41 happiness units by sitting next to Carol.",
        ];

        let expected = HashMap::from([
            (
                "Alice".to_string(),
                HashMap::from([
                    ("Bob".to_string(), 54),
                    ("Carol".to_string(), -79),
                    ("David".to_string(), -2),
                ]),
            ),
            (
                "Bob".to_string(),
                HashMap::from([
                    ("Alice".to_string(), 83),
                    ("Carol".to_string(), -7),
                    ("David".to_string(), -63),
                ]),
            ),
            (
                "Carol".to_string(),
                HashMap::from([
                    ("Alice".to_string(), -62),
                    ("Bob".to_string(), 60),
                    ("David".to_string(), 55),
                ]),
            ),
            (
                "David".to_string(),
                HashMap::from([
                    ("Alice".to_string(), 46),
                    ("Bob".to_string(), -7),
                    ("Carol".to_string(), 41),
                ]),
            ),
        ]);

        let actual = create_happiness_graph(&input);

        assert_eq!(actual, expected);
    }

    #[test]
    #[ignore = "TODO"]
    fn test_maximize_happiness() {
        #[rustfmt::skip]
        let input = vec![
            "Alice would gain 54 happiness units by sitting next to Bob.",
            "Alice would lose 79 happiness units by sitting next to Carol.",
            "Alice would lose 2 happiness units by sitting next to David.",
            "Bob would gain 83 happiness units by sitting next to Alice.",
            "Bob would lose 7 happiness units by sitting next to Carol.",
            "Bob would lose 63 happiness units by sitting next to David.",
            "Carol would lose 62 happiness units by sitting next to Alice.",
            "Carol would gain 60 happiness units by sitting next to Bob.",
            "Carol would gain 55 happiness units by sitting next to David.",
            "David would gain 46 happiness units by sitting next to Alice.",
            "David would lose 7 happiness units by sitting next to Bob.",
            "David would gain 41 happiness units by sitting next to Carol.",
        ];

        let happiness_graph = create_happiness_graph(&input);
        let expected = 330;
        let actual = maximize_happiness(&happiness_graph);
        assert_eq!(actual, expected);
    }

    //#[test]
    //fn debug_test() {

    //    #[rustfmt::skip]
    //    let input = vec![
    //        "Alice would lose 57 happiness units by sitting next to Bob.",
    //        "Alice would lose 62 happiness units by sitting next to Carol.",
    //        "Alice would lose 75 happiness units by sitting next to David.",
    //        "Alice would gain 71 happiness units by sitting next to Eric.",
    //        "Alice would lose 22 happiness units by sitting next to Frank.",
    //        "Alice would lose 23 happiness units by sitting next to George.",
    //        "Alice would lose 76 happiness units by sitting next to Mallory.",
    //        "Bob would lose 14 happiness units by sitting next to Alice.",
    //        "Bob would gain 48 happiness units by sitting next to Carol.",
    //        "Bob would gain 89 happiness units by sitting next to David.",
    //        "Bob would gain 86 happiness units by sitting next to Eric.",
    //        "Bob would lose 2 happiness units by sitting next to Frank.",
    //        "Bob would gain 27 happiness units by sitting next to George.",
    //        "Bob would gain 19 happiness units by sitting next to Mallory.",
    //        "Carol would gain 37 happiness units by sitting next to Alice.",
    //        "Carol would gain 45 happiness units by sitting next to Bob.",
    //        "Carol would gain 24 happiness units by sitting next to David.",
    //        "Carol would gain 5 happiness units by sitting next to Eric.",
    //        "Carol would lose 68 happiness units by sitting next to Frank.",
    //        "Carol would lose 25 happiness units by sitting next to George.",
    //        "Carol would gain 30 happiness units by sitting next to Mallory.",
    //        "David would lose 51 happiness units by sitting next to Alice.",
    //        "David would gain 34 happiness units by sitting next to Bob.",
    //        "David would gain 99 happiness units by sitting next to Carol.",
    //        "David would gain 91 happiness units by sitting next to Eric.",
    //        "David would lose 38 happiness units by sitting next to Frank.",
    //        "David would gain 60 happiness units by sitting next to George.",
    //        "David would lose 63 happiness units by sitting next to Mallory.",
    //        "Eric would gain 23 happiness units by sitting next to Alice.",
    //        "Eric would lose 69 happiness units by sitting next to Bob.",
    //        "Eric would lose 33 happiness units by sitting next to Carol.",
    //        "Eric would lose 47 happiness units by sitting next to David.",
    //        "Eric would gain 75 happiness units by sitting next to Frank.",
    //        "Eric would gain 82 happiness units by sitting next to George.",
    //        "Eric would gain 13 happiness units by sitting next to Mallory.",
    //        "Frank would gain 77 happiness units by sitting next to Alice.",
    //        "Frank would gain 27 happiness units by sitting next to Bob.",
    //        "Frank would lose 87 happiness units by sitting next to Carol.",
    //        "Frank would gain 74 happiness units by sitting next to David.",
    //        "Frank would lose 41 happiness units by sitting next to Eric.",
    //        "Frank would lose 99 happiness units by sitting next to George.",
    //        "Frank would gain 26 happiness units by sitting next to Mallory.",
    //        "George would lose 63 happiness units by sitting next to Alice.",
    //        "George would lose 51 happiness units by sitting next to Bob.",
    //        "George would lose 60 happiness units by sitting next to Carol.",
    //        "George would gain 30 happiness units by sitting next to David.",
    //        "George would lose 100 happiness units by sitting next to Eric.",
    //        "George would lose 63 happiness units by sitting next to Frank.",
    //        "George would gain 57 happiness units by sitting next to Mallory.",
    //        "Mallory would lose 71 happiness units by sitting next to Alice.",
    //        "Mallory would lose 28 happiness units by sitting next to Bob.",
    //        "Mallory would lose 10 happiness units by sitting next to Carol.",
    //        "Mallory would gain 44 happiness units by sitting next to David.",
    //        "Mallory would gain 22 happiness units by sitting next to Eric.",
    //        "Mallory would gain 79 happiness units by sitting next to Frank.",
    //        "Mallory would lose 16 happiness units by sitting next to George.",
    //    ];

    //    let happiness_graph = create_happiness_graph(&input);
    //    let expected = -1;
    //    let actual = maximize_happiness(&happiness_graph);
    //    assert_eq!(actual, expected);
    //}
}
