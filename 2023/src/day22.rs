use crate::Solution;
use itertools::iproduct;
use itertools::Itertools;
use std::cmp;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

pub struct Day22;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Coordinate {
    x: isize,
    y: isize,
    z: isize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Brick(Coordinate, Coordinate);

#[derive(Debug)]
pub struct ParseError;

impl FromStr for Coordinate {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x_str, y_str, z_str) = s.split(',').collect_tuple().ok_or(ParseError)?;
        let x = x_str.parse::<isize>().map_err(|_| ParseError)?;
        let y = y_str.parse::<isize>().map_err(|_| ParseError)?;
        let z = z_str.parse::<isize>().map_err(|_| ParseError)?;
        Ok(Coordinate { x: x, y: y, z: z })
    }
}

impl FromStr for Brick {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split_once("~").ok_or(ParseError)?;
        let a = split.0.parse::<Coordinate>().map_err(|_| ParseError)?;
        let b = split.1.parse::<Coordinate>().map_err(|_| ParseError)?;
        Ok(Brick(a, b))
    }
}

impl Day22 {
    fn supported_by(bricks: &Vec<Brick>) -> HashMap<Brick, HashSet<Brick>> {
        // map from (x, y) to (highest z, brick at that coordinate)
        let mut height_at = HashMap::<(isize, isize), (isize, Brick)>::new();
        // map from brick (after falling) to the bricks that support it
        let mut supported_by = HashMap::<Brick, HashSet<Brick>>::new();
        // iterate over bricks sorted by min-z coordinate
        for mut brick in bricks
            .clone()
            .into_iter()
            .sorted_by_key(|b| cmp::min(b.0.z, b.1.z))
        {
            let max_x = cmp::max(brick.0.x, brick.1.x);
            let max_y = cmp::max(brick.0.y, brick.1.y);
            let min_x = cmp::min(brick.0.x, brick.1.x);
            let min_y = cmp::min(brick.0.y, brick.1.y);
            let mut max_h = iproduct!(min_x..=max_x, min_y..=max_y)
                .filter_map(|coords| height_at.get(&coords))
                .map(|&(h, _)| h)
                .max()
                .unwrap_or(0);
            let supporting_bricks = iproduct!(min_x..=max_x, min_y..=max_y)
                .filter_map(|coords| height_at.get(&coords))
                .filter(|&(h, _)| *h == max_h)
                .map(|&(_, b)| b)
                .collect::<HashSet<Brick>>();
            let min_z = cmp::min(brick.0.z, brick.1.z);
            let shift = min_z - max_h - 1;
            if shift > 0 {
                brick.0.z -= shift;
                brick.1.z -= shift;
            }
            supported_by.insert(brick, supporting_bricks);
            max_h = cmp::max(brick.0.z, brick.1.z);
            for x in min_x..=max_x {
                for y in min_y..=max_y {
                    height_at
                        .entry((x, y))
                        .and_modify(|e| {
                            if max_h > e.0 {
                                *e = (max_h, brick)
                            }
                        })
                        .or_insert((max_h, brick));
                }
            }
        }
        supported_by
    }
}

impl Solution for Day22 {
    type ParsedInput = Vec<Brick>;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        input_lines
            .lines()
            .map(|s| s.parse::<Brick>().unwrap())
            .collect()
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        let bricks = _parsed_input;
        let supported_by = Self::supported_by(&bricks);
        let sole_supports = supported_by
            .values()
            .filter_map(|v| {
                if v.len() == 1 {
                    Some(v.iter().next().unwrap().clone())
                } else {
                    None
                }
            })
            .collect::<HashSet<Brick>>();
        let result = bricks.len() - sole_supports.len();
        result.to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        let bricks = _parsed_input;
        let supported_by = Self::supported_by(&bricks);
        let mut graph = HashMap::new();
        for (&k, vs) in supported_by.iter() {
            for v in vs.iter() {
                let entry = graph.entry(v).or_insert(HashSet::new());
                entry.insert(k);
            }
        }
        let mut result = 0;
        for (&support, directs) in graph.iter() {
            let mut fallen = HashSet::from([support]);
            let mut stack = Vec::from_iter(directs);
            while let Some(brick) = stack.pop() {
                let Some(supports) = supported_by.get(brick) else {
                    continue;
                };
                if supports.iter().all(|b| fallen.contains(b)) {
                    fallen.insert(brick);
                    if let Some(directs) = graph.get(brick) {
                        stack.extend(directs);
                    }
                }
            }
            result += fallen.len() - 1;
        }
        result.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

    #[test]
    fn check_day22_part1_case1() {
        assert_eq!(Day22::solve_part_one(TEST_INPUT), "5".to_string())
    }

    #[test]
    fn check_day22_part2_case1() {
        assert_eq!(Day22::solve_part_two(TEST_INPUT), "7".to_string())
    }
}
