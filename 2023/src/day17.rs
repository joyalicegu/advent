use crate::Solution;
use core::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

pub struct Day17;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    fn left(&self) -> Direction {
        match self {
            Direction::N => Direction::W,
            Direction::E => Direction::N,
            Direction::S => Direction::E,
            Direction::W => Direction::S,
        }
    }

    fn right(&self) -> Direction {
        match self {
            Direction::N => Direction::E,
            Direction::E => Direction::S,
            Direction::S => Direction::W,
            Direction::W => Direction::N,
        }
    }

    fn offsets(&self) -> (isize, isize) {
        match self {
            Direction::N => (-1, 0),
            Direction::E => (0, 1),
            Direction::S => (1, 0),
            Direction::W => (0, -1),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct State {
    heat_loss: u32,
    posdir: (usize, usize, Direction),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .heat_loss
            .cmp(&self.heat_loss)
            .then_with(|| self.posdir.cmp(&other.posdir))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Day17 {
    fn move_n(
        steps: usize,
        d: Direction,
        r: usize,
        c: usize,
        bounds: (usize, usize),
    ) -> Option<(usize, usize)> {
        let (dr, dc) = d.offsets();
        let (nr, nc) = (
            r as isize + dr * steps as isize,
            c as isize + dc * steps as isize,
        );
        if nr < 0 || nr >= bounds.0 as isize || nc < 0 || nc >= bounds.1 as isize {
            return None;
        }
        Some((nr as usize, nc as usize))
    }

    fn least_heat_loss(grid: &Vec<Vec<u32>>, min_steps: usize, max_steps: usize) -> u32 {
        // dijkstra's algorithm?
        let (rows, cols) = (grid.len(), grid[0].len());
        let bounds = (rows, cols);
        let mut heap = BinaryHeap::new();
        let mut dist = HashMap::<(usize, usize, Direction), u32>::new();
        heap.push(State {
            heat_loss: 0,
            posdir: (0, 0, Direction::N),
        });
        dist.insert((0, 0, Direction::N), 0);
        heap.push(State {
            heat_loss: 0,
            posdir: (0, 0, Direction::W),
        });
        dist.insert((0, 0, Direction::W), 0);
        while let Some(State { heat_loss, posdir }) = heap.pop() {
            let (r, c, d) = posdir;
            if (r, c) == (rows - 1, cols - 1) {
                continue;
            };
            if let Some(&min_heat_loss) = dist.get(&(r, c, d)) {
                if heat_loss > min_heat_loss {
                    continue;
                }
            }
            for nd in [d.left(), d.right()] {
                for i in min_steps..=max_steps {
                    if let Some((nr, nc)) = Self::move_n(i, nd, r, c, bounds) {
                        let cost = (1..=i)
                            .map(|j| {
                                let (nnr, nnc) = Self::move_n(j, nd, r, c, bounds).unwrap();
                                grid[nnr][nnc]
                            })
                            .sum::<u32>();
                        let state = State {
                            heat_loss: heat_loss + cost,
                            posdir: (nr, nc, nd),
                        };
                        if let Some(&min_heat_loss) = dist.get(&(nr, nc, nd)) {
                            if state.heat_loss < min_heat_loss {
                                heap.push(state);
                                dist.insert((nr, nc, nd), state.heat_loss);
                            }
                        } else {
                            heap.push(state);
                            dist.insert((nr, nc, nd), state.heat_loss);
                        }
                    }
                }
            }
        }
        *[Direction::N, Direction::E, Direction::S, Direction::W]
            .into_iter()
            .filter_map(|d| dist.get(&(rows - 1, cols - 1, d)))
            .min()
            .unwrap()
    }
}

impl Solution for Day17 {
    type ParsedInput = Vec<Vec<u32>>;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        input_lines
            .lines()
            .map(|l| l.chars().filter_map(|c| c.to_digit(10)).collect())
            .collect::<Vec<Vec<u32>>>()
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        let grid = _parsed_input;
        Self::least_heat_loss(grid, 1, 3).to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        let grid = _parsed_input;
        Self::least_heat_loss(grid, 4, 10).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    #[test]
    fn check_day17_part1_case1() {
        assert_eq!(Day17::solve_part_one(TEST_INPUT), "102".to_string())
    }

    #[test]
    fn check_day17_part2_case1() {
        assert_eq!(Day17::solve_part_two(TEST_INPUT), "94".to_string())
    }
}
