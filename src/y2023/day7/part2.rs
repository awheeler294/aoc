use std::{collections::HashMap, convert::TryFrom};

use anyhow::anyhow;

pub fn score_game(game_data: &[&str]) -> usize {
    let mut hands = game_data
        .iter()
        .map(|line| Hand::try_from(*line))
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    hands.sort();

    // dbg!(&hands);

    let mut score = 0;

    for (i, hand) in hands.iter().enumerate() {
        score += hand.bid * (i + 1);
    }

    score
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum PlayingCard {
    Joker,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl TryFrom<char> for PlayingCard {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Self::Ace),
            'K' => Ok(Self::King),
            'Q' => Ok(Self::Queen),
            'T' => Ok(Self::Ten),
            '9' => Ok(Self::Nine),
            '8' => Ok(Self::Eight),
            '7' => Ok(Self::Seven),
            '6' => Ok(Self::Six),
            '5' => Ok(Self::Five),
            '4' => Ok(Self::Four),
            '3' => Ok(Self::Three),
            '2' => Ok(Self::Two),
            '1' => Ok(Self::One),
            'J' => Ok(Self::Joker),

            _ => Err(anyhow!("Could not parse `{value}` as PlayingCard")),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

impl HandType {
    pub fn new(cards: &[PlayingCard; 5]) -> Self {
        let mut joker_count = 0;

        let card_counts = {
            let mut card_counts = HashMap::new();
            for card in cards.iter() {
                if *card == PlayingCard::Joker {
                    joker_count += 1;
                } else {
                    card_counts
                        .entry(card)
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                }
            }

            card_counts
        };

        // dbg!(&cards);
        // dbg!(&card_counts);

        let mut three_of_a_kind = false;
        let mut one_pair = false;

        // dbg!(&joker_count);
        for (_card, count) in card_counts.iter() {
            match *count {
                5 => {
                    return HandType::FiveOfKind;
                }

                4 => {
                    if joker_count >= 1 {
                        return HandType::FiveOfKind;
                    }

                    return HandType::FourOfKind;
                }

                3 => {
                    if joker_count >= 2 {
                        return HandType::FiveOfKind;
                    }

                    if joker_count == 1 {
                        return HandType::FourOfKind;
                    }

                    if one_pair {
                        return HandType::FullHouse;
                    }

                    three_of_a_kind = true;
                }

                2 => {
                    if joker_count >= 3 {
                        return HandType::FiveOfKind;
                    }

                    if joker_count == 2 {
                        return HandType::FourOfKind;
                    }

                    if three_of_a_kind {
                        return HandType::FullHouse;
                    }

                    if joker_count == 1 {
                        three_of_a_kind = true;
                        continue;
                    }

                    if one_pair {
                        return HandType::TwoPair;
                    }

                    one_pair = true;
                }

                _ => {}
            }
        }

        if three_of_a_kind {
            return HandType::ThreeOfKind;
        } else if one_pair {
            return HandType::OnePair;
        }

        // at this point we have some number of jokers and no other cards match
        match joker_count {
            5 => {
                return HandType::FiveOfKind;
            }
            4 => {
                return HandType::FiveOfKind;
            }
            3 => {
                return HandType::FourOfKind;
            }
            2 => {
                return HandType::ThreeOfKind;
            }
            1 => {
                return HandType::OnePair;
            }
            _ => {}
        }

        Self::HighCard
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord)]
struct Hand {
    cards: [PlayingCard; 5],
    bid: usize,
    hand_type: HandType,
}

impl Hand {
    pub fn new(cards: [PlayingCard; 5], bid: usize) -> Self {
        let hand_type = HandType::new(&cards);

        Self {
            cards,
            bid,
            hand_type,
        }
    }

    pub fn parse_cards(input: &str) -> anyhow::Result<[PlayingCard; 5]> {
        if input.chars().count() != 5 {
            return Err(anyhow!(
                "parse_cards: input must be of length 5. input: {input}"
            ));
        }

        let mut cards = [PlayingCard::One; 5];
        for (i, ch) in input.chars().enumerate() {
            cards[i] = PlayingCard::try_from(ch)?;
        }

        Ok(cards)
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl PartialOrd for Hand {
    fn lt(&self, other: &Self) -> bool {
        if self.hand_type == other.hand_type {
            for (this_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
                if this_card != other_card {
                    return this_card < other_card;
                }
            }
        }

        self.hand_type < other.hand_type
    }

    fn le(&self, other: &Self) -> bool {
        if self.hand_type == other.hand_type {
            for (this_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
                if this_card != other_card {
                    return this_card < other_card;
                }
            }
        }

        self.hand_type <= other.hand_type
    }

    fn gt(&self, other: &Self) -> bool {
        // dbg!(self);
        // dbg!(other);
        // dbg!(self.hand_type == other.hand_type);
        if self.hand_type == other.hand_type {
            for (this_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
                if this_card != other_card {
                    return this_card > other_card;
                }
            }
        }

        // dbg!(self.hand_type > other.hand_type);
        self.hand_type > other.hand_type
    }

    fn ge(&self, other: &Self) -> bool {
        if self.hand_type == other.hand_type {
            for (this_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
                if this_card != other_card {
                    return this_card > other_card;
                }
            }
        }

        self.hand_type >= other.hand_type
    }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.hand_type == other.hand_type {
            for (this_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
                if this_card != other_card {
                    return this_card.partial_cmp(other_card);
                }
            }
        }

        self.hand_type.partial_cmp(&other.hand_type)
    }
}

impl TryFrom<&str> for Hand {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (cards, bid) = value
            .split_once(' ')
            .ok_or_else(|| anyhow!("Could not split `{value}` at space character"))?;
        let cards = Self::parse_cards(&cards)?;
        let bid = bid.trim().parse()?;

        Ok(Self::new(cards, bid))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_hand_type() {
        let cards = Hand::parse_cards("AAAAA").unwrap();
        let expected = HandType::FiveOfKind;
        let actual = HandType::new(&cards);
        assert_eq!(actual, expected);

        let cards = Hand::parse_cards("AA8AA").unwrap();
        let expected = HandType::FourOfKind;
        let actual = HandType::new(&cards);
        assert_eq!(actual, expected);

        let cards = Hand::parse_cards("23332").unwrap();
        let expected = HandType::FullHouse;
        let actual = HandType::new(&cards);
        assert_eq!(actual, expected);

        let cards = Hand::parse_cards("TTT98").unwrap();
        let expected = HandType::ThreeOfKind;
        let actual = HandType::new(&cards);
        assert_eq!(actual, expected);

        let cards = Hand::parse_cards("23432").unwrap();
        let expected = HandType::TwoPair;
        let actual = HandType::new(&cards);
        assert_eq!(actual, expected);

        let cards = Hand::parse_cards("A23A4").unwrap();
        let expected = HandType::OnePair;
        let actual = HandType::new(&cards);
        assert_eq!(actual, expected);

        let cards = Hand::parse_cards("23456").unwrap();
        let expected = HandType::HighCard;
        let actual = HandType::new(&cards);
        assert_eq!(actual, expected);

        let cards = Hand::parse_cards("JJJJJ").unwrap();
        let expected = HandType::FiveOfKind;
        let actual = HandType::new(&cards);
        assert_eq!(actual, expected);

        let cards = Hand::parse_cards("JJ7T9").unwrap();
        let expected = HandType::ThreeOfKind;
        let actual = HandType::new(&cards);
        assert_eq!(actual, expected);

        let cards = Hand::parse_cards("J28K9").unwrap();
        let expected = HandType::OnePair;
        let actual = HandType::new(&cards);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_hand_ord() {
        let hands = vec![
            Hand::try_from("32T3K 765").unwrap(), // 0
            Hand::try_from("T55J5 684").unwrap(), // 1
            Hand::try_from("KK677 28 ").unwrap(), // 2
            Hand::try_from("KTJJT 220").unwrap(), // 3
            Hand::try_from("QQQJA 483").unwrap(), // 4
        ];

        assert_eq!(hands[0], hands[0]);
        assert_ne!(hands[0], hands[1]);

        assert!(hands[3] > hands[0]);
        assert!(hands[0] < hands[3]);

        assert!(hands[2] < hands[3]);
        assert!(hands[3] > hands[2]);

        assert!(hands[1] < hands[4]);
        assert!(hands[4] > hands[1]);

        #[rustfmt::skip]
        let expected = vec![
            hands[0].clone(),
            hands[2].clone(),
            hands[1].clone(),
            hands[4].clone(),
            hands[3].clone(),

        ];

        let actual = {
            let mut hands = hands;
            hands.sort();
            hands
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_hands() {
        // test data courtesy of https://www.reddit.com/r/adventofcode/comments/18cr4xr/2023_day_7_better_example_input_not_a_spoiler/
        let input = [
            "2345A 1", "Q2KJJ 13", "Q2Q2Q 19", "T3T3J 17", "T3Q33 11", "2345J 3", "J345A 2",
            "32T3K 5", "T55J5 29", "KK677 7", "KTJJT 34", "QQQJA 31", "JJJJJ 37", "JAAAA 43",
            "AAAAJ 59", "AAAAA 61", "2AAAA 23", "2JJJJ 53", "JJJJ2 41",
        ];

        let expected = vec![
            Hand::try_from("2345A 1 ").unwrap(),
            Hand::try_from("J345A 2 ").unwrap(),
            Hand::try_from("2345J 3 ").unwrap(),
            Hand::try_from("32T3K 5 ").unwrap(),
            Hand::try_from("KK677 7 ").unwrap(),
            Hand::try_from("T3Q33 11").unwrap(),
            Hand::try_from("Q2KJJ 13").unwrap(),
            Hand::try_from("T3T3J 17").unwrap(),
            Hand::try_from("Q2Q2Q 19").unwrap(),
            Hand::try_from("2AAAA 23").unwrap(),
            Hand::try_from("T55J5 29").unwrap(),
            Hand::try_from("QQQJA 31").unwrap(),
            Hand::try_from("KTJJT 34").unwrap(),
            Hand::try_from("JJJJJ 37").unwrap(),
            Hand::try_from("JJJJ2 41").unwrap(),
            Hand::try_from("JAAAA 43").unwrap(),
            Hand::try_from("2JJJJ 53").unwrap(),
            Hand::try_from("AAAAJ 59").unwrap(),
            Hand::try_from("AAAAA 61").unwrap(),
        ];

        let actual = {
            let mut hands = input
                .iter()
                .map(|line| Hand::try_from(*line))
                .collect::<Result<Vec<_>, _>>()
                .unwrap();

            hands.sort();

            hands
        };

        for (actual, expected) in actual.iter().zip(expected.iter()) {
            assert_eq!(actual, expected);
        }

        // assert_eq!(actual, expected, "{actual:#?} {expected:#?}");
    }

    #[test]
    fn test_score_game() {
        let input = [
            "32T3K 765",
            "T55J5 684",
            "KK677 28",
            "KTJJT 220",
            "QQQJA 483",
        ];

        let expected = 5905;
        let actual = score_game(&input);

        assert_eq!(expected, actual);
    }
}
