use crate::Solution;
use std::collections::HashSet;

pub struct Day06;

impl Day06 {
    fn find_start(grid: &Vec<Vec<char>>) -> (usize, usize) {
        grid.iter()
            .enumerate()
            .filter_map(|(r, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(|(c, &ch)| {
                        if ch == '^' || ch == '>' || ch == '<' || ch == 'v' {
                            Some((r, c))
                        } else {
                            None
                        }
                    })
                    .next()
            })
            .next()
            .unwrap()
    }

    fn turn_right(dr: isize, dc: isize) -> (isize, isize) {
        match (dr, dc) {
            (0, 1) => (1, 0),
            (1, 0) => (0, -1),
            (0, -1) => (-1, 0),
            (-1, 0) => (0, 1),
            _ => panic!("Invalid character"),
        }
    }

    fn patrol(grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
        let mut positions = Vec::new();
        let (mut r, mut c) = Self::find_start(grid);
        let (mut dr, mut dc): (isize, isize) = match grid[r][c] {
            '>' => (0, 1),
            'v' => (1, 0),
            '<' => (0, -1),
            '^' => (-1, 0),
            _ => panic!("Invalid character"),
        };
        let max_r = grid.len();
        let max_c = grid[0].len();
        loop {
            positions.push((r, c)); // add current position to path
            if (r == 0 && dr < 0)
                || (c == 0 && dc < 0)
                || (r == max_r - 1 && dr > 0)
                || (c == max_c - 1 && dc > 0)
            {
                break; // leaving the grid
            }
            let (nr, nc) = ((r as isize + dr) as usize, (c as isize + dc) as usize);
            if grid[nr][nc] == '#' {
                // encountered an obstacle
                (dr, dc) = Self::turn_right(dr, dc);
                (r, c) = ((r as isize + dr) as usize, (c as isize + dc) as usize);
            } else {
                (r, c) = (nr, nc);
            }
        }
        positions
    }
}

impl Solution for Day06 {
    type ParsedInput = Vec<Vec<char>>;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        input_lines
            .lines()
            .map(|l| l.chars().collect())
            .collect::<Vec<Vec<char>>>()
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        let grid = _parsed_input;
        HashSet::<(usize, usize)>::from_iter(Self::patrol(grid))
            .len()
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

    const TEST_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn check_day06_part1_case1() {
        assert_eq!(Day06::solve_part_one(TEST_INPUT), "41".to_string())
    }
}
