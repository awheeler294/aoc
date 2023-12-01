use std::collections::{HashMap, HashSet};

pub fn solve(_input: &[&str]) -> String {
    let part1 = "";
    let part2 = "";

    format!(" Part1: {} \n Part2: {}", part1, part2)
}

#[derive(Debug, PartialEq, Eq)]
struct Node {
    value: usize,
    adjency: Vec<String>,
}

fn parse_input(input: &[&str]) -> HashMap<String, Node> {
    let mut graph = HashMap::new();

    for line in input {
        let mut tokens = line.split_whitespace();

        let label = tokens.nth(1).unwrap();
        let flow_rate = tokens
            .nth(2)
            .unwrap()
            .split('=')
            .nth(1)
            .unwrap()
            .replace(';', "");
        let value = flow_rate
            .parse::<usize>()
            .unwrap_or_else(|err| panic!("Could not parse `{}` as usize: {}", flow_rate, err));
        tokens.next();
        tokens.next();
        tokens.next();
        tokens.next();
        let mut adjency = Vec::new();
        for tunnel in tokens {
            adjency.push(tunnel.replace(',', ""));
        }

        graph.insert(label.to_string(), Node { value, adjency });
    }

    graph
}

#[allow(dead_code)]
fn find_max_flow_rate(input: &[&str]) -> usize {
    let graph = parse_input(input);

    max_flow_rate("AA", &graph, 30, &HashSet::new())
}

fn max_flow_rate(
    from: &str,
    graph: &HashMap<String, Node>,
    time_remaining: usize,
    visited: &HashSet<String>,
) -> usize {
    let mut visited = visited.clone();

    let current_node = graph.get(from).unwrap();

    let mut results = Vec::new();

    let flow_rate = current_node.value;

    let total_flow = flow_rate * (time_remaining.saturating_sub(1));

    for label in current_node.adjency.iter() {
        if !visited.contains(label) {
            if time_remaining >= 1 {
                results.push((
                    label,
                    max_flow_rate(label, graph, time_remaining - 1, &visited),
                ));
            }
            if flow_rate > 0 && time_remaining >= 2 {
                visited.insert(from.to_string());
                results.push((
                    label,
                    total_flow + max_flow_rate(label, graph, time_remaining - 2, &visited),
                ));
            }
        }
    }

    let l = "stay".to_string();
    results.push((&l, total_flow));

    results.sort_by_key(|e| e.1);

    dbg!(from);
    dbg!(&current_node);
    dbg!(&results);

    results.pop().unwrap().1
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_input() {
        let input = [
            "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB",
            "Valve BB has flow rate=13; tunnels lead to valves CC, AA",
            "Valve CC has flow rate=2; tunnels lead to valves DD, BB",
            "Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE",
            "Valve EE has flow rate=3; tunnels lead to valves FF, DD",
            "Valve FF has flow rate=0; tunnels lead to valves EE, GG",
            "Valve GG has flow rate=0; tunnels lead to valves FF, HH",
            "Valve HH has flow rate=22; tunnel leads to valve GG",
            "Valve II has flow rate=0; tunnels lead to valves AA, JJ",
            "Valve JJ has flow rate=21; tunnel leads to valve II",
        ];

        #[rustfmt::skip]
        let expected = HashMap::from([
            ("AA".to_string(), Node { value: 0,  adjency: vec!["DD".to_string(), "II".to_string(), "BB".to_string()]}),
            ("BB".to_string(), Node { value: 13, adjency: vec!["CC".to_string(), "AA".to_string()]}),
            ("CC".to_string(), Node { value: 2,  adjency: vec!["DD".to_string(), "BB".to_string()]}),
            ("DD".to_string(), Node { value: 20, adjency: vec!["CC".to_string(), "AA".to_string(), "EE".to_string()]}),
            ("EE".to_string(), Node { value: 3,  adjency: vec!["FF".to_string(), "DD".to_string()]}),
            ("FF".to_string(), Node { value: 0,  adjency: vec!["EE".to_string(), "GG".to_string()]}),
            ("GG".to_string(), Node { value: 0,  adjency: vec!["FF".to_string(), "HH".to_string()]}),
            ("HH".to_string(), Node { value: 22, adjency: vec!["GG".to_string()]}),
            ("II".to_string(), Node { value: 0,  adjency: vec!["AA".to_string(), "JJ".to_string()]}),
            ("JJ".to_string(), Node { value: 21, adjency: vec!["II".to_string()]}),
        ]);

        let actual = parse_input(&input);

        assert_eq!(
            actual, expected,
            "\n Got `{:#?}` when expecting `{:#?}` from calling parse_input on {:#?}",
            actual, expected, input
        );
    }

    // #[test]
    fn test_find_max_flow_rate() {
        let input = [
            "Valve AA has flow rate=0; tunnels lead to valves BB",
            "Valve BB has flow rate=13; tunnels lead to valves AA",
        ];

        let actual = find_max_flow_rate(&input);
        let expected = 364;

        assert_eq!(
            actual, expected,
            "\n Got `{:#?}` when expecting `{:#?}` from calling find_max_flow_rate on {:#?}",
            actual, expected, input
        );

        let input = [
            "Valve AA has flow rate=0; tunnels lead to valves BB CC",
            "Valve BB has flow rate=13; tunnels lead to valves AA CC",
            "Valve CC has flow rate=2; tunnels lead to valves AA, BB",
        ];

        let actual = find_max_flow_rate(&input);
        let expected = 364 + 52;

        assert_eq!(
            actual, expected,
            "\n Got `{:#?}` when expecting `{:#?}` from calling find_max_flow_rate on {:#?}",
            actual, expected, input
        );

        let graph = HashMap::from([(
            "CC".to_string(),
            Node {
                value: 2,
                adjency: vec![],
            },
        )]);
        let time_remaining = 5;
        let actual = max_flow_rate("CC", &graph, time_remaining + 1, &HashSet::new());
        let expected = graph.get("CC").unwrap().value * time_remaining;
        assert_eq!(
            actual, expected,
            "\n Got `{:#?}` when expecting `{:#?}`",
            actual, expected
        );

        let input = [
            "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB",
            "Valve BB has flow rate=13; tunnels lead to valves CC, AA",
            "Valve CC has flow rate=2; tunnels lead to valves DD, BB",
            "Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE",
            "Valve EE has flow rate=3; tunnels lead to valves FF, DD",
            "Valve FF has flow rate=0; tunnels lead to valves EE, GG",
            "Valve GG has flow rate=0; tunnels lead to valves FF, HH",
            "Valve HH has flow rate=22; tunnel leads to valve GG",
            "Valve II has flow rate=0; tunnels lead to valves AA, JJ",
            "Valve JJ has flow rate=21; tunnel leads to valve II",
        ];

        let actual = find_max_flow_rate(&input);
        let expected = 1651;

        assert_eq!(
            actual, expected,
            "\n Got `{:#?}` when expecting `{:#?}` from calling find_max_flow_rate on {:#?}",
            actual, expected, input
        );
    }
}
