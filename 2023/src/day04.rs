use crate::Solution;
use std::cmp;
use std::collections::HashSet;
use std::iter;

pub struct Day04;

pub struct Card {
    matches: usize,
}

impl Day04 {
    fn parse_card(line: &str) -> Card {
        let (_, rest) = line.split_once(": ").unwrap();
        let (w_str, y_str) = rest.split_once(" | ").unwrap();
        let w: Vec<u32> = w_str
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect();
        let y: Vec<u32> = y_str
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect();
        let set: HashSet<&u32> = HashSet::from_iter(w.iter());
        let matches = y.iter().filter(|n| set.contains(n)).count();
        Card { matches: matches }
    }

    fn points(card: &Card) -> u32 {
        if card.matches > 0 {
            1 << (card.matches - 1)
        } else {
            0
        }
    }

    fn process_scratchcards(cards: &Vec<Card>) -> usize {
        let mut unprocessed: Vec<usize> = iter::repeat(1).take(cards.len()).collect();
        let mut processed: Vec<usize> = iter::repeat(0).take(cards.len()).collect();
        while unprocessed.iter().sum::<usize>() > 0 {
            for (i, card) in cards.iter().enumerate() {
                let instances = unprocessed[i];
                if instances == 0 {
                    continue;
                }
                for j in (i + 1)..=(cmp::min(i + card.matches, cards.len())) {
                    unprocessed[j] += instances;
                }
                // move cards to processed
                unprocessed[i] -= instances;
                processed[i] += instances;
            }
        }
        processed.iter().sum::<usize>()
    }
}

impl Solution for Day04 {
    type ParsedInput = Vec<Card>;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        input_lines.lines().map(Self::parse_card).collect()
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        let cards = _parsed_input;
        cards.iter().map(Self::points).sum::<u32>().to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        let cards = _parsed_input;
        Self::process_scratchcards(&cards).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn check_day04_part1_case1() {
        assert_eq!(Day04::solve_part_one(TEST_INPUT), "13".to_string())
    }

    #[test]
    fn check_day04_part2_case1() {
        assert_eq!(Day04::solve_part_two(TEST_INPUT), "30".to_string())
    }
}
