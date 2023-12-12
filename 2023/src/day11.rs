use crate::Solution;
use std::collections::HashSet;

pub struct Day11;

impl Day11 {
    fn expanded_manhattan_distance(
        galaxies: &Vec<(usize, usize)>,
        populated_rows: &HashSet<usize>,
        populated_cols: &HashSet<usize>,
        expansion_factor: usize,
        i: usize,
        j: usize,
    ) -> usize {
        let ((ir, ic), (jr, jc)) = (galaxies[i], galaxies[j]);
        let (r_lo, r_hi) = if ir < jr { (ir, jr) } else { (jr, ir) };
        let (c_lo, c_hi) = if ic < jc { (ic, jc) } else { (jc, ic) };
        let normal_rows = populated_rows
            .iter()
            .filter(|&r| &r_lo <= r && r < &r_hi)
            .count();
        let normal_cols = populated_cols
            .iter()
            .filter(|&c| &c_lo <= c && c < &c_hi)
            .count();
        let expanded_rows = (r_hi - r_lo) - normal_rows;
        let expanded_cols = (c_hi - c_lo) - normal_cols;
        normal_rows + normal_cols + (expanded_rows + expanded_cols) * expansion_factor
    }

    fn distances(galaxies: &Vec<(usize, usize)>, expansion_factor: usize) -> Vec<usize> {
        let populated_rows = galaxies.iter().map(|&(r, _)| r).collect::<HashSet<usize>>();
        let populated_cols = galaxies.iter().map(|&(_, c)| c).collect::<HashSet<usize>>();
        let mut distances = Vec::new();
        for i in 0..galaxies.len() {
            for j in i + 1..galaxies.len() {
                distances.push(Self::expanded_manhattan_distance(
                    &galaxies,
                    &populated_rows,
                    &populated_cols,
                    expansion_factor,
                    i,
                    j,
                ));
            }
        }
        distances
    }
}

impl Solution for Day11 {
    type ParsedInput = Vec<(usize, usize)>;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        input_lines
            .lines()
            .enumerate()
            .map(|(r, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(move |(c, space)| if space == '#' { Some((r, c)) } else { None })
            })
            .flatten()
            .collect()
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        let galaxies = _parsed_input;
        Self::distances(galaxies, 2)
            .iter()
            .sum::<usize>()
            .to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        let galaxies = _parsed_input;
        Self::distances(galaxies, 1000000)
            .iter()
            .sum::<usize>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn check_day11_part1_case1() {
        assert_eq!(Day11::solve_part_one(TEST_INPUT), "374".to_string())
    }
}
