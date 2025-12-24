use crate::Solution;
use std::collections::HashSet;

pub struct Day07;

impl Day07 {
    fn count_splits(diagram: &Vec<Vec<char>>) -> usize {
        let start = diagram[0].iter().position(|&ch| ch == 'S').unwrap();
        let width = diagram[0].len();
        let mut beams = HashSet::from([start]);
        let mut total = 0;
        for row in 1..diagram.len() {
            let splitters: HashSet<usize> = diagram[row]
                .iter()
                .enumerate()
                .filter(|&(_, &ch)| ch == '^')
                .map(|(i, _)| i)
                .collect();
            for split in beams.clone().intersection(&splitters) {
                total += 1;
                beams.remove(&split);
                if *split >= 1 {
                    beams.insert(split - 1);
                }
                if *split < width - 1 {
                    beams.insert(split + 1);
                }
            }
        }
        total
    }
}

impl Solution for Day07 {
    type ParsedInput = Vec<Vec<char>>;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        input_lines
            .lines()
            .map(|line| line.chars().collect())
            .collect()
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        let diagram = _parsed_input;
        Self::count_splits(diagram).to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        "0".to_string()
        // TODO
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn check_day07_part1_case1() {
        assert_eq!(Day07::solve_part_one(TEST_INPUT), "21".to_string())
    }
}
