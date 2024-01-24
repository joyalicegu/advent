use crate::Solution;
use std::collections::HashSet;

pub struct Day23;

impl Day23 {
    fn longest_hike_length(grid: &Vec<Vec<char>>) -> usize {
        // just do this the "dumb" way for part 1
        let (rows, cols) = (grid.len(), grid[0].len());
        let start = (0, grid[0].iter().position(|&p| p == '.').unwrap());
        let end = (
            rows - 1,
            grid[rows - 1].iter().position(|&p| p == '.').unwrap(),
        );
        let mut stack = Vec::new();
        stack.push((start, HashSet::new(), 0));
        let mut longest = 0;
        while let Some(state) = stack.pop() {
            let (current, mut visited, length) = state;
            visited.insert(current);
            if current == end {
                if length > longest {
                    longest = length;
                }
                continue;
            }
            let (r, c) = current;
            let (ir, ic) = (r as isize, c as isize);
            let (irows, icols) = (rows as isize, cols as isize);
            for (inr, inc) in [(ir + 1, ic), (ir, ic - 1), (ir - 1, ic), (ir, ic + 1)] {
                if inr < 0 || inc < 0 || inr >= irows || inc >= icols {
                    continue;
                }
                let (nr, nc) = (inr as usize, inc as usize);
                if visited.contains(&(nr, nc)) || grid[nr][nc] == '#' {
                    continue;
                }
                if match grid[r][c] {
                    '>' => (inr, inc) != (ir, ic + 1),
                    '^' => (inr, inc) != (ir - 1, ic),
                    '<' => (inr, inc) != (ir, ic - 1),
                    'v' => (inr, inc) != (ir + 1, ic),
                    _ => false,
                } {
                    continue;
                }
                stack.push(((nr, nc), visited.clone(), length + 1));
            }
        }
        longest
    }
}

impl Solution for Day23 {
    type ParsedInput = Vec<Vec<char>>;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        input_lines.lines().map(|l| l.chars().collect()).collect()
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        let grid = _parsed_input;
        Self::longest_hike_length(&grid).to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        "0".to_string()
        // TODO
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

    #[test]
    fn check_day23_part1_case1() {
        assert_eq!(Day23::solve_part_one(TEST_INPUT), "94".to_string())
    }
}
