use crate::Solution;
use std::collections::HashMap;
use std::iter;

pub struct Day12;

impl Day12 {
    fn unfold(row: &Vec<char>, counts: &Vec<usize>) -> (Vec<char>, Vec<usize>) {
        (
            row.clone()
                .into_iter()
                .chain(iter::once('?'))
                .cycle()
                .take((row.len() + 1) * 5 - 1)
                .collect(),
            counts
                .clone()
                .into_iter()
                .cycle()
                .take(counts.len() * 5)
                .collect(),
        )
    }

    fn _initialize(counts: &Vec<usize>) -> (HashMap<(usize, char), usize>, usize) {
        let mut transitions = HashMap::new();
        let mut state = 0;
        let mut first = true;
        for &count in counts.iter() {
            if first {
                first = false;
            } else {
                transitions.insert((state, '.'), state + 1); // \.
                state += 1;
            }
            transitions.insert((state, '.'), state); // \.*
            for _ in 0..count {
                transitions.insert((state, '#'), state + 1); // #
                state += 1;
            }
        }
        transitions.insert((state, '.'), state); // \.*
        (transitions, state + 1)
    }

    fn _advance(
        transitions: &HashMap<(usize, char), usize>,
        positions: &Vec<usize>,
        c: char,
    ) -> Vec<usize> {
        let mut new_positions: Vec<usize> = vec![0; positions.len()];
        for (i, count) in positions.iter().enumerate() {
            if c == '.' || c == '?' {
                if let Some(&j) = transitions.get(&(i, '.')) {
                    new_positions[j] += count;
                }
            }
            if c == '#' || c == '?' {
                if let Some(&j) = transitions.get(&(i, '#')) {
                    new_positions[j] += count;
                }
            }
        }
        new_positions
    }

    // similar to:
    // https://alexoxorn.github.io/posts/aoc-day12-regular_languages/
    fn ways(row: &Vec<char>, counts: &Vec<usize>) -> usize {
        let (transitions, states) = Self::_initialize(counts);
        let mut positions: Vec<usize> = vec![0; states];
        positions[0] = 1;
        for &c in row.iter() {
            positions = Self::_advance(&transitions, &positions, c);
        }
        *positions.last().unwrap()
    }
}

impl Solution for Day12 {
    type ParsedInput = Vec<(Vec<char>, Vec<usize>)>;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        input_lines
            .lines()
            .map(|l| l.split_once(" ").unwrap())
            .map(|(row, counts)| {
                (
                    row.chars().collect(),
                    counts
                        .split(",")
                        .map(|x| x.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>(),
                )
            })
            .collect::<Vec<(Vec<char>, Vec<usize>)>>()
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        _parsed_input
            .iter()
            .map(|(row, counts)| Self::ways(row, counts))
            .sum::<usize>()
            .to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        _parsed_input
            .iter()
            .map(|(row, counts)| Self::unfold(row, counts))
            .map(|(row, counts)| Self::ways(&row, &counts))
            .sum::<usize>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn check_day12_part1_case1() {
        assert_eq!(Day12::solve_part_one(TEST_INPUT), "21".to_string())
    }

    #[test]
    fn check_day12_part2_case1() {
        assert_eq!(Day12::solve_part_two(TEST_INPUT), "525152".to_string())
    }
}
