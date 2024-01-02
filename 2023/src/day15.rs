use crate::Solution;

pub struct Day15;

impl Day15 {
    fn run_hash_algorithm(step: &String) -> usize {
        step.as_bytes().iter().fold(0, |mut current_value, &c| {
            current_value += c as usize;
            current_value *= 17;
            current_value %= 256;
            current_value
        })
    }
}

impl Solution for Day15 {
    type ParsedInput = Vec<String>;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        input_lines
            .replace("\n", "")
            .split(",")
            .map(|s| s.to_string())
            .collect()
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        let steps = _parsed_input;
        steps
            .iter()
            .map(Self::run_hash_algorithm)
            .sum::<usize>()
            .to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        "0".to_string()
        // TODO
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn check_day15_part1_case1() {
        assert_eq!(Day15::solve_part_one(TEST_INPUT), "1320".to_string())
    }
}
