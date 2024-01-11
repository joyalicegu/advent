use crate::Solution;
use itertools::Itertools;
use std::str::FromStr;

pub struct Day18;

#[derive(Copy, Clone)]
pub enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    fn offsets(&self) -> (isize, isize) {
        match self {
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Up => (-1, 0),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseDirectionError;

impl FromStr for Direction {
    type Err = ParseDirectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Direction::Right),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "U" => Ok(Direction::Up),
            "0" => Ok(Direction::Right),
            "1" => Ok(Direction::Down),
            "2" => Ok(Direction::Left),
            "3" => Ok(Direction::Up),
            _ => Err(ParseDirectionError),
        }
    }
}

impl Day18 {
    fn move_i(i: isize, d: Direction, point: (isize, isize)) -> (isize, isize) {
        let (r, c) = point;
        let (dr, dc) = d.offsets();
        (r + dr * i, c + dc * i)
    }

    fn polygon(plan: &Vec<(Direction, isize)>) -> Vec<(isize, isize)> {
        let mut polygon = Vec::new();
        let mut point = (0, 0);
        for &(d, i) in plan.iter() {
            point = Self::move_i(i, d, point);
            polygon.push(point);
        }
        assert!(point == (0, 0));
        polygon
    }

    fn boundary(plan: &Vec<(Direction, isize)>) -> isize {
        plan.iter().map(|&(_, i)| i).sum::<isize>()
    }

    fn area(polygon: &Vec<(isize, isize)>) -> isize {
        // https://duckduckgo.com/?q=polygon+area+formula+from+coordinates
        // https://en.wikipedia.org/wiki/Shoelace_formula
        (polygon
            .iter()
            .cycle()
            .tuple_windows()
            .take(polygon.len())
            .map(|((r, c), (nr, nc))| r * nc - nr * c)
            .sum::<isize>()
            / 2)
        .abs()
    }

    fn interior(area: isize, boundary: isize) -> isize {
        // https://duckduckgo.com/?q=polygon+integer+points
        // https://en.wikipedia.org/wiki/Pick's_theorem
        area - boundary / 2 + 1
    }

    fn volume(plan: &Vec<(Direction, isize)>) -> isize {
        let polygon = Self::polygon(plan);
        let boundary = Self::boundary(plan);
        let area = Self::area(&polygon);
        let interior = Self::interior(area, boundary);
        interior + boundary
    }
}

impl Solution for Day18 {
    type ParsedInput = (Vec<(Direction, isize)>, Vec<(Direction, isize)>);

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        (
            input_lines
                .lines()
                .map(|line| {
                    let (direction_str, meters_str, _) =
                        line.split_whitespace().collect_tuple().unwrap();
                    (
                        direction_str.parse::<Direction>().unwrap(),
                        meters_str.parse::<isize>().unwrap(),
                    )
                })
                .collect(),
            input_lines
                .lines()
                .map(|line| {
                    let hexcode = line.split(&['#', ')']).nth(1).unwrap();
                    (
                        hexcode[5..].parse::<Direction>().unwrap(),
                        isize::from_str_radix(&hexcode[..5], 16).unwrap(),
                    )
                })
                .collect(),
        )
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        let plan = &_parsed_input.0;
        Self::volume(&plan).to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        let plan = &_parsed_input.1;
        Self::volume(&plan).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn check_day18_part1_case1() {
        assert_eq!(Day18::solve_part_one(TEST_INPUT), "62".to_string())
    }

    #[test]
    fn check_day18_part2_case1() {
        assert_eq!(
            Day18::solve_part_two(TEST_INPUT),
            "952408144115".to_string()
        )
    }

    #[test]
    fn check_day18_interior_volume_case1() {
        /*
         * ### ###
         * # ### #
         * # ### #
         * ### ###
         */
        let plan = vec![
            (Direction::Right, 2),
            (Direction::Down, 1),
            (Direction::Right, 2),
            (Direction::Up, 1),
            (Direction::Right, 2),
            (Direction::Down, 3),
            (Direction::Left, 2),
            (Direction::Up, 1),
            (Direction::Left, 2),
            (Direction::Down, 1),
            (Direction::Left, 2),
            (Direction::Up, 3),
        ];
        let polygon = Day18::polygon(&plan);
        let boundary = Day18::boundary(&plan);
        let area = Day18::area(&polygon);
        let interior = Day18::interior(area, boundary);
        assert_eq!(interior, 4)
    }
}
