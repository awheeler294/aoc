use std::collections::{HashMap, VecDeque};

pub fn solve(input: &[&str]) -> String {
    let part_1 = solve_part_1(input);
    let part_2 = solve_part_2(input);

    format!(" Part1: {part_1} \n Part2: {part_2}")
}

fn solve_part_1(input: &[&str]) -> u32 {
    let graph = input
        .iter()
        .map(|line| {
            let (name, outputs) = line
                .split_once(':')
                .unwrap_or_else(|| panic!("Parse Error - Could not find ':' in `{}`", line));

            (name, outputs.split_whitespace().collect::<Vec<_>>())
        })
        .collect::<HashMap<_, _>>();

    let mut to_visit = VecDeque::from(["you"]);
    let mut paths = 0;

    while !to_visit.is_empty() {
        let node = to_visit.pop_front().unwrap();

        for &output in graph.get(&node).unwrap().iter() {
            if output == "out" {
                paths += 1;
            } else {
                to_visit.push_back(&output);
            }
        }
    }

    paths
}

fn solve_part_2(input: &[&str]) -> u32 {
    let graph = input
        .iter()
        .map(|line| {
            let (name, outputs) = line
                .split_once(':')
                .unwrap_or_else(|| panic!("Parse Error - Could not find ':' in `{}`", line));

            (name, outputs.split_whitespace().collect::<Vec<_>>())
        })
        .collect::<HashMap<_, _>>();

    fn valid_routes<'a>(
        position: &'a str,
        visited_dac: bool,
        visited_fft: bool,
        graph: &HashMap<&str, Vec<&'a str>>,
        memo: &mut HashMap<(&'a str, bool, bool), u32>,
    ) -> u32 {
        // dbg!(position);
        if visited_dac && visited_fft && position == "out" {
            return 1;
        }

        let visited_dac = visited_dac || position == "dac";
        let visited_fft = visited_fft || position == "fft";

        if let Some(count) = memo.get(&(position, visited_dac, visited_fft)) {
            return *count;
        }

        let Some(outputs) = graph.get(position) else {
            return 0;
        };

        let mut count = 0;

        for output in outputs {
            count += valid_routes(output, visited_dac, visited_fft, graph, memo);
        }

        memo.insert((position, visited_dac, visited_fft), count);

        count
    }

    valid_routes("svr", false, false, &graph, &mut HashMap::new())
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_solve_part_1() {
        #[rustfmt::skip]
        let input = [
            "aaa: you hhh",
            "you: bbb ccc",
            "bbb: ddd eee",
            "ccc: ddd eee fff",
            "ddd: ggg",
            "eee: out",
            "fff: out",
            "ggg: out",
            "hhh: ccc fff iii",
            "iii: out",
        ];

        let expected = 5;

        let actual = solve_part_1(&input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_solve_part_2() {
        #[rustfmt::skip]
        let input = [
            "svr: aaa bbb",
            "aaa: fft",
            "fft: ccc",
            "bbb: tty",
            "tty: ccc",
            "ccc: ddd eee",
            "ddd: hub",
            "hub: fff",
            "eee: dac",
            "dac: fff",
            "fff: ggg hhh",
            "ggg: out",
            "hhh: out",
        ];

        let expected = 2;

        let actual = solve_part_2(&input);

        assert_eq!(actual, expected);
    }
}
