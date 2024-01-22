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

    fn reachable(grid: &Vec<Vec<char>>, steps: Option<usize>) -> HashSet<(usize, (usize, usize))> {
        let start = Self::start(grid);
        let (rows, cols) = (grid.len() as isize, grid[0].len() as isize);
        let mut queue = VecDeque::from([(0, start)]);
        let mut reached = HashSet::new();
        let mut visited = HashSet::new();
        while let Some((mut i, (r, c))) = queue.pop_front() {
            if visited.contains(&(r, c)) {
                continue;
            }
            visited.insert((r, c));
            reached.insert((i, (r, c)));
            if let Some(limit) = steps {
                if i == limit {
                    continue;
                }
            }
            i += 1;
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
        Self::reachable(grid, Some(steps))
            .iter()
            .filter(|&(i, _)| i % 2 == 0)
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
        let grid = _parsed_input;

        /* the input contains two diamonds.
         * let's call them the corner diamond and the center diamond.
         * the center and bordering rows and columns are clear.
         * the S is in the center of the center diamond.
         *
         * the size of the input is 131 x 131.
         * it takes (131 - 1) / 2 = 65 steps to reach the side from S.
         *
         * given no obstructions, we would expect the reachable steps
         * to form a kind of banded-diamond expanding outward.
         * the center columns/rows are clear,
         * so we know the centers will be reachable.
         * 131 / 2 = 65 is odd, so from S, the edges will not be reachable
         * so reachable tiles will alternate in parity as we go outward.
         *
         * steps = 65 + 131 * x
         *
         * A = center diamond (same parity as center) (odd in our case)
         * A' = center diamond (alt. parity)
         * B = corners (same parity as center)
         * B' = corners (alt. parity)
         * C = corner diamond = (B + B') / 2
         *
         * ((n + 1)^2 - (n - 1)^2) / 2 = 4 * (n - 1)
         * https://en.wikipedia.org/wiki/Difference_of_two_squares#Difference_of_two_consecutive_perfect_squares
         *
         * f(0) = A
         * f(1) = f(0) + 4*1 (A' + C) = A + 4A' + 4C
         * f(2) = f(1) + 4*2 (A + C)  = 9A + 4A' + 12C
         * f(3) = f(2) + 4*3 (A' + C) = 9A + 16A' + 24C
         * f(4) = f(3) + 4*4 (A + C)  = 25A + 16A' + 40C
         * f(5) = f(4) + 4*5 (A' + C) = 25A + 36A' + 60C
         * f(6) = f(5) + 4*6 (A + C)  = 49A + 36A' + 84C
         *  odd x: f(x) = x^2 A + (x+1)^2 A' + 2x(x+1) C
         * even x: f(x) = (x+1)^2 A + x^2 A' + 2x(x+1) C
         */

        let reached = Self::reachable(&grid, None);

        let odd_center_diamond = reached
            .iter()
            .filter(|&(i, _)| *i % 2 != 0 && *i <= 65)
            .count();
        let even_center_diamond = reached
            .iter()
            .filter(|&(i, _)| *i % 2 == 0 && *i <= 65)
            .count();
        let corner_diamond = reached.iter().filter(|&(i, _)| *i > 65).count() / 2;

        let x = 202300; // (26501365 - 65) / 131
        let result = (x + 1) * (x + 1) * odd_center_diamond
            + x * x * even_center_diamond
            + 2 * x * (x + 1) * corner_diamond;

        result.to_string()
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
