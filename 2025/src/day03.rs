use crate::Solution;

pub struct Day03;

impl Day03 {
    fn max_joltage(bank: &Vec<u64>, count: usize) -> u64 {
        // remove k digits
        let mut k = bank.len() - count;
        let mut stack = Vec::new();
        for i in 0..bank.len() {
            while k > 0 && !stack.is_empty() && *stack.last().unwrap() < bank[i] {
                stack.pop();
                k -= 1;
            }
            stack.push(bank[i]);
        }
        stack.iter().take(count).fold(0, |acc, x| acc * 10 + x)
    }

    fn total_output_joltage(banks: &Vec<Vec<u64>>, count: usize) -> u64 {
        let mut total = 0;
        for bank in banks.iter() {
            total += Self::max_joltage(bank, count);
        }
        total
    }
}

impl Solution for Day03 {
    type ParsedInput = Vec<Vec<u64>>;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        input_lines
            .lines()
            .map(|line| {
                line.chars()
                    .map(|ch| ch.to_digit(10).unwrap() as u64)
                    .collect()
            })
            .collect()
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        Self::total_output_joltage(&_parsed_input, 2).to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        Self::total_output_joltage(&_parsed_input, 12).to_string()
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

    #[test]
    fn check_day03_part2_case1() {
        assert_eq!(
            Day03::solve_part_two(TEST_INPUT),
            "3121910778619".to_string()
        )
    }

    #[test]
    fn check_day03_part2_case2() {
        assert_eq!(
            Day03::solve_part_two("1111111111111119911111111111111111"),
            "991111111111".to_string()
        )
    }
}
