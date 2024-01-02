use crate::Solution;

pub struct Day14;

impl Day14 {
    fn tilt_north(_platform: &Vec<Vec<char>>) -> Vec<Vec<char>> {
        let mut platform = _platform.clone();
        for i in 0..platform.len() {
            for j in 0..platform[i].len() {
                if platform[i][j] != 'O' {
                    continue;
                }
                for k in (1..=i).rev() {
                    if platform[k - 1][j] != '.' {
                        break;
                    }
                    platform[k][j] = '.';
                    platform[k - 1][j] = 'O';
                }
            }
        }
        platform
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

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        let platform = _parsed_input;
        let tilted = Self::tilt_north(platform);
        Self::calculate_load(&tilted).to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        "0".to_string()
        // TODO
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
}
