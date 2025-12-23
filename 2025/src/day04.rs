use crate::Solution;
use itertools::iproduct;
use std::collections::{HashMap, HashSet};

pub struct Day04;

impl Day04 {
    fn neighbors(r: usize, c: usize, rows: usize, cols: usize) -> Vec<(usize, usize)> {
        iproduct!(r.saturating_sub(1)..=(r + 1), c.saturating_sub(1)..=(c + 1))
            .filter(|(nr, nc)| (r, c) != (*nr, *nc) && (*nr < rows) && (*nc < cols))
            .collect()
    }

    fn graph(grid: &Vec<Vec<char>>) -> HashMap<(usize, usize), HashSet<(usize, usize)>> {
        let (rows, cols) = (grid.len(), grid[0].len());
        let mut adj = HashMap::new();
        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] != '@' {
                    continue;
                }
                adj.insert((r, c), HashSet::new());
                for &(nr, nc) in Self::neighbors(r, c, rows, cols).iter() {
                    if grid[nr][nc] == '@' {
                        adj.entry((r, c)).or_insert(HashSet::new()).insert((nr, nc));
                    }
                }
            }
        }
        adj
    }

    fn accessible(
        adj: &HashMap<(usize, usize), HashSet<(usize, usize)>>,
        k: usize,
    ) -> Vec<(usize, usize)> {
        let mut result = Vec::new();
        for (&node, neighbors) in adj {
            if neighbors.len() < k {
                result.push(node);
            }
        }
        result
    }

    fn k_core(
        adj: &HashMap<(usize, usize), HashSet<(usize, usize)>>,
        k: usize,
    ) -> HashMap<(usize, usize), HashSet<(usize, usize)>> {
        // https://en.wikipedia.org/wiki/Degeneracy_(graph_theory)#k-Cores
        let mut result = adj.clone();
        loop {
            let nodes = Self::accessible(&result, k);
            if nodes.is_empty() {
                break;
            }
            // println!("{:?}", nodes.len());
            for node in nodes {
                let neighbors: HashSet<(usize, usize)> = result.get(&node).unwrap().clone();
                for neighbor in neighbors.into_iter() {
                    result
                        .entry(neighbor)
                        .or_insert(HashSet::new())
                        .remove(&node);
                }
                result.remove(&node);
            }
        }
        result
    }
}

impl Solution for Day04 {
    type ParsedInput = HashMap<(usize, usize), HashSet<(usize, usize)>>;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        let grid: Vec<Vec<char>> = input_lines
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        Self::graph(&grid)
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        let adj = _parsed_input;
        Self::accessible(adj, 4).len().to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        let adj = _parsed_input;
        let before = adj.keys().len();
        let after = Self::k_core(adj, 4).keys().len();
        (before - after).to_string()
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

    #[test]
    fn check_day04_part2_case1() {
        assert_eq!(Day04::solve_part_two(TEST_INPUT), "43".to_string())
    }

    #[test]
    fn check_day04_part2_case2() {
        assert_eq!(
            Day04::solve_part_two(
                "...
.@.
..."
            ),
            "1".to_string()
        )
    }
}
