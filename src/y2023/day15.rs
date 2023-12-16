use regex::Regex;

pub fn solve(input: &[&str]) -> String {
    let part1 = hash_instructions(input);
    let part2 = total_power(input);

    format!(" Part1: {} \n Part2: {}", part1, part2)
}

fn total_power(input: &[&str]) -> usize {
    let mut map = HolidayHashmap::new();
    for command in input.join("").split(',') {
        map.process_command(command);
    }

    map.focusing_power()
}

fn hash_instructions(input: &[&str]) -> usize {
    input.join("").split(',').map(holiday_hash).sum()
}

fn holiday_hash(value: &str) -> usize {
    let mut current = 0;
    for ch in value.chars() {
        current = ((current + ch as usize) * 17) % 256
    }

    current
}

struct HolidayHashmap {
    boxes: Vec<Vec<(String, usize)>>,
    command_re: Regex,
}

impl HolidayHashmap {
    fn new() -> Self {
        Self {
            boxes: vec![vec![]; 256],
            command_re: Regex::new(r"(?P<label>[a-z]+)(?P<command>=\d|-)").unwrap(),
        }
    }

    fn process_command(&mut self, command: &str) {
        let captures = self.command_re.captures(command).unwrap();

        let label = &captures["label"];
        let command = &captures["command"];

        let index = holiday_hash(label);

        let mut chars = command.chars();
        match chars.next() {
            Some('-') => {
                for (i, (lens_label, _focal_length)) in self.boxes[index].iter_mut().enumerate() {
                    if *lens_label == label {
                        self.boxes[index].remove(i);
                        break;
                    }
                }
            }

            Some('=') => {
                let focal_length = chars.next().unwrap().to_digit(10).unwrap() as usize;

                let mut found = false;
                for (i, (lens_label, _focal_length)) in self.boxes[index].iter_mut().enumerate() {
                    if *lens_label == label {
                        self.boxes[index][i].1 = focal_length;
                        found = true;
                        break;
                    }
                }

                if found == false {
                    self.boxes[index].push((String::from(label), focal_length));
                }
            }

            _ => unreachable!(),
        }
    }

    fn focusing_power(&self) -> usize {
        let mut total = 0;

        for (n, b) in self.boxes.iter().enumerate() {
            for (i, (_label, focal_length)) in b.iter().enumerate() {
                total += (n + 1) * (i + 1) * focal_length;
            }
        }

        total
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_holiday_hash() {
        let input = "HASH";

        let expected = 52;
        let actual = holiday_hash(input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_hash_instructions() {
        let input = ["rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"];

        let expected = 1320;
        let actual = hash_instructions(&input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_total_power() {
        let input = ["rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"];

        let expected = 145;
        let actual = total_power(&input);

        assert_eq!(actual, expected);
    }
}
