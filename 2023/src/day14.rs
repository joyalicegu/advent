use crate::Solution;
use itertools::Either;
use std::collections::HashMap;

pub struct Day14;

impl Day14 {
    fn tilt(platform: &mut Vec<Vec<char>>, dr: isize, dc: isize) -> () {
        let (rows, cols) = (platform.len(), platform[0].len());
        for r in if dr < 0 {
            Either::Left(0..rows)
        } else {
            Either::Right((0..rows).rev())
        } {
            for c in if dc < 0 {
                Either::Left(0..cols)
            } else {
                Either::Right((0..cols).rev())
            } {
                if platform[r][c] != 'O' {
                    continue;
                }
                let (mut cr, mut cc) = (r, c);
                loop {
                    if (cr == 0 && dr < 0)
                        || (cc == 0 && dc < 0)
                        || (cr == rows - 1 && dr > 0)
                        || (cc == cols - 1 && dc > 0)
                    {
                        break;
                    }
                    let nr = (cr as isize + dr) as usize;
                    let nc = (cc as isize + dc) as usize;
                    if platform[nr][nc] != '.' {
                        break;
                    }
                    platform[cr][cc] = '.';
                    platform[nr][nc] = 'O';
                    (cr, cc) = (nr, nc);
                }
            }
        }
    }

    fn tilt_north(platform: &mut Vec<Vec<char>>) -> () {
        Self::tilt(platform, -1, 0)
    }

    fn spin_cycle(platform: &mut Vec<Vec<char>>) -> () {
        Self::tilt(platform, -1, 0); // N
        Self::tilt(platform, 0, -1); // W
        Self::tilt(platform, 1, 0); // S
        Self::tilt(platform, 0, 1); // E
    }

    fn spin_cycles(platform: &mut Vec<Vec<char>>, cycles: usize) -> () {
        let mut map = HashMap::<Vec<Vec<char>>, usize>::new();
        let mut cycle = 0;
        while cycle < cycles {
            if let Some(last_cycle) = map.get(platform) {
                let remaining = (cycles - cycle) % (cycle - last_cycle);
                cycle = cycles - remaining; // skip ahead
                break;
            } else {
                map.insert(platform.clone(), cycle);
                Self::spin_cycle(platform);
                cycle += 1;
            }
        }
        while cycle < cycles {
            Self::spin_cycle(platform);
            cycle += 1;
        }
    }

    fn calculate_load(platform: &Vec<Vec<char>>) -> usize {
        platform
            .iter()
            .map(|row| row.iter().filter(|&c| *c == 'O').count())
            .enumerate()
            .map(|(i, count)| count * (platform.len() - i))
            .sum::<usize>()
    }
}

impl Solution for Day14 {
    type ParsedInput = Vec<Vec<char>>;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        input_lines
            .lines()
            .map(|l| l.chars().collect())
            .collect::<Vec<Vec<char>>>()
    }

    // TODO
    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        let platform = &mut _parsed_input.clone();
        Self::tilt_north(platform);
        Self::calculate_load(platform).to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        let platform = &mut _parsed_input.clone();
        Self::spin_cycles(platform, 1000000000);
        Self::calculate_load(platform).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn check_day14_part1_case1() {
        assert_eq!(Day14::solve_part_one(TEST_INPUT), "136".to_string())
    }

    #[test]
    fn check_day14_part2_case1() {
        assert_eq!(Day14::solve_part_two(TEST_INPUT), "64".to_string())
    }
}
