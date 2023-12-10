use crate::Solution;
use itertools::Itertools;
use num::integer::lcm;
use regex::Regex;
use std::collections::HashMap;

pub struct Day08;

pub enum Direction {
    Left,
    Right,
}

type Node = String;

impl Day08 {
    fn traverse(
        directions: &Vec<Direction>,
        network: &HashMap<Node, (Node, Node)>,
        direction_index: usize,
        src: Node,
        at_dst: &dyn Fn(&Node) -> bool,
    ) -> (Node, usize) {
        let mut node = &src;
        let mut steps: usize = 0;
        let mut i = direction_index;
        loop {
            node = match directions[i] {
                Direction::Left => &network.get(node).unwrap().0,
                Direction::Right => &network.get(node).unwrap().1,
            };
            i = (i + 1) % directions.len();
            steps += 1;
            if at_dst(node) {
                break;
            }
            if node == &src && i == 0 {
                panic!("Day 8: Can't find a path between nodes")
            }
        }
        (node.to_string(), steps)
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
        let at_dst = |node: &Node| node == &dst;
        Self::traverse(&directions, &network, 0, src, &at_dst)
            .1
            .to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        let (directions, network) = _parsed_input;
        let at_dst = |node: &Node| node.ends_with('Z');
        let srcs = network
            .keys()
            .filter(|n| n.ends_with('A'))
            .map(|n| n.to_string());
        let (dsts, paths): (Vec<Node>, Vec<usize>) = srcs
            .map(|src| Self::traverse(&directions, &network, 0, src, &at_dst))
            .unzip();
        {
            // assert that your solution works for the given input
            let cycles: Vec<usize> = dsts
                .into_iter()
                .enumerate()
                .map(|(j, dst)| {
                    let i = paths[j] % directions.len();
                    let at_dst = |node: &Node| node == &dst;
                    Self::traverse(&directions, &network, i, dst.clone(), &at_dst).1
                })
                .collect();
            assert!(
                paths.iter().zip(cycles.iter()).all(|(p, c)| p == c),
                "paths {:?} != cycles {:?}",
                paths,
                cycles
            );
        }
        paths.into_iter().fold(1, |acc, n| lcm(acc, n)).to_string()
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

    #[test]
    fn check_day08_part2_case1() {
        assert_eq!(
            Day08::solve_part_two(
                "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"
            ),
            "6".to_string()
        )
    }
}
