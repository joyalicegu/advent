use crate::Solution;
use itertools::Itertools;

pub struct Day10;

#[derive(Copy, Clone)]
pub enum Direction {
    N,
    E,
    S,
    W,
}

impl Day10 {
    fn next_row_and_col(row: usize, col: usize, next_from: Direction) -> (usize, usize) {
        match next_from {
            Direction::N => (row + 1, col),
            Direction::E => (row, col.saturating_sub(1)),
            Direction::S => (row.saturating_sub(1), col),
            Direction::W => (row, col + 1),
        }
    }

    fn follow_pipe(
        pipes: &Vec<Vec<char>>,
        position: (usize, usize, Direction),
    ) -> (usize, usize, Direction) {
        let (row, col, from) = position;
        let pipe = pipes[row][col];
        let next_from = match (pipe, from) {
            ('|', Direction::N) => Direction::N,
            ('|', Direction::S) => Direction::S,
            ('-', Direction::E) => Direction::E,
            ('-', Direction::W) => Direction::W,
            ('L', Direction::N) => Direction::W,
            ('L', Direction::E) => Direction::S,
            ('J', Direction::N) => Direction::E,
            ('J', Direction::W) => Direction::S,
            ('7', Direction::S) => Direction::E,
            ('7', Direction::W) => Direction::N,
            ('F', Direction::S) => Direction::W,
            ('F', Direction::E) => Direction::N,
            _ => panic!("Day 10: Invalid direction for pipe {}", pipe),
        };
        let (next_row, next_col) = Self::next_row_and_col(row, col, next_from);
        (next_row, next_col, next_from)
    }

    fn valid_pipe(pipe: char, from: Direction) -> bool {
        match from {
            Direction::N => "|LJ".contains(pipe),
            Direction::E => "-LF".contains(pipe),
            Direction::S => "|7F".contains(pipe),
            Direction::W => "-J7".contains(pipe),
        }
    }

    fn find_start(pipes: &Vec<Vec<char>>) -> (usize, usize) {
        pipes
            .iter()
            .enumerate()
            .filter_map(|(r, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(|(c, &pipe)| if pipe == 'S' { Some((r, c)) } else { None })
                    .next()
            })
            .next()
            .unwrap()
    }

    fn neighbors(
        pipes: &Vec<Vec<char>>,
        start: (usize, usize),
    ) -> ((usize, usize, Direction), (usize, usize, Direction)) {
        let (row, col) = start;
        [Direction::N, Direction::E, Direction::S, Direction::W]
            .into_iter()
            .filter_map(|next_from| {
                let (next_row, next_col) = Self::next_row_and_col(row, col, next_from);
                if let Some(row) = pipes.get(next_row) {
                    if let Some(pipe) = row.get(next_col) {
                        if Self::valid_pipe(*pipe, next_from) {
                            return Some((next_row, next_col, next_from));
                        }
                    }
                }
                return None;
            })
            .collect_tuple()
            .unwrap()
    }

    fn find_loop(pipes: &Vec<Vec<char>>) -> Vec<(usize, usize, Direction)> {
        let start = Self::find_start(&pipes);
        let mut a = Self::neighbors(&pipes, start).0;
        let mut positions = Vec::from([a]);
        while (a.0, a.1) != (start.0, start.1) {
            a = Self::follow_pipe(&pipes, a);
            positions.push(a);
        }
        positions
    }
}

impl Solution for Day10 {
    type ParsedInput = Vec<Vec<char>>;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        input_lines
            .lines()
            .map(|line| line.chars().collect())
            .collect()
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        let pipes = _parsed_input;
        (Self::find_loop(&pipes).len() / 2).to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        "0".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day10_part1_case1() {
        assert_eq!(
            Day10::solve_part_one(
                "-L|F7
7S-7|
L|7||
-L-J|
L|-JF"
            ),
            "4".to_string()
        )
    }

    #[test]
    fn check_day10_part1_case2() {
        assert_eq!(
            Day10::solve_part_one(
                "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ"
            ),
            "8".to_string()
        )
    }
}
