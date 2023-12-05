use std::fmt;

pub fn solve(input: &[&str]) -> String {
    let part1 = monkey_business(&mut parse_monkeys(input));
    let part2 = monkey_business_2(&mut parse_monkeys(input));

    format!(" Part1: {} \n Part2: {}", part1, part2)
}

struct Monkey {
    items: Vec<u64>,
    operator: char,
    operand: Option<u64>,
    test_divisor: u64,
    if_true: usize,
    if_false: usize,
    inspected_count: usize,
}

impl Monkey {
    fn new(
        items: &[u64],
        operator: char,
        operand: Option<u64>,
        test_divisor: u64,
        if_true: usize,
        if_false: usize,
    ) -> Self {
        Self {
            items: Vec::from(items),
            operator,
            operand,
            test_divisor,
            if_true,
            if_false,
            inspected_count: 0,
        }
    }

    // fn new_empty() -> Self {
    //     Self {
    //         items: Vec::new(),
    //         operation: Box::new(|x| x),
    //         test: Box::new(|_| 0),
    //         inspected_count: 0,
    //     }
    // }

    fn get_throws(&mut self, worry_relief: u64, lcm: u64) -> Vec<(usize, u64)> {
        let mut to_throw = vec![];

        for item in self.items.drain(..) {
            let worry_level = match self.operator {
                '*' => item * self.operand.unwrap_or(item),
                '+' => item + self.operand.unwrap(),
                _ => panic!("Unknown operator `{}`", self.operator),
            };

            let worry_level = (worry_level / worry_relief) % lcm;
            let destination = {
                if worry_level % self.test_divisor == 0 {
                    self.if_true
                } else {
                    self.if_false
                }
            };

            to_throw.push((destination, worry_level));
            self.inspected_count += 1;
        }

        to_throw
    }
}

impl fmt::Debug for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Monkey: {{ ")?;

        if f.alternate() {
            let mut pretty_format = String::new();
            pretty_format.push_str("\n   ");

            write!(
                f,
                "{pretty_format}inspected_count: {:#?}",
                self.inspected_count
            )?;
            write!(f, "{pretty_format}items: {:#?}", self.items)?;

            pretty_format.truncate(pretty_format.len() - 3);

            writeln!(f)?;
        } else {
            write!(f, "items: {:?} ", self.items)?;
            write!(f, "inspected_count: {:?} ", self.inspected_count)?;
        }

        write!(f, "}}")
    }
}

fn monkey_business(monkeys: &mut [Monkey]) -> usize {
    do_monkey_business(monkeys, 20, 3)
}

fn monkey_business_2(monkeys: &mut [Monkey]) -> usize {
    do_monkey_business(monkeys, 10_000, 1)
}

fn do_monkey_business(monkeys: &mut [Monkey], rounds: u64, worry_relief: u64) -> usize {
    let lcm = monkeys.iter().fold(1, |lcm, monkey| {
        least_common_multiple(lcm, monkey.test_divisor)
    });
    for _round in 0..rounds {
        for i in 0..monkeys.len() {
            let monkey = monkeys.get_mut(i).unwrap();

            let to_throw = monkey.get_throws(worry_relief, lcm);

            for (destination, item) in to_throw {
                monkeys.get_mut(destination).unwrap().items.push(item);
            }
        }
    }

    monkeys.sort_by(|a, b| a.inspected_count.partial_cmp(&b.inspected_count).unwrap());
    //
    // dbg!(&monkeys);
    //
    // for m in monkeys.iter().rev().take(2).map(|m| m.inspected_count) {
    //     dbg!(m);
    // }
    monkeys
        .iter()
        .rev()
        .take(2)
        .map(|m| m.inspected_count)
        .product()
}

fn parse_monkeys(input: &[&str]) -> Vec<Monkey> {
    let mut monkeys = Vec::new();

    let mut lines = input.iter();
    while let Some(line) = lines.next() {
        if line.starts_with("Monkey") {
            let mut items = Vec::new();

            // parse starting items line
            let line = lines.next().unwrap();

            let (_label, starting_items) = line.split_once(':').unwrap();
            for value in starting_items.split(',') {
                items.push(value.trim().parse::<u64>().unwrap());
            }

            // parse operation line
            let line = lines.next().unwrap();

            let (_label, operation) = line.split_once(':').unwrap();
            let mut operation = operation.split(' ').skip(4);

            let operator = operation.next().unwrap().trim().chars().next().unwrap();
            let operand = {
                let operand = operation.next().unwrap().trim();
                if operand.starts_with("old") {
                    None
                } else {
                    Some(operand.parse::<u64>().unwrap_or_else(|err| {
                        panic!(
                            "Could not parse `{:#?}` as u64 from line `{:#?}` Error: {}",
                            operand, line, err
                        )
                    }))
                }
            };

            // parse test lines
            let line = lines.next().unwrap();
            let divisible_by = line
                .trim()
                .split(' ')
                .last()
                .unwrap()
                .parse::<u64>()
                .unwrap();
            let line = lines.next().unwrap();
            let if_true = line
                .trim()
                .split(' ')
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let line = lines.next().unwrap();
            let if_false = line
                .trim()
                .split(' ')
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap();

            monkeys.push(Monkey::new(
                &items,
                operator,
                operand,
                divisible_by,
                if_true,
                if_false,
            ));
        }
    }

    monkeys
}

fn least_common_multiple(a: u64, b: u64) -> u64 {
    let (mut x, mut y) = (a.min(b), a.max(b));
    let mut remainder = x % y;
    while remainder != 0 {
        x = y;
        y = remainder;
        remainder = x % y;
    }

    (a * b) / y
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_lcm() {
        assert_eq!(least_common_multiple(15, 20), 60);
    }

    #[test]
    fn test_monkey_business() {
        let mut monkeys = [
            Monkey::new(&[79, 98], '*', Some(19), 23, 2, 3),
            Monkey::new(&[54, 65, 75, 74], '+', Some(6), 19, 2, 0),
            Monkey::new(&[79, 60, 97], '*', None, 13, 1, 3),
            Monkey::new(&[74], '+', Some(3), 17, 0, 1),
        ];

        let actual = monkey_business(&mut monkeys);
        let expected = 10_605;
        assert_eq!(
            actual, expected,
            "\n Got {actual} when expecting {expected} from calling monkey_business"
        );
    }

    #[test]
    fn test_parse_monkeys() {
        let input = [
            "Monkey 0:",
            "  Starting items: 79, 98",
            "  Operation: new = old * 19",
            "  Test: divisible by 23",
            "    If true: throw to monkey 2",
            "    If false: throw to monkey 3",
            "",
            "Monkey 1:",
            "  Starting items: 54, 65, 75, 74",
            "  Operation: new = old + 6",
            "  Test: divisible by 19",
            "    If true: throw to monkey 2",
            "    If false: throw to monkey 0",
            "",
            "Monkey 2:",
            "  Starting items: 79, 60, 97",
            "  Operation: new = old * old",
            "  Test: divisible by 13",
            "    If true: throw to monkey 1",
            "    If false: throw to monkey 3",
            "",
            "Monkey 3:",
            "  Starting items: 74",
            "  Operation: new = old + 3",
            "  Test: divisible by 17",
            "    If true: throw to monkey 0",
            "    If false: throw to monkey 1",
        ];

        let mut monkeys = parse_monkeys(&input);

        let actual = monkey_business(&mut monkeys);
        let expected = 10_605;

        assert_eq!(actual, expected, "\n Got {actual} when expecting {expected} from calling monkey_business on parsed monkeys");
    }

    #[test]
    fn test_monkey_business_2() {
        let input = [
            "Monkey 0:",
            "  Starting items: 79, 98",
            "  Operation: new = old * 19",
            "  Test: divisible by 23",
            "    If true: throw to monkey 2",
            "    If false: throw to monkey 3",
            "",
            "Monkey 1:",
            "  Starting items: 54, 65, 75, 74",
            "  Operation: new = old + 6",
            "  Test: divisible by 19",
            "    If true: throw to monkey 2",
            "    If false: throw to monkey 0",
            "",
            "Monkey 2:",
            "  Starting items: 79, 60, 97",
            "  Operation: new = old * old",
            "  Test: divisible by 13",
            "    If true: throw to monkey 1",
            "    If false: throw to monkey 3",
            "",
            "Monkey 3:",
            "  Starting items: 74",
            "  Operation: new = old + 3",
            "  Test: divisible by 17",
            "    If true: throw to monkey 0",
            "    If false: throw to monkey 1",
        ];

        let mut monkeys = parse_monkeys(&input);

        let actual = monkey_business_2(&mut monkeys);
        let expected = 2_713_310_158;

        assert_eq!(actual, expected, "\n Got {actual} when expecting {expected} from calling monkey_business_2 on parsed monkeys");
    }
}
