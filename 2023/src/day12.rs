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

    fn _ways(
        row: &Vec<char>,
        counts: &Vec<usize>,
        j: usize,
        count: Option<usize>,
        cache: &mut HashMap<(Vec<char>, usize, Option<usize>), usize>,
    ) -> usize {
        // sorry, i know this isn't very attractive...
        let key = (row.clone(), j, count);
        let mut j = j;
        let mut count = count;
        if let Some(result) = cache.get(&key) {
            return *result;
        }
        for i in 0..row.len() {
            if row[i] == '#' {
                if let Some(x) = count {
                    count = Some(x + 1);
                    if j >= counts.len() || (x + 1) > counts[j] {
                        cache.insert(key, 0);
                        return 0; // invalid
                    }
                } else {
                    count = Some(1)
                }
            } else if row[i] == '.' {
                if let Some(x) = count {
                    if j >= counts.len() || x < counts[j] {
                        cache.insert(key, 0);
                        return 0; // invalid
                    }
                    j += 1;
                    count = None;
                }
            } else if row[i] == '?' {
                let mut result = 0;
                let mut mut_row: Vec<char> = row[i..].to_vec();
                // pick '#'
                mut_row[0] = '#';
                result += Self::_ways(&mut_row, &counts, j, count, cache);
                // pick '.'
                mut_row[0] = '.';
                result += Self::_ways(&mut_row, &counts, j, count, cache);
                cache.insert(key, result);
                return result;
            }
        }
        if let Some(x) = count {
            if j >= counts.len() || x < counts[j] {
                cache.insert(key, 0);
                return 0; // invalid
            }
            j += 1;
        }
        if j != counts.len() {
            cache.insert(key, 0);
            return 0; // invalid
        }
        1 // populated and valid
    }

    fn ways(row: &Vec<char>, counts: &Vec<usize>) -> usize {
        let mut cache = HashMap::new();
        Self::_ways(row, counts, 0, None, &mut cache)
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
