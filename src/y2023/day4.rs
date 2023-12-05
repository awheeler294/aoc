use std::{collections::HashMap, usize};

pub fn solve(input: &[&str]) -> String {
    let part1 = score_cards(input);
    let part2 = count_winning_cards(input);

    format!(" Part1: {} \n Part2: {}", part1, part2)
}

fn score_cards(cards: &[&str]) -> usize {
    cards
        .iter()
        .map(|line| Card::new(line))
        .map(|card| card.score())
        .sum()
}

fn count_winning_cards(cards: &[&str]) -> usize {
    let cards = cards
        .iter()
        .map(|line| Card::new(line))
        .collect::<Vec<Card>>();

    let mut count = 0;

    for card in cards.iter() {
        count += card.winning_card_total(&cards);
    }

    count
}

struct Card {
    id: usize,
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>,
}

impl Card {
    pub fn new(card_info: &str) -> Self {
        let (id, winning_numbers, numbers) = Self::parse_card_info(card_info);

        Self {
            id,
            winning_numbers,
            numbers,
        }
    }

    pub fn score(&self) -> usize {
        self.numbers
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .fold(0, |acc, _| if acc == 0 { 1 } else { acc * 2 })
    }

    pub fn winning_card_total(&self, cards: &Vec<Card>) -> usize {
        let mut card_counts = HashMap::new();
        self.count_winning_cards(cards, &mut card_counts);

        card_counts.iter().fold(0, |acc, (_, count)| acc + count)
    }

    pub fn count_winning_cards(&self, cards: &Vec<Card>, card_counts: &mut HashMap<usize, usize>) {
        card_counts
            .entry(self.id)
            .and_modify(|count| *count += 1)
            .or_insert(1);

        let winning_count = self
            .numbers
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .count();

        for card in cards[self.id..self.id + winning_count].iter() {
            card.count_winning_cards(cards, card_counts);
        }
    }

    fn parse_card_info(card_info: &str) -> (usize, Vec<u32>, Vec<u32>) {
        let (metadata, right) = card_info.split_once(':').unwrap();

        let id = Self::parse_id(metadata);

        let (winning_numbers, numbers) = right.split_once('|').unwrap();

        let winning_numbers = Self::parse_numbers(winning_numbers);
        let numbers = Self::parse_numbers(numbers);

        (id, winning_numbers, numbers)
    }

    fn parse_id(info: &str) -> usize {
        let (_, mut id) = info.split_once(' ').unwrap();

        while let Some(s) = id.strip_prefix(' ') {
            id = s;
        }

        id.parse::<usize>()
            .expect(&format!("Cound not parse `{id}` as digit"))
    }

    fn parse_numbers(numbers: &str) -> Vec<u32> {
        numbers
            .split(' ')
            .filter_map(|n| n.parse::<u32>().ok())
            .collect()
    }
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_score_cards() {
        let input = [
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ];

        let expected = 13;
        let actual = score_cards(&input);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_count_winning_cards() {
        let input = [
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ];

        let expected = 30;
        let actual = count_winning_cards(&input);

        assert_eq!(actual, expected);
    }
}
