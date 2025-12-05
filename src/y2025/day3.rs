pub fn solve(input: &[&str]) -> String {
    let part_1 = solve_part_1(input);
    let part_2 = solve_part_2(input);

    format!(" Part1: {part_1} \n Part2: {part_2}")
}

fn solve_part_1(input: &[&str]) -> u32 {
    let mut total = 0;

    for line in input {
        let battery: Vec<u32> = line
            .chars()
            .map(|n| {
                n.to_digit(10)
                    .unwrap_or_else(|| panic!("malformed number `{}`", n))
            })
            .collect();

        // dbg!(&battery);

        let msi = max_index(&battery[0..battery.len() - 1], 0);
        // dbg!(msi);
        let lsi = max_index(&battery[0..battery.len()], msi + 1);
        // dbg!(lsi);

        total += (battery[msi] * 10) + battery[lsi];
    }

    total
}

fn solve_part_2(input: &[&str]) -> u64 {
    let mut total = 0_u64;

    for line in input {
        let battery: Vec<u32> = line
            .chars()
            .map(|n| {
                n.to_digit(10)
                    .unwrap_or_else(|| panic!("malformed number `{}`", n))
            })
            .collect();

        // dbg!(&battery);

        let mut digits = Vec::with_capacity(12);
        let mut start = 0;
        let mut reserved = 11;

        for _ in 0..12 {
            let end = battery.len() - reserved;
            // dbg!(battery.len());
            // dbg!(reserved);
            // dbg!(end);
            let i = max_index(&battery[0..end], start);
            digits.push(battery[i] as u64);
            start = i + 1;
            reserved = reserved.saturating_sub(1);
        }

        total += digits
            .iter()
            .rev()
            .enumerate()
            .fold(0_u64, |acc, (i, d)| acc + d * 10_u64.pow(i as u32));
    }

    total
}

fn max_index(nums: &[u32], start: usize) -> usize {
    // eprintln!("");
    // dbg!(nums);
    // dbg!(start);
    let mut max = start;
    for i in start + 1..nums.len() {
        if nums[i] > nums[max] {
            max = i;
        }
    }

    // dbg!(max);
    max
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_solve_part_1() {
        #[rustfmt::skip]
      let input = [
         "987654321111111",
         "811111111111119",
         "234234234234278",
         "818181911112111",
      ];

        let expected = 357;

        let actual = solve_part_1(&input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_solve_part_2() {
        #[rustfmt::skip]
      let input = [
         "987654321111111",
         "811111111111119",
         "234234234234278",
         "818181911112111",
      ];

        let expected = 3121910778619;

        let actual = solve_part_2(&input);

        assert_eq!(actual, expected);
    }
}
