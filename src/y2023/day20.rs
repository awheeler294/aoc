use std::{
    collections::{HashMap, VecDeque},
    hash::Hash,
};

use crate::algorithims::least_common_multiple;

pub fn solve(input: &[&str]) -> String {
    let part1 = run_circuit(1000, input);
    let part2 = find_needed_iterations(input);

    format!(" Part1: {} \n Part2: {}", part1, part2)
}

fn find_needed_iterations(input: &[&str]) -> usize {
    let modules = parse_modules(input);
    dbg!(find_cycle_len(
        &"dc".to_string(),
        &"tx".to_string(),
        &modules
    ));

    let rx_control = modules
        .iter()
        .find(|(_k, v)| v.destination_modules.contains(&"rx".to_string()))
        .unwrap()
        .0;
    let needed_modules = modules.iter().filter_map(|(k, v)| {
        if v.destination_modules.contains(rx_control) {
            Some(k)
        } else {
            None
        }
    });

    let cycle_lengths = needed_modules
        .map(|m| find_cycle_len(m, &rx_control, &modules))
        .collect::<Vec<_>>();

    dbg!(&cycle_lengths);

    cycle_lengths
        .iter()
        .fold(1, |acc, l| least_common_multiple(acc, *l))
}

fn find_cycle_len(
    source_target: &String,
    destination_target: &String,
    modules: &HashMap<String, Module>,
) -> usize {
    let mut modules: HashMap<String, Module> = modules.clone();
    // dbg!(&modules);
    // dbg!((&source_target, &destination_target));

    let mut cycles = 0;

    let mut to_process = VecDeque::new();

    loop {
        to_process.push_back(("broadcaster".to_string(), false, "broadcaster".to_string()));
        cycles += 1;

        // dbg!(&modules);
        while let Some((source, signal, label)) = to_process.pop_front() {
            // dbg!((&source, &signal, &label));
            let module = modules
                .entry(label.clone())
                .or_insert_with(|| Module::new(ModuleType::Untyped, vec![]));
            if let Some((new_signal, destinations)) = module.process_signal(signal, &source) {
                for d in destinations {
                    // if label == *source_target && d == *destination_target && signal == true && cycles > 3 {
                    //     dbg!((&label, &d, &signal));
                    //     return cycles;
                    // }
                    to_process.push_back((label.clone(), new_signal, d));
                }
            }
            // dbg!(&modules);
        }

        // dbg!(cycles);
        if let ModuleType::Conjunction(state) =
            &modules.get(destination_target).unwrap().module_type
        {
            // dbg!(&state);
            if *state.get(source_target).unwrap() {
                break cycles;
            }
        }
    }

    // dbg!(cycles);
    // cycles
}

fn run_circuit(iterations: usize, input: &[&str]) -> usize {
    let mut modules = parse_modules(input);

    let mut to_process = VecDeque::new();

    for _ in 0..iterations {
        to_process.push_back(("broadcaster".to_string(), false, "broadcaster".to_string()));

        // dbg!(&modules);
        while let Some((source, signal, label)) = to_process.pop_front() {
            // dbg!((&source, &signal, &label));
            let module = modules
                .entry(label.clone())
                .or_insert_with(|| Module::new(ModuleType::Untyped, vec![]));
            if let Some((new_signal, destinations)) = module.process_signal(signal, &source) {
                for d in destinations {
                    to_process.push_back((label.clone(), new_signal, d));
                }
            }
            // dbg!(&modules);
        }
    }

    // dbg!(&modules);
    modules.values().map(|m| m.high_pulses).sum::<usize>()
        * modules.values().map(|m| m.low_pulses).sum::<usize>()
}

fn parse_modules<'a>(input: &[&str]) -> HashMap<String, Module> {
    let mut conjunctions = vec![];

    let mut modules = input
        .into_iter()
        .map(|line| {
            let (module, destinations) = line.split_once("->").unwrap();
            let destinations = destinations
                .split(',')
                .map(|d| d.trim().to_string())
                .collect::<Vec<String>>();
            let mut iter = module.chars();
            let type_code = iter.next().unwrap();
            let label = iter.as_str().trim().to_string();

            match type_code {
                '%' => (
                    label,
                    Module::new(ModuleType::FlipFlop(false), destinations),
                ),
                '&' => {
                    conjunctions.push(label.clone());
                    (
                        label,
                        Module::new(ModuleType::Conjunction(HashMap::new()), destinations),
                    )
                }
                _ => (
                    module.trim().to_string(),
                    Module::new(ModuleType::Broadcast, destinations),
                ),
            }
        })
        .collect::<HashMap<String, Module>>();

    for conjunction in conjunctions.iter() {
        let inputs = modules
            .iter()
            .filter_map(|(k, v)| {
                if v.destination_modules.contains(conjunction) {
                    Some(k.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        if let ModuleType::Conjunction(ref mut state) =
            modules.get_mut(conjunction).unwrap().module_type
        {
            for input in inputs.into_iter() {
                state.insert(input, false);
            }
        }
    }

    modules
}

#[derive(Debug, Clone)]
enum ModuleType {
    FlipFlop(bool),
    Conjunction(HashMap<String, bool>),
    Broadcast,
    Untyped,
}

impl Hash for ModuleType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
        match self {
            ModuleType::FlipFlop(m_state) => {
                m_state.hash(state);
            }

            ModuleType::Conjunction(m_state) => {
                for (k, v) in m_state.into_iter() {
                    k.hash(state);
                    v.hash(state);
                }
            }

            ModuleType::Broadcast => {}

            ModuleType::Untyped => {}
        }
    }
}

#[derive(Debug, Clone)]
struct Module {
    high_pulses: usize,
    low_pulses: usize,
    module_type: ModuleType,
    destination_modules: Vec<String>,
}

impl Module {
    fn new(module_type: ModuleType, destination_modules: Vec<String>) -> Self {
        let mut module_type = module_type;

        match module_type {
            ModuleType::FlipFlop(ref mut state) => {
                *state = false;
            }

            ModuleType::Conjunction(ref mut _state) => {}

            ModuleType::Broadcast => {}

            ModuleType::Untyped => {}
        }

        Self {
            high_pulses: 0,
            low_pulses: 0,
            module_type,
            destination_modules,
        }
    }

    fn process_signal(&mut self, signal: bool, source: &str) -> Option<(bool, Vec<String>)> {
        if signal {
            self.high_pulses += 1;
        } else {
            self.low_pulses += 1;
        }

        match self.module_type {
            ModuleType::FlipFlop(ref mut state) => {
                // dbg!(signal);
                if signal == false {
                    *state = !*state;
                    // dbg!(&state);
                    Some((*state, self.destination_modules.clone()))
                } else {
                    None
                }
            }

            ModuleType::Conjunction(ref mut state) => {
                // dbg!(&state);
                // dbg!(&source);
                *state.get_mut(source).unwrap() = signal;
                if state.values().all(|v| *v) {
                    Some((false, self.destination_modules.clone()))
                } else {
                    Some((true, self.destination_modules.clone()))
                }
            }

            ModuleType::Broadcast => Some((signal, self.destination_modules.clone())),

            ModuleType::Untyped => None,
        }
    }
}

impl Hash for Module {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.module_type.hash(state);
        self.destination_modules.hash(state);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_run_circuit() {
        let input = [
            "broadcaster -> a, b, c",
            "%a -> b",
            "%b -> c",
            "%c -> inv",
            "&inv -> a",
        ];

        let expected = 32;
        let actual = run_circuit(1, &input);
        assert_eq!(actual, expected);

        let expected = 32000000;
        let actual = run_circuit(1000, &input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_run_circuit_example_2() {
        let input = [
            "broadcaster -> a",
            "%a -> inv, con",
            "&inv -> b",
            "%b -> con",
            "&con -> output",
        ];

        let expected = 11687500;
        let actual = run_circuit(1000, &input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_find_cycle_len() {
        let input = [
            "broadcaster -> a",
            "%a -> inv, con",
            "&inv -> b",
            "%b -> con",
            "&con -> output",
        ];

        let modules = parse_modules(&input);

        let expected = 1;
        let actual = find_cycle_len(&"b".to_string(), &"con".to_string(), &modules);
        assert_eq!(actual, expected);
    }
}
