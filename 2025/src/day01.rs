use crate::Solution;

pub struct Day01;

impl Day01 {
    fn follow_rotations(rotations: &Vec<i32>) -> usize {
        let mut dial = 50;
        let mut count = 0;
        for rotation in rotations.iter() {
            dial += rotation;
            while dial < 0 {
                dial += 100;
            }
            while dial > 99 {
                dial -= 100;
            }
            if dial == 0 {
                count += 1;
            }
        }
        count
    }
}

impl Solution for Day01 {
    type ParsedInput = Vec<i32>;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        input_lines
            .lines()
            .map(|r| {
                let d = r[1..].parse::<i32>().unwrap();
                if r.starts_with('L') {
                    -d
                } else {
                    d
                }
            })
            .collect()
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        let rotations = _parsed_input;
        Self::follow_rotations(rotations).to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        "0".to_string()
        // TODO
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn check_day01_part1_case1() {
        assert_eq!(Day01::solve_part_one(TEST_INPUT), "3".to_string())
    }
}
