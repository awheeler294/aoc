pub fn solve(input: &[&str]) -> String {
    let nums = input[0]
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();
    //dbg!(&nums);
    //let lns = look_and_say(&nums, 40);
    //dbg!(&lns);
    let part1 = look_and_say(&nums, 40).len();
    let part2 = look_and_say(&nums, 50).len();

    format!(" Part1: {} \n Part2: {}", part1, part2)
}

fn look_and_say(nums: &[u32], iterations: usize) -> Vec<u32> {
    if nums.len() < 1 {
        return Vec::new();
    }

    let mut nums = Vec::from(nums);
    let mut result = Vec::new();

    for _ in 0..iterations {
        result = Vec::new();
        let mut n = nums[0];
        let mut count = 0;

        for num in nums {
            if num == n {
                count += 1;
            } else {
                result.push(count);
                result.push(n);

                n = num;
                count = 1;
            }
        }

        result.push(count);
        result.push(n);

        nums = result.clone();
    }

    result
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_look_and_say() {
        let nums = vec![1];
        let expected = vec![1, 1];
        let actual = look_and_say(&nums, 1);
        assert_eq!(actual, expected);

        let nums = vec![1];
        let expected = vec![2, 1];
        let actual = look_and_say(&nums, 2);
        assert_eq!(actual, expected);

        let nums = vec![1];
        let expected = vec![1, 2, 1, 1];
        let actual = look_and_say(&nums, 3);
        assert_eq!(actual, expected);

        let nums = vec![1];
        let expected = vec![1, 1, 1, 2, 2, 1];
        let actual = look_and_say(&nums, 4);
        assert_eq!(actual, expected);

        let nums = vec![1];
        let expected = vec![3, 1, 2, 2, 1, 1];
        let actual = look_and_say(&nums, 5);
        assert_eq!(actual, expected);

        let nums = vec![1];
        let expected = vec![1, 3, 1, 1, 2, 2, 2, 1];
        let actual = look_and_say(&nums, 6);
        assert_eq!(actual, expected);

        let nums = vec![1];
        let expected = vec![1, 1, 1, 3, 2, 1, 3, 2, 1, 1];
        let actual = look_and_say(&nums, 7);
        assert_eq!(actual, expected);

        let nums = vec![1];
        let expected = vec![3, 1, 1, 3, 1, 2, 1, 1, 1, 3, 1, 2, 2, 1];
        let actual = look_and_say(&nums, 8);
        assert_eq!(actual, expected);

        let nums = vec![1, 3, 2, 1, 1, 3, 1, 1, 1, 2];
        let expected = vec![1, 1, 1, 3, 1, 2, 2, 1, 1, 3, 3, 1, 1, 2];
        let actual = look_and_say(&nums, 1);
        assert_eq!(actual, expected);

        let nums = vec![1, 3, 2, 1, 1, 3, 1, 1, 1, 2];
        let expected = vec![3, 1, 1, 3, 1, 1, 2, 2, 2, 1, 2, 3, 2, 1, 1, 2];
        let actual = look_and_say(&nums, 2);
        assert_eq!(actual, expected);
    }
}
