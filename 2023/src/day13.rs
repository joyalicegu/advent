use crate::Solution;
use std::collections::HashMap;

pub struct Day13;

pub struct Pattern {
    rows: Vec<String>,
    cols: Vec<String>,
}

impl Day13 {
    fn reflection_index(elements: &Vec<String>) -> Option<usize> {
        let mut map: HashMap<(usize, usize), usize> = HashMap::new();
        let mut result = None;
        // adjacent pairs
        for i in 0..(elements.len() - 1) {
            let j = i + 1;
            let x = j;
            if elements[i] == elements[j] {
                map.insert((i, j), x);
                if i == 0 || j == elements.len() - 1 {
                    result = Some(x);
                }
            }
        }
        // increasing lengths
        for k in (4..elements.len()).step_by(2) {
            for i in 0..(elements.len() - k + 1) {
                let j = i + k - 1;
                if let Some(&x) = map.get(&(i + 1, j - 1)) {
                    if elements[i] == elements[j] {
                        map.insert((i, j), x);
                        if i == 0 || j == elements.len() - 1 {
                            result = Some(x);
                        }
                    }
                }
            }
        }
        result
    }

    fn find_reflection(pattern: &Pattern) -> (Option<usize>, Option<usize>) {
        let h = Self::reflection_index(&pattern.rows);
        let v = Self::reflection_index(&pattern.cols);
        // println!("h={:?} v={:?}", h, v);
        (h, v)
    }

    fn summarize(reflection: (Option<usize>, Option<usize>)) -> usize {
        let (h, v) = reflection;
        v.unwrap_or(0) + 100 * h.unwrap_or(0)
    }
}

impl Solution for Day13 {
    type ParsedInput = Vec<Pattern>;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        input_lines
            .split("\n\n")
            .map(|p| {
                let grid = p
                    .lines()
                    .map(|l| l.chars().collect())
                    .collect::<Vec<Vec<char>>>();
                let rows = p.lines().map(|l| l.to_string()).collect();
                let cols = (0..grid[0].len())
                    .map(|c| (0..grid.len()).map(|r| grid[r][c]).collect::<String>())
                    .collect();
                Pattern {
                    rows: rows,
                    cols: cols,
                }
            })
            .collect()
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        let patterns = _parsed_input;
        patterns
            .iter()
            .map(Self::find_reflection)
            .map(Self::summarize)
            .sum::<usize>()
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

    const TEST_INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn check_day13_part1_case1() {
        assert_eq!(Day13::solve_part_one(TEST_INPUT), "405".to_string())
    }

    #[test]
    fn check_day13_part1_case2() {
        assert_eq!(
            Day13::solve_part_one(
                "#...#..######.#.#
...#.###.##..#.#.
###....#..#..#.#.
.##.######.###.#.
##..#..#..###....
##..#..#..###....
.##.##########.#.
###....#..#..#.#.
...#.###.##..#.#.
#...#..######.#.#
.##.#.#..#....##.
.##.#.#..#....##.
#...#..######.#.#"
            ),
            "1100".to_string()
        )
    }
}
