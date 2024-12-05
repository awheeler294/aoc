use std::{collections::HashMap, ops::Range};

use regex::Regex;

pub fn solve(input: &[&str]) -> String {
    let part1 = process_parts(&input);
    let part2 = "";

    format!(" Part1: {} \n Part2: {}", part1, part2)
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct PartRange {
    x: Range<usize>,
    m: Range<usize>,
    a: Range<usize>,
    s: Range<usize>,
}

#[allow(dead_code)]
fn calculate_combinations(input: &[&str]) -> usize {
    let workflows = parse_workflows(input);

    let ranges = PartRange {
        x: 1..4000,
        m: 1..4000,
        a: 1..4000,
        s: 1..4000,
    };

    count_combinations("in", ranges, &workflows)
}

#[allow(dead_code)]
fn count_combinations(
    start: &str,
    ranges: PartRange,
    workflows: &HashMap<&str, Vec<Rule>>,
) -> usize {
    let workflow = workflows.get(start).unwrap();

    let mut count = 0;

    let mut non_passed_ranges = ranges.clone();

    for rule in workflow.iter() {
        dbg!(&start);
        dbg!(&non_passed_ranges);
        let mut passed_ranges = non_passed_ranges.clone();

        if let Some((class, operator)) = &rule.operation {
            let (pr, npr, value) = match class {
                Class::X(value) => (&mut passed_ranges.x, &mut non_passed_ranges.x, value),
                Class::M(value) => (&mut passed_ranges.m, &mut non_passed_ranges.m, value),
                Class::A(value) => (&mut passed_ranges.a, &mut non_passed_ranges.a, value),
                Class::S(value) => (&mut passed_ranges.s, &mut non_passed_ranges.s, value),
            };

            match operator {
                Operator::Gt => {
                    pr.start = *value;
                    npr.end = *value;
                }
                Operator::Lt => {
                    pr.end = *value;
                    npr.start = *value;
                }
            }
        }

        match &rule.result {
            Action::Accept => {
                dbg!(&passed_ranges);
                dbg!("Accept!");
                count += (passed_ranges.x.end - passed_ranges.x.start)
                    * (passed_ranges.m.end - passed_ranges.m.start)
                    * (passed_ranges.a.end - passed_ranges.a.start)
                    * (passed_ranges.s.end - passed_ranges.s.start);
            }
            Action::Reject => {
                dbg!(&passed_ranges);
                dbg!("Reject!");
            }
            Action::Workflow(key) => count += count_combinations(&key, passed_ranges, workflows),
        }
    }

    count
}

fn process_parts(input: &[&str]) -> usize {
    let workflows = parse_workflows(input);
    let parts = parse_parts(input);
    let mut accepted = vec![];

    for part in parts.iter() {
        // dbg!(&part);
        let mut workflow = workflows.get("in").unwrap();
        let mut process = true;

        while process {
            // dbg!(&workflow);
            for rule in workflow.iter() {
                // dbg!(&rule);
                let action = rule.apply(&part);
                // dbg!(&action);
                match action {
                    Some(Action::Accept) => {
                        accepted.push(part);
                        process = false;
                        break;
                    }

                    Some(Action::Reject) => {
                        process = false;
                        break;
                    }

                    Some(Action::Workflow(label)) => {
                        workflow = workflows.get(label.as_str()).unwrap();
                        break;
                    }

                    None => {}
                }
            }
        }
    }

    accepted
        .iter()
        .fold(0, |acc, part| acc + part.x + part.m + part.a + part.s)
}

fn parse_workflows<'a>(input: &[&'a str]) -> HashMap<&'a str, Vec<Rule>> {
    let rule_re =
        Regex::new(r"(?P<class>[xmas])(?P<operator>[><])(?P<value>\d+):(?P<result>([a-z]+|R|A))")
            .unwrap();
    let default_re = Regex::new(r"(\w+)\}").unwrap();

    let mut workflows: HashMap<&str, Vec<Rule>> = HashMap::new();

    let mut input = input.iter();
    loop {
        let line = input.next().unwrap();
        if line.is_empty() {
            break;
        }

        let (label, rest) = line.split_once('{').unwrap();
        let mut rules = vec![];

        for rule in rest.split(',') {
            if let Some(captures) = rule_re.captures(rule) {
                let operator = Operator::from_str(&captures["operator"]);
                let value = &captures["value"].parse::<usize>().unwrap();
                let class = Class::parse(&captures["class"], *value);
                let result = Action::from_str(&captures["result"]);

                let rule = Rule::new(Some((class, operator)), result);
                rules.push(rule);
            }

            for capture in default_re.captures_iter(rule) {
                let (_, [c]) = capture.extract();
                let default = Action::from_str(c);
                rules.push(Rule::new(None, default));
            }
        }

        workflows.insert(label, rules);
    }

    workflows
}

fn parse_parts(input: &[&str]) -> Vec<Part> {
    let part_re = Regex::new(r"\{x=(?P<x>\d+),m=(?P<m>\d+),a=(?P<a>\d+),s=(?P<s>\d+)\}").unwrap();
    input
        .iter()
        .filter_map(|l| {
            if let Some(captures) = part_re.captures(l) {
                let x = &captures["x"].parse::<usize>().unwrap();
                let m = &captures["m"].parse::<usize>().unwrap();
                let a = &captures["a"].parse::<usize>().unwrap();
                let s = &captures["s"].parse::<usize>().unwrap();

                Some(Part {
                    x: *x,
                    m: *m,
                    a: *a,
                    s: *s,
                })
            } else {
                None
            }
        })
        .collect()
}

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

#[derive(Debug)]
enum Action {
    Accept,
    Reject,
    Workflow(String),
}

impl Action {
    fn from_str(value: &str) -> Self {
        match value {
            "A" => Self::Accept,
            "R" => Self::Reject,
            _ => Self::Workflow(value.to_string()),
        }
    }
}

#[derive(Debug)]
enum Class {
    X(usize),
    M(usize),
    A(usize),
    S(usize),
}

impl Class {
    fn parse(class: &str, value: usize) -> Self {
        match class {
            "x" => Self::X(value),
            "m" => Self::M(value),
            "a" => Self::A(value),
            "s" => Self::S(value),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum Operator {
    Gt,
    Lt,
}

impl Operator {
    fn from_str(value: &str) -> Self {
        match value {
            "<" => Self::Lt,
            ">" => Self::Gt,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Rule {
    operation: Option<(Class, Operator)>,
    result: Action,
}

impl Rule {
    fn new(operation: Option<(Class, Operator)>, result: Action) -> Self {
        Self { operation, result }
    }

    fn apply(&self, part: &Part) -> Option<&Action> {
        if let Some((class, operator)) = &self.operation {
            let (lhs, rhs) = match class {
                Class::X(value) => (part.x, value),
                Class::M(value) => (part.m, value),
                Class::A(value) => (part.a, value),
                Class::S(value) => (part.s, value),
            };

            let result = match operator {
                Operator::Lt => lhs < *rhs,
                Operator::Gt => lhs > *rhs,
            };

            if result {
                return Some(&self.result);
            } else {
                return None;
            }
        }

        return Some(&self.result);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_process_parts() {
        let input = [
            "px{a<2006:qkq,m>2090:A,rfg}",
            "pv{a>1716:R,A}",
            "lnx{m>1548:A,A}",
            "rfg{s<537:gd,x>2440:R,A}",
            "qs{s>3448:A,lnx}",
            "qkq{x<1416:A,crn}",
            "crn{x>2662:A,R}",
            "in{s<1351:px,qqz}",
            "qqz{s>2770:qs,m<1801:hdj,R}",
            "gd{a>3333:R,R}",
            "hdj{m>838:A,pv}",
            "",
            "{x=787,m=2655,a=1222,s=2876}",
            "{x=1679,m=44,a=2067,s=496}",
            "{x=2036,m=264,a=79,s=2244}",
            "{x=2461,m=1339,a=466,s=291}",
            "{x=2127,m=1623,a=2188,s=1013}",
        ];

        let expected = 19114;
        let actual = process_parts(&input);

        assert_eq!(actual, expected);
    }

    #[test]
    #[ignore = "part 2 not working"]
    fn test_calculate_combinations() {
        let input = [
            "px{a<2006:qkq,m>2090:A,rfg}",
            "pv{a>1716:R,A}",
            "lnx{m>1548:A,A}",
            "rfg{s<537:gd,x>2440:R,A}",
            "qs{s>3448:A,lnx}",
            "qkq{x<1416:A,crn}",
            "crn{x>2662:A,R}",
            "in{s<1351:px,qqz}",
            "qqz{s>2770:qs,m<1801:hdj,R}",
            "gd{a>3333:R,R}",
            "hdj{m>838:A,pv}",
            "",
            "{x=787,m=2655,a=1222,s=2876}",
            "{x=1679,m=44,a=2067,s=496}",
            "{x=2036,m=264,a=79,s=2244}",
            "{x=2461,m=1339,a=466,s=291}",
            "{x=2127,m=1623,a=2188,s=1013}",
        ];

        let expected = 167409079868000;
        let actual = calculate_combinations(&input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_calculate_combinations_max() {
        #[rustfmt::skip]
        let input = [
            "in{A}", 
            "", 
            "{x=787,m=2655,a=1222,s=2876}"
        ];

        let expected = 255744095984001;
        let actual = calculate_combinations(&input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_calculate_combinations_branching_max() {
        #[rustfmt::skip]
        let input = [
            "in{x<1:mas,mas}",
            "mas{m<1:as,as}",
            "as{a<1:s,s}",
            "s{s<1:A,A}",
            "",
            "{x=787,m=2655,a=1222,s=2876}"
        ];

        let expected = 255744095984001;
        let actual = calculate_combinations(&input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_calculate_combinations_min() {
        #[rustfmt::skip]
        let input = [
            "in{x<1:mas,R}", 
            "mas{m<1:as,R}", 
            "as{a<1:s,R}", 
            "s{s<1:A,R}", 
            "", 
            "{x=787,m=2655,a=1222,s=2876}"
        ];

        let expected = 0;
        let actual = calculate_combinations(&input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_calculate_combinations_identity() {
        #[rustfmt::skip]
        let input = [
            "in{x<2:mas,R}",
            "mas{m<2:as,R}",
            "as{a<2:s,R}",
            "s{s<2:A,R}",
            "",
            "{x=787,m=2655,a=1222,s=2876}"
        ];

        let expected = 1;
        let actual = calculate_combinations(&input);

        assert_eq!(actual, expected);
    }
}
