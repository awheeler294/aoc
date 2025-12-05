pub fn solve(input: &[&str]) -> String {
    let inventory = create_inventory(input);
    let part1 = part1(&inventory);
    let part2 = part2(&inventory);

    format!(" Part1: {} \n Part2: {}", part1, part2)
}

fn part1(inventory: &[u32]) -> u32 {
    *inventory.last().unwrap_or(&0)
}

fn part2(inventory: &[u32]) -> u32 {
    inventory.iter().rev().take(3).sum::<u32>()
}

fn create_inventory(lines: &[&str]) -> Vec<u32> {
    let mut inventory = vec![0];
    for line in lines {
        if line.is_empty() {
            inventory.push(0);
        } else {
            let calories = line
                .parse::<u32>()
                .unwrap_or_else(|e| panic!("unable to parse '{}' as i32: {}", line, e));
            *inventory.last_mut().unwrap() += calories;
        }
    }

    inventory.sort();

    inventory
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part1() {
        #[rustfmt::skip]
        let input = vec![
            "1000",
            "2000",
            "3000",
            "",
            "4000",
            "",
            "5000",
            "6000",
            "",
            "7000",
            "8000",
            "9000",
            "",
            "10000",
        ];
        let inventory = create_inventory(&input);
        let actual = part1(&inventory);
        let expected = 24000;
        assert_eq!(
            actual, expected,
            "\n  Got {actual} when expecting {expected}"
        );
    }

    #[test]
    fn test_part2() {
        #[rustfmt::skip]
        let input = vec![
            "1000",
            "2000",
            "3000",
            "",
            "4000",
            "",
            "5000",
            "6000",
            "",
            "7000",
            "8000",
            "9000",
            "",
            "10000",
        ];
        let inventory = create_inventory(&input);
        let actual = part2(&inventory);
        let expected = 45000;
        assert_eq!(
            actual, expected,
            "\n  Got {actual} when expecting {expected}"
        );
    }
}
