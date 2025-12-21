use crate::Solution;
use itertools::iproduct;

pub struct Day04;

impl Day04 {
    fn neighbors(r: usize, c: usize, rows: usize, cols: usize) -> Vec<(usize, usize)> {
        iproduct!(r.saturating_sub(1)..=(r + 1), c.saturating_sub(1)..=(c + 1))
            .filter(|(nr, nc)| (r, c) != (*nr, *nc) && (*nr < rows) && (*nc < cols))
            .collect()
    }

    fn accessible(grid: &Vec<Vec<char>>) -> usize {
        let rows = grid.len();
        let cols = grid[0].len();
        let mut count = 0;
        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] != '@' {
                    continue;
                }
                let mut adjacent = 0;
                for &(nr, nc) in Self::neighbors(r, c, rows, cols).iter() {
                    if grid[nr][nc] == '@' {
                        adjacent += 1
                    };
                }
                if adjacent < 4 {
                    count += 1;
                }
            }
        }
        count
    }
}

impl Solution for Day04 {
    type ParsedInput = Vec<Vec<char>>;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        input_lines
            .lines()
            .map(|line| line.chars().collect())
            .collect()
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        Self::accessible(&_parsed_input).to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        "0".to_string()
        // TODO
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn check_day04_part1_case1() {
        assert_eq!(Day04::solve_part_one(TEST_INPUT), "13".to_string())
    }
}
