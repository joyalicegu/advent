use crate::Solution;
use itertools::Itertools;

pub struct Day07;

#[derive(Copy, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum Label {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Eq, Ord, PartialEq, PartialOrd)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Eq, Ord, PartialEq, PartialOrd)]
pub struct Hand {
    hand_type: HandType,
    cards: (Label, Label, Label, Label, Label),
    bid: usize,
}

impl Day07 {
    fn char_to_label(c: char) -> Label {
        match c {
            'A' => Label::Ace,
            'K' => Label::King,
            'Q' => Label::Queen,
            'J' => Label::Jack,
            'T' => Label::Ten,
            '9' => Label::Nine,
            '8' => Label::Eight,
            '7' => Label::Seven,
            '6' => Label::Six,
            '5' => Label::Five,
            '4' => Label::Four,
            '3' => Label::Three,
            '2' => Label::Two,
            _ => panic!("Day 7: Invalid card label"),
        }
    }

    fn count_cards(cards: &Vec<Label>) -> Vec<usize> {
        cards
            .iter()
            .sorted()
            .group_by(|&card| *card)
            .into_iter()
            .map(|(_, group)| group.count())
            .sorted()
            .rev()
            .collect::<Vec<usize>>()
    }

    fn count_cards_with_joker(cards: &Vec<Label>) -> Vec<usize> {
        // count jokers and other cards
        let joker_count = cards.iter().filter(|&c| *c == Label::Joker).count();
        let mut counts = Self::count_cards(
            &cards
                .clone()
                .into_iter()
                .filter(|&card| card != Label::Joker)
                .collect(),
        );
        // apportion jokers appropriately
        if counts.len() == 0 {
            counts.push(joker_count);
        } else {
            counts[0] += joker_count;
        }
        counts
    }

    fn counts_to_hand_type(counts: &Vec<usize>) -> HandType {
        // where counts is sorted in descending order
        match counts[0] {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => match counts[1] {
                2 => HandType::FullHouse,
                _ => HandType::ThreeOfAKind,
            },
            2 => match counts[1] {
                2 => HandType::TwoPair,
                _ => HandType::OnePair,
            },
            _ => HandType::HighCard,
        }
    }

    fn parse_hand(line: &str) -> (Hand, Hand) {
        let (cards_str, bid_str) = line.split_once(" ").unwrap();
        let bid = bid_str.parse::<usize>().unwrap();
        let cards: Vec<Label> = cards_str.chars().map(Self::char_to_label).collect();
        let hand_type = {
            let counts = Self::count_cards(&cards);
            Self::counts_to_hand_type(&counts)
        };
        let cards_with_joker: Vec<Label> = cards
            .iter()
            .map(|&c| match c {
                Label::Jack => Label::Joker,
                _ => c,
            })
            .collect();
        let hand_type_with_joker = {
            let counts = Self::count_cards_with_joker(&cards_with_joker);
            Self::counts_to_hand_type(&counts)
        };
        (
            Hand {
                hand_type: hand_type,
                cards: cards.into_iter().collect_tuple().unwrap(),
                bid: bid,
            },
            Hand {
                hand_type: hand_type_with_joker,
                cards: cards_with_joker.into_iter().collect_tuple().unwrap(),
                bid: bid,
            },
        )
    }

    fn total_winnings(hands: &Vec<Hand>) -> usize {
        // assumes hands is sorted with ascending rank
        hands
            .iter()
            .enumerate()
            .map(|(r, h)| (r + 1) * h.bid)
            .sum::<usize>()
    }
}

impl Solution for Day07 {
    type ParsedInput = (Vec<Hand>, Vec<Hand>);

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        input_lines.lines().map(Self::parse_hand).unzip()
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        let hands = &mut _parsed_input.0;
        hands.sort();
        Self::total_winnings(&hands).to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        let hands = &mut _parsed_input.1;
        hands.sort();
        Self::total_winnings(&hands).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn check_day07_part1_case1() {
        assert_eq!(Day07::solve_part_one(TEST_INPUT), "6440".to_string())
    }

    #[test]
    fn check_day07_part2_case1() {
        assert_eq!(Day07::solve_part_two(TEST_INPUT), "5905".to_string())
    }
}
