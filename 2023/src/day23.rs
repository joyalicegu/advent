use crate::Solution;
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

pub struct Day23;

impl Day23 {
    fn find_start(grid: &Vec<Vec<char>>) -> (usize, usize) {
        (0, grid[0].iter().position(|&p| p == '.').unwrap())
    }

    fn find_end(grid: &Vec<Vec<char>>) -> (usize, usize) {
        let rows = grid.len();
        (
            rows - 1,
            grid[rows - 1].iter().position(|&p| p == '.').unwrap(),
        )
    }

    fn longest_hike_length(grid: &Vec<Vec<char>>) -> usize {
        // just do this the "dumb" way for part 1
        let (rows, cols) = (grid.len(), grid[0].len());
        let start = Self::find_start(grid);
        let end = Self::find_end(grid);
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

    fn graph(grid: &Vec<Vec<char>>) -> HashMap<(usize, usize), HashSet<((usize, usize), usize)>> {
        let (rows, cols) = (grid.len(), grid[0].len());
        let (irows, icols) = (rows as isize, cols as isize);
        let start = Self::find_start(grid);
        let end = Self::find_end(grid);
        let mut nodes = Vec::from([start, end]);
        for r in 1..(rows - 1) {
            for c in 1..(cols - 1) {
                if grid[r][c] == '#' {
                    continue;
                }
                let neighbors: Vec<(usize, usize)> =
                    [(r + 1, c), (r, c - 1), (r - 1, c), (r, c + 1)]
                        .into_iter()
                        .filter(|&(nr, nc)| grid[nr][nc] != '#')
                        .collect();
                if neighbors.len() > 2 {
                    nodes.push((r, c));
                }
            }
        }
        let mut adj = HashMap::new();
        let mut visited = HashSet::new();
        for &i in nodes.iter() {
            let mut queue = VecDeque::from([(i, 0)]);
            visited.insert(i);
            while let Some(((r, c), steps)) = queue.pop_front() {
                let (ir, ic) = (r as isize, c as isize);
                for (inr, inc) in [(ir + 1, ic), (ir, ic - 1), (ir - 1, ic), (ir, ic + 1)] {
                    if inr < 0 || inc < 0 || inr >= irows || inc >= icols {
                        continue;
                    }
                    let (nr, nc) = (inr as usize, inc as usize);
                    if visited.contains(&(nr, nc)) || grid[nr][nc] == '#' {
                        continue;
                    }
                    let j = (nr, nc);
                    if nodes.contains(&j) {
                        adj.entry(i)
                            .or_insert(HashSet::new())
                            .insert((j, steps + 1));
                        adj.entry(j)
                            .or_insert(HashSet::new())
                            .insert((i, steps + 1));
                        // don't add to visited because we may want to visit from another direction
                    } else {
                        queue.push_back((j, steps + 1));
                        visited.insert(j);
                    }
                }
            }
        }
        adj
    }

    fn longest_path_length(
        adj: &HashMap<(usize, usize), HashSet<((usize, usize), usize)>>,
        start: (usize, usize),
        end: (usize, usize),
    ) -> usize {
        let mut stack = Vec::from([(start, Vec::new(), HashSet::new(), 0)]);
        let mut longest = 0;
        let mut longest_history = None;
        while let Some(state) = stack.pop() {
            let (src, history, mut visited, length) = state;
            if visited.contains(&src) {
                continue;
            }
            visited.insert(src);
            if src == end {
                if length > longest {
                    longest = length;
                    longest_history = Some(history.clone());
                }
                continue;
            }
            let Some(edges) = adj.get(&src) else {
                continue;
            };
            for &(dst, steps) in edges.iter() {
                let mut history_ = history.clone();
                history_.push((dst, steps));
                stack.push((dst, history_, visited.clone(), length + steps));
            }
        }
        let history = longest_history.unwrap();
        println!("history:");
        for (node, steps) in history.iter() {
            println!("{:?} steps to {:?}", steps, node);
        }
        println!("longest: {:?}", longest);
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
        let grid = _parsed_input;
        let start = Self::find_start(&grid);
        let end = Self::find_end(&grid);
        println!("start: {:?}", start);
        println!("  end: {:?}", end);
        let adj = Self::graph(&grid);

        println!("adj:");
        for src in adj.keys().sorted() {
            let v = adj.get(src).unwrap();
            println!("{:?}: {:?}", src, v);
        }

        Self::longest_path_length(&adj, start, end).to_string()
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

    #[test]
    fn check_day23_part2_case1() {
        assert_eq!(Day23::solve_part_two(TEST_INPUT), "154".to_string())
    }
}
