use crate::Solution;
use itertools::Itertools;
use std::collections::HashSet;

pub struct Day10;

#[derive(Copy, Clone)]
pub enum Direction {
    N,
    E,
    S,
    W,
}

#[derive(Copy, Clone)]
pub enum Turn {
    Left,
    Right,
}

impl Day10 {
    fn move_from(row: usize, col: usize, next_from: Direction) -> (usize, usize) {
        match next_from {
            Direction::N => (row + 1, col),
            Direction::E => (row, col.saturating_sub(1)),
            Direction::S => (row.saturating_sub(1), col),
            Direction::W => (row, col + 1),
        }
    }

    fn get_turn(from: Direction, next_from: Direction) -> Option<Turn> {
        match (from, next_from) {
            (Direction::N, Direction::N) => None,
            (Direction::S, Direction::S) => None,
            (Direction::E, Direction::E) => None,
            (Direction::W, Direction::W) => None,
            (Direction::N, Direction::W) => Some(Turn::Left),
            (Direction::N, Direction::E) => Some(Turn::Right),
            (Direction::E, Direction::N) => Some(Turn::Left),
            (Direction::E, Direction::S) => Some(Turn::Right),
            (Direction::S, Direction::E) => Some(Turn::Left),
            (Direction::S, Direction::W) => Some(Turn::Right),
            (Direction::W, Direction::S) => Some(Turn::Left),
            (Direction::W, Direction::N) => Some(Turn::Right),
            _ => panic!("Day 10: Invalid pair of directions"),
        }
    }

    fn get_next_from(pipe: char, from: Direction) -> Direction {
        match (pipe, from) {
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
        }
    }

    fn follow_pipe(
        pipes: &Vec<Vec<char>>,
        position: (usize, usize, Direction),
    ) -> (usize, usize, Direction) {
        let (row, col, from) = position;
        let pipe = pipes[row][col];
        let next_from = Self::get_next_from(pipe, from);
        let (next_row, next_col) = Self::move_from(row, col, next_from);
        (next_row, next_col, next_from)
    }

    fn is_valid_pipe(pipe: char, from: Direction) -> bool {
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

    fn from_start(
        pipes: &Vec<Vec<char>>,
        start: (usize, usize),
    ) -> ((usize, usize, Direction), (usize, usize, Direction)) {
        let (row, col) = start;
        [Direction::N, Direction::E, Direction::S, Direction::W]
            .into_iter()
            .filter_map(|next_from| {
                let (next_row, next_col) = Self::move_from(row, col, next_from);
                if let Some(row) = pipes.get(next_row) {
                    if let Some(pipe) = row.get(next_col) {
                        if Self::is_valid_pipe(*pipe, next_from) {
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
        let mut a = Self::from_start(&pipes, start).0;
        let mut positions = Vec::from([a]);
        while (a.0, a.1) != (start.0, start.1) {
            a = Self::follow_pipe(&pipes, a);
            positions.push(a);
        }
        positions
    }

    fn is_counterclockwise(main_loop: &Vec<(usize, usize, Direction)>) -> bool {
        let last_from = main_loop[main_loop.len() - 1].2;
        let ((lefts, rights), _) = main_loop.iter().map(|&(_, _, from)| from).fold(
            ((0, 0), last_from),
            |((lefts, rights), prev_from), from| match Self::get_turn(prev_from, from) {
                Some(Turn::Left) => ((lefts + 1, rights), from),
                Some(Turn::Right) => ((lefts, rights + 1), from),
                None => ((lefts, rights), from),
            },
        );
        lefts > rights
    }

    fn step_inward(r: usize, c: usize, direction: Direction, ccw: bool) -> (usize, usize) {
        match (direction, ccw) {
            (Direction::N, true) => Self::move_from(r, c, Direction::W),
            (Direction::E, true) => Self::move_from(r, c, Direction::N),
            (Direction::S, true) => Self::move_from(r, c, Direction::E),
            (Direction::W, true) => Self::move_from(r, c, Direction::S),
            (Direction::N, false) => Self::move_from(r, c, Direction::E),
            (Direction::E, false) => Self::move_from(r, c, Direction::S),
            (Direction::S, false) => Self::move_from(r, c, Direction::W),
            (Direction::W, false) => Self::move_from(r, c, Direction::N),
        }
    }

    fn count_enclosed_tiles(
        pipes: &Vec<Vec<char>>,
        main_loop: Vec<(usize, usize, Direction)>,
    ) -> usize {
        let ccw = Self::is_counterclockwise(&main_loop);
        let loop_locations = main_loop
            .iter()
            .map(|&(r, c, _)| (r, c))
            .collect::<HashSet<(usize, usize)>>();
        // find initial positions
        let mut candidates = Vec::new();
        for (i, &(r, c, from)) in main_loop.iter().enumerate() {
            let next_from = main_loop[(i + 1) % main_loop.len()].2;
            candidates.push(Self::step_inward(r, c, from, ccw));
            candidates.push(Self::step_inward(r, c, next_from, ccw));
        }
        candidates.dedup();
        candidates.retain(|&(r, c)| c < pipes[r].len() && !loop_locations.contains(&(r, c)));
        // fill from initial positions
        let mut enclosed_tiles = HashSet::new();
        while let Some((r, c)) = candidates.pop() {
            if loop_locations.contains(&(r, c)) || enclosed_tiles.contains(&(r, c)) {
                continue;
            }
            enclosed_tiles.insert((r, c));
            candidates.append(
                &mut [Direction::N, Direction::E, Direction::S, Direction::W]
                    .into_iter()
                    .map(|direction| Self::move_from(r, c, direction))
                    .filter(|&(r, c)| c < pipes[r].len())
                    .collect::<Vec<(usize, usize)>>(),
            );
        }
        enclosed_tiles.len()
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
        let pipes = _parsed_input;
        let main_loop = Self::find_loop(&pipes);
        Self::count_enclosed_tiles(&pipes, main_loop).to_string()
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

    #[test]
    fn check_day10_part2_case1() {
        assert_eq!(
            Day10::solve_part_two(
                "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."
            ),
            "4".to_string()
        )
    }

    #[test]
    fn check_day10_part2_case2() {
        assert_eq!(
            Day10::solve_part_two(
                "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
.........."
            ),
            "4".to_string()
        )
    }

    #[test]
    fn check_day10_part2_case3() {
        assert_eq!(
            Day10::solve_part_two(
                ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."
            ),
            "8".to_string()
        )
    }

    #[test]
    fn check_day10_part2_case4() {
        assert_eq!(
            Day10::solve_part_two(
                "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"
            ),
            "10".to_string()
        )
    }
}
