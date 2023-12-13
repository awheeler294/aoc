use std::cmp::Ordering;

pub fn solve(input: &[&str]) -> String {
    let part1 = find_out_of_order_packets(input);
    let part2 = sort_packets(input);

    format!(" Part1: {} \n Part2: {}", part1, part2)
}

fn sort_packets(input: &[&str]) -> usize {
    let mut packets = parse_input(input);

    let div_1 = PacketData::List(vec![PacketData::List(vec![PacketData::Value(2)])]);
    let div_2 = PacketData::List(vec![PacketData::List(vec![PacketData::Value(6)])]);
    packets.append(&mut vec![div_1.clone(), div_2.clone()]);

    packets.sort_by(compare_packets);

    // dbg!(&packets);

    let mut acc = 1;
    for (i, packet) in packets.iter().enumerate() {
        if *packet == div_1 || *packet == div_2 {
            acc *= i + 1;
        }
    }

    acc
}

fn find_out_of_order_packets(input: &[&str]) -> usize {
    let packets = parse_input(input);

    let mut acc = 0;

    for (i, pair) in packets.chunks_exact(2).enumerate() {
        let a = pair.get(0).unwrap();
        let b = pair.get(1).unwrap();

        if compare_packets(a, b) == Ordering::Less {
            acc += i + 1;
        }
    }

    acc
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    OpenBracket,
    CloseBracket,
    Value(u32),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum PacketData {
    List(Vec<PacketData>),
    Value(u32),
}

#[derive(Debug, PartialEq, Eq)]
struct TokenStream {
    stream: Vec<Token>,
    idx: usize,
}

impl TokenStream {
    fn new(stream: Vec<Token>) -> TokenStream {
        Self { stream, idx: 0 }
    }
}

impl Iterator for TokenStream {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.idx += 1;
        self.stream.get(self.idx - 1).cloned()
    }
}

fn parse_input(input: &[&str]) -> Vec<PacketData> {
    let tokens = tokenize_input(input);
    let mut packets = Vec::new();

    for mut token_stream in tokens {
        packets.extend(parse_packet(&mut token_stream));
    }

    packets
}

fn parse_packet(tokens: &mut TokenStream) -> Vec<PacketData> {
    let mut data = Vec::new();

    while let Some(token) = tokens.next() {
        match token {
            Token::OpenBracket => data.push(PacketData::List(parse_packet(tokens))),
            Token::CloseBracket => break,
            Token::Value(val) => data.push(PacketData::Value(val)),
        }
    }

    data
}

fn tokenize_input(input: &[&str]) -> Vec<TokenStream> {
    let mut results = Vec::new();
    let mut acc = None;

    for line in input {
        if !line.is_empty() {
            let mut tokens = Vec::new();

            for ch in line.chars() {
                match ch {
                    '[' => tokens.push(Token::OpenBracket),
                    ']' => {
                        if let Some(val) = acc {
                            tokens.push(Token::Value(val));
                            acc = None;
                        }
                        tokens.push(Token::CloseBracket);
                    }
                    ',' => {
                        if let Some(val) = acc {
                            tokens.push(Token::Value(val));
                            acc = None;
                        }
                    }
                    _ => {
                        if let Some(digit) = ch.to_digit(10) {
                            acc = match acc {
                                Some(val) => Some(val * 10),
                                None => Some(0),
                            };

                            acc = Some(acc.unwrap() + digit);
                        }
                    }
                }
            }

            results.push(TokenStream::new(tokens));
        }
    }

    results
}

fn compare_packets(a: &PacketData, b: &PacketData) -> Ordering {
    match (a, b) {
        (&PacketData::Value(a_val), &PacketData::Value(b_val)) => a_val.cmp(&b_val),
        (&PacketData::Value(a_val), b_list) => {
            compare_packets(&PacketData::List(vec![PacketData::Value(a_val)]), b_list)
        }
        (a_list, &PacketData::Value(b_val)) => {
            compare_packets(a_list, &PacketData::List(vec![PacketData::Value(b_val)]))
        }
        (&PacketData::List(ref a_list), &PacketData::List(ref b_list)) => {
            let mut a_list_iter = a_list.iter();
            let mut b_list_iter = b_list.iter();

            loop {
                let a_item = a_list_iter.next();
                let b_item = b_list_iter.next();

                match (a_item, b_item) {
                    (None, Some(_)) => return Ordering::Less,
                    (Some(_), None) => return Ordering::Greater,
                    (None, None) => return Ordering::Equal,
                    (Some(left), Some(right)) => match compare_packets(left, right) {
                        Ordering::Equal => continue,
                        Ordering::Less => return Ordering::Less,
                        Ordering::Greater => return Ordering::Greater,
                    },
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_find_out_of_order_packets() {
        #[rustfmt::skip]
        let input = [
            "[1,1,3,1,1]",
            "[1,1,5,1,1]",
            "",
            "[[1],[2,3,4]]",
            "[[1],4]",
            "",
            "[9]",
            "[[8,7,6]]",
            "",
            "[[4,4],4,4]",
            "[[4,4],4,4,4]",
            "",
            "[7,7,7,7]",
            "[7,7,7]",
            "",
            "[]",
            "[3]",
            "",
            "[[[]]]",
            "[[]]",
            "",
            "[1,[2,[3,[4,[5,6,7]]]],8,9]",
            "[1,[2,[3,[4,[5,6,0]]]],8,9]",
        ];

        let actual = find_out_of_order_packets(&input);
        let expected = 13;

        assert_eq!(
            actual, expected,
            "\n Got {:#?} when expecting {:#?} from calling find_out_of_order_packets on {:#?}",
            actual, expected, input
        );
    }

    #[test]
    fn test_sort_packets() {
        #[rustfmt::skip]
        let input = [
            "[1,1,3,1,1]",
            "[1,1,5,1,1]",
            "",
            "[[1],[2,3,4]]",
            "[[1],4]",
            "",
            "[9]",
            "[[8,7,6]]",
            "",
            "[[4,4],4,4]",
            "[[4,4],4,4,4]",
            "",
            "[7,7,7,7]",
            "[7,7,7]",
            "",
            "[]",
            "[3]",
            "",
            "[[[]]]",
            "[[]]",
            "",
            "[1,[2,[3,[4,[5,6,7]]]],8,9]",
            "[1,[2,[3,[4,[5,6,0]]]],8,9]",
        ];

        let actual = sort_packets(&input);
        let expected = 140;

        assert_eq!(
            actual, expected,
            "\n Got {:#?} when expecting {:#?} from calling find_out_of_order_packets on {:#?}",
            actual, expected, input
        );
    }

    #[test]
    fn test_tokenize_input() {
        #[rustfmt::skip]
        let input = [
            "[1,1,3,1,1]",
            "[1,1,5,1,1]",
            "",
            "[[1],[2,3,4]]",
            "[[1],4]",
        ];

        let expected = vec![
            TokenStream::new(vec![
                Token::OpenBracket,
                Token::Value(1),
                Token::Value(1),
                Token::Value(3),
                Token::Value(1),
                Token::Value(1),
                Token::CloseBracket,
            ]),
            TokenStream::new(vec![
                Token::OpenBracket,
                Token::Value(1),
                Token::Value(1),
                Token::Value(5),
                Token::Value(1),
                Token::Value(1),
                Token::CloseBracket,
            ]),
            TokenStream::new(vec![
                Token::OpenBracket,
                Token::OpenBracket,
                Token::Value(1),
                Token::CloseBracket,
                Token::OpenBracket,
                Token::Value(2),
                Token::Value(3),
                Token::Value(4),
                Token::CloseBracket,
                Token::CloseBracket,
            ]),
            TokenStream::new(vec![
                Token::OpenBracket,
                Token::OpenBracket,
                Token::Value(1),
                Token::CloseBracket,
                Token::Value(4),
                Token::CloseBracket,
            ]),
        ];

        let actual = tokenize_input(&input);

        assert_eq!(
            actual, expected,
            "\n Got {:#?} when expecting {:#?} from calling parse_input on {:#?}",
            actual, expected, input
        );
    }

    #[test]
    fn test_parse_input() {
        #[rustfmt::skip]
        let input = [
            "[1,1,3,1,1]",
            "[1,1,5,1,1]",
            "",
            "[[1],[2,3,4]]",
            "[[1],4]",
        ];

        let expected = [
            PacketData::List(vec![
                PacketData::Value(1),
                PacketData::Value(1),
                PacketData::Value(3),
                PacketData::Value(1),
                PacketData::Value(1),
            ]),
            PacketData::List(vec![
                PacketData::Value(1),
                PacketData::Value(1),
                PacketData::Value(5),
                PacketData::Value(1),
                PacketData::Value(1),
            ]),
            PacketData::List(vec![
                PacketData::List(vec![PacketData::Value(1)]),
                PacketData::List(vec![
                    PacketData::Value(2),
                    PacketData::Value(3),
                    PacketData::Value(4),
                ]),
            ]),
            PacketData::List(vec![
                PacketData::List(vec![PacketData::Value(1)]),
                PacketData::Value(4),
            ]),
        ];
        let actual = parse_input(&input);

        assert_eq!(
            actual, expected,
            "\n Got {:#?} when expecting {:#?} from calling parse_input on {:#?}",
            expected, actual, input
        );
    }

    #[test]
    fn test_comparison() {
        // macro_rules! compare {
        //     ($first:expr, $second:expr, $cmp:expr) => {
        //         let first = parse_packet($first);
        //         let second = parse_packet($second);
        //         assert_eq!(compare_packets(&first, &second), $cmp);
        //     };
        // }

        // compare!("[]", "[]", Ordering::Equal);
        // compare!("[]", "[1]", Ordering::Less);
        // compare!("[1]", "[]", Ordering::Greater);
        // compare!("[1]", "[[1]]", Ordering::Equal);
        // compare!("[1, [2], 3]", "[1, 2, 3]", Ordering::Equal);

        let cases = [
            (vec!["[]"], vec!["[]"], Ordering::Equal),
            (vec!["[]"], vec!["[1]"], Ordering::Less),
            (vec!["[1]"], vec!["[]"], Ordering::Greater),
            (vec!["[1]"], vec!["[[1]]"], Ordering::Equal),
            (vec!["[1, [2], 3]"], vec!["[1, 2, 3]"], Ordering::Equal),
        ];

        for (a, b, expected) in cases {
            let packet_a = parse_packet(&mut tokenize_input(&a)[0]);
            let packet_b = parse_packet(&mut tokenize_input(&b)[0]);
            let actual = compare_packets(&packet_a[0], &packet_b[0]);

            assert_eq!(
                actual, expected,
                "\n Got {:#?} when expecting {:#?} from calling compare_packets on {:#?}, {:#?}",
                expected, actual, packet_a, packet_b
            );
        }
    }
}
