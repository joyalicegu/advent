use crate::Solution;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

pub struct Day08;

pub enum Direction {
    Left,
    Right,
}

type Node = String;

impl Day08 {
    fn steps(
        directions: &Vec<Direction>,
        network: &HashMap<Node, (Node, Node)>,
        src: Node,
        dst: Node,
    ) -> usize {
        let mut node = &src;
        let mut steps: usize = 0;
        let mut i = 0;
        while node != &dst {
            node = match directions[i] {
                Direction::Left => &network[node].0,
                Direction::Right => &network[node].1,
            };
            i = (i + 1) % directions.len();
            steps += 1;
            if node == &src && i == 0 {
                panic!("Day 8: Can't find a path between nodes")
            }
        }
        steps
    }
}

impl Solution for Day08 {
    type ParsedInput = (Vec<Direction>, HashMap<Node, (Node, Node)>);

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        let (first, remainder) = input_lines.split_once("\n\n").unwrap();
        let directions: Vec<Direction> = first
            .chars()
            .map(|c| match c {
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => panic!("Day 8: Invalid Input"),
            })
            .collect();
        let node_re = Regex::new(r"[A-Z0-9]{3}").unwrap();
        let network: HashMap<Node, (Node, Node)> = remainder
            .lines()
            .map(|line| {
                let (node, left, right) = node_re
                    .find_iter(line)
                    .map(|m| m.as_str().to_string())
                    .collect_tuple()
                    .unwrap();
                (node, (left, right))
            })
            .collect();
        (directions, network)
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        let (directions, network) = _parsed_input;
        let (src, dst) = ("AAA".to_string(), "ZZZ".to_string());
        Self::steps(&directions, &network, src, dst).to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        "0".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day08_part1_case1() {
        assert_eq!(
            Day08::solve_part_one(
                "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"
            ),
            "2".to_string()
        )
    }

    #[test]
    fn check_day08_part1_case2() {
        assert_eq!(
            Day08::solve_part_one(
                "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"
            ),
            "6".to_string()
        )
    }
}
