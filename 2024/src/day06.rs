use crate::Solution;

pub struct Day06;

impl Day06 {
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

    // TODO helpers
}

impl Solution for Day06 {
    type ParsedInput = Vec<Vec<char>>;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        input_lines
            .lines()
            .map(|l| l.chars().collect())
            .collect::<Vec<Vec<char>>>()
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        let grid = _parsed_input;
        Self::count_guard_positions(
        "0".to_string()
        // TODO
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        "0".to_string()
        // TODO
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn check_day06_part1_case1() {
        assert_eq!(Day06::solve_part_one(TEST_INPUT), "41".to_string())
    }
}
