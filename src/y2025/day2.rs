pub fn solve(input: &[&str]) -> String {
    let part_1 = solve_part_1(input);
    let part_2 = solve_part_2(input);

    format!(" Part1: {part_1} \n Part2: {part_2}")
}

fn solve_part_1(input: &[&str]) -> u64 {
    // dbg!(input);

    let mut acc = 0;

    for range in input[0].split(',') {
        // dbg!(range);

        let (start, end) = {
            let (start, end) = range
                .split_once('-')
                .unwrap_or_else(|| panic!("malformed range `{}`", range));

            (
                start
                    .parse::<u64>()
                    .unwrap_or_else(|e| panic!("malformed number `{}`: {e}", start)),
                end.parse::<u64>()
                    .unwrap_or_else(|e| panic!("malformed number `{}`: {e}", end)),
            )
        };

        for n in start..=end {
            let s = n.to_string().chars().collect::<Vec<_>>();
            if s.len() % 2 == 0 {
                let mid = s.len() / 2;
                if s[..mid] == s[mid..] {
                    acc += n;
                }
            }
        }
    }

    acc
}

fn solve_part_2(input: &[&str]) -> u64 {
    // dbg!(input);

    let mut acc = 0;

    for range in input[0].split(',') {
        // dbg!(range);

        let (start, end) = {
            let (start, end) = range
                .split_once('-')
                .unwrap_or_else(|| panic!("malformed range `{}`", range));

            (
                start
                    .parse::<u64>()
                    .unwrap_or_else(|e| panic!("malformed number `{}`: {e}", start)),
                end.parse::<u64>()
                    .unwrap_or_else(|e| panic!("malformed number `{}`: {e}", end)),
            )
        };

        for n in start..=end {
            // eprint!("");
            // dbg!(n);
            let s = n.to_string().chars().collect::<Vec<_>>();
            for l in 1..=s.len() / 2 {
                // dbg!(l);
                let mut chunks = s.chunks_exact(l);
                if chunks.remainder().len() == 0 {
                    let first = chunks.next();
                    if chunks.all(|e| Some(e) == first) {
                        // eprint!("invalid!");
                        acc += n;
                        break;
                    }
                }
            }
        }
    }

    acc
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_solve_part_1() {
        let input = [
         "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
      ];

        let expected = 1227775554;

        let actual = solve_part_1(&input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_solve_part_2() {
        let input = [
         "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
      ];

        let expected = 4174379265;

        let actual = solve_part_2(&input);

        assert_eq!(actual, expected);
    }
}
