use crate::Solution;

pub struct Day03;

impl Day03 {
    fn total_output_joltage(banks: &Vec<Vec<u32>>) -> u32 {
        let mut total = 0;
        for bank in banks.iter() {
            let mut max_joltage = 0;
            for i in 0..(bank.len() - 1) {
                for j in (i + 1)..bank.len() {
                    let joltage = bank[i] * 10 + bank[j];
                    if joltage > max_joltage {
                        max_joltage = joltage;
                    }
                }
            }
            total += max_joltage;
        }
        total
    }
}

impl Solution for Day03 {
    type ParsedInput = Vec<Vec<u32>>;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        input_lines
            .lines()
            .map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap()).collect())
            .collect()
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        Self::total_output_joltage(&_parsed_input).to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        "0".to_string()
        // TODO
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn check_day03_part1_case1() {
        assert_eq!(Day03::solve_part_one(TEST_INPUT), "357".to_string())
    }
}
