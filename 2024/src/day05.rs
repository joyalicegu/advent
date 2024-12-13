use crate::Solution;
use itertools::iproduct;
use itertools::Itertools;
use std::collections::HashSet;

pub struct Day05;

impl Day05 {
    fn correctly_ordered(update: &Vec<u32>, rules: &HashSet<(u32, u32)>) -> bool {
        iproduct!(update.iter().enumerate(), update.iter().enumerate())
            .filter(|((i, &a), (j, &b))| i < j && rules.contains(&(b, a)))
            .count()
            == 0
    }

    fn middle_page_number(update: &Vec<u32>) -> u32 {
        update[update.len() / 2]
    }
}

impl Solution for Day05 {
    type ParsedInput = (HashSet<(u32, u32)>, Vec<Vec<u32>>);

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        let (rules_str, updates_str) = input_lines.split_once("\n\n").unwrap();
        let rules = rules_str
            .lines()
            .map(|line| {
                line.split("|")
                    .map(|s| s.parse::<u32>().unwrap())
                    .collect_tuple()
                    .unwrap()
            })
            .collect();
        let updates = updates_str
            .lines()
            .map(|line| line.split(",").map(|s| s.parse::<u32>().unwrap()).collect())
            .collect();
        (rules, updates)
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        let (rules, updates) = _parsed_input;
        updates
            .iter()
            .filter(|u| Self::correctly_ordered(u, rules))
            .map(Self::middle_page_number)
            .sum::<u32>()
            .to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        "0".to_string()
        // TODO
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn check_day05_part1_case1() {
        assert_eq!(Day05::solve_part_one(TEST_INPUT), "143".to_string())
    }
}
