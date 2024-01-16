use crate::Solution;
use std::collections::{HashSet, VecDeque};

pub struct Day21;

impl Day21 {
    fn start(grid: &Vec<Vec<char>>) -> (usize, usize) {
        for r in 0..grid.len() {
            for c in 0..grid[r].len() {
                if grid[r][c] == 'S' {
                    return (r, c);
                }
            }
        }
        panic!("Day 21: start not found");
    }

    fn reachable(grid: &Vec<Vec<char>>, steps: usize) -> HashSet<(usize, (usize, usize))> {
        let start = Self::start(grid);
        let (rows, cols) = (grid.len() as isize, grid[0].len() as isize);
        let mut queue = VecDeque::from([(steps, start)]);
        let mut reached = HashSet::new();
        let mut visited = HashSet::new();
        while let Some((mut i, (r, c))) = queue.pop_front() {
            if visited.contains(&(r, c)) {
                continue;
            }
            visited.insert((r, c));
            reached.insert((i, (r, c)));
            if i == 0 {
                continue;
            }
            i -= 1;
            let (r, c) = (r as isize, c as isize);
            for (nr, nc) in [(r + 1, c), (r, c - 1), (r - 1, c), (r, c + 1)] {
                if nr < 0 || nr >= rows || nc < 0 || nc >= cols {
                    continue;
                }
                let (unr, unc) = (nr as usize, nc as usize);
                if grid[unr][unc] == '#' {
                    continue;
                }
                queue.push_back((i, (unr, unc)));
            }
        }
        reached
    }

    fn reachable_in_steps(grid: &Vec<Vec<char>>, steps: usize) -> HashSet<(usize, usize)> {
        Self::reachable(grid, steps)
            .iter()
            .filter(|&(i, _)| (steps - i) % 2 == 0)
            .map(|&(_, (r, c))| (r, c))
            .collect()
    }
}

impl Solution for Day21 {
    type ParsedInput = Vec<Vec<char>>;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        input_lines
            .lines()
            .map(|l| l.chars().collect())
            .collect::<Vec<Vec<char>>>()
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        let grid = _parsed_input;
        Self::reachable_in_steps(&grid, 64).len().to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        "0".to_string()
        // TODO
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

    #[test]
    fn check_day21_part1_case1() {
        assert_eq!(
            Day21::reachable_in_steps(&Day21::parse_input(TEST_INPUT), 6).len(),
            16
        )
    }
}
