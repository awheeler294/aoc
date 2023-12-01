use md5;

pub fn solve(input: &[&str]) -> String {
    let part1 = mine(input[0], 5);
    let part2 = mine(input[0], 6);

    format!(" Part1: {} \n Part2: {}", part1, part2)
}

fn mine(key: &str, difficulty: usize) -> u32 {
    let mut n = 0;
    loop {
        let digest = md5::compute(format!("{}{}", key, n).as_bytes());
        let hex = format!("{:x}", digest);
        let mut is_success = true;
        for i in 0..difficulty {
            if hex.chars().nth(i).unwrap() != '0' {
                is_success = false;
                break;
            }
        }
        if is_success {
            //dbg!(&hex);
            return n;
        }
        n += 1;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_md5() {
        let digest = md5::compute(b"abcdefghijklmnopqrstuvwxyz");
        assert_eq!(format!("{:x}", digest), "c3fcd3d76192e4007dfb496cca67e13b");
    }

    #[test]
    #[ignore] // This test is expensive
    fn test_mine_full() {
        let key = "abcdef";
        let expected = 609043;
        let actual = mine(key, 5);
        assert_eq!(actual, expected);

        let key = "pqrstuv";
        let expected = 1048970;
        let actual = mine(key, 5);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_mine_cheap() {
        let key = "abcdef";
        let expected = 31;
        let actual = mine(key, 1);
        assert_eq!(actual, expected);

        let key = "pqrstuv";
        let expected = 53;
        let actual = mine(key, 1);
        assert_eq!(actual, expected);
    }
}
