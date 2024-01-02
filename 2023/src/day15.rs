use crate::Solution;

pub struct Day15;

#[derive(Clone)]
pub struct Lens {
    label: String,
    focal_length: usize,
}

impl Day15 {
    fn run_hash_algorithm(step: &String) -> usize {
        step.as_bytes().iter().fold(0, |mut current_value, &c| {
            current_value += c as usize;
            current_value *= 17;
            current_value %= 256;
            current_value
        })
    }

    fn parse_step(step: &String) -> (String, Option<usize>) {
        if let Some((label, number)) = step.split_once('=') {
            (label.to_string(), Some(number.parse::<usize>().unwrap()))
        } else {
            let (label, _) = step.split_once('-').unwrap();
            (label.to_string(), None)
        }
    }

    fn boxes(steps: &Vec<String>) -> Vec<Vec<Lens>> {
        let mut boxes = vec![vec![]; 256];
        for step in steps.iter() {
            let (label, operation) = Self::parse_step(step);
            let box_number = Self::run_hash_algorithm(&label);
            let lens_index = boxes[box_number]
                .iter()
                .position(|l: &Lens| l.label == label);
            if let Some(focal_length) = operation {
                let lens = Lens {
                    label: label,
                    focal_length: focal_length,
                };
                if let Some(i) = lens_index {
                    boxes[box_number][i] = lens;
                } else {
                    boxes[box_number].push(lens);
                }
            } else {
                if let Some(i) = lens_index {
                    boxes[box_number].remove(i);
                }
            }
        }
        boxes
    }

    fn total_focusing_power(boxes: Vec<Vec<Lens>>) -> usize {
        let mut result = 0;
        for (i, lenses) in boxes.iter().enumerate() {
            for (j, lens) in lenses.iter().enumerate() {
                result += (i + 1) * (j + 1) * lens.focal_length;
            }
        }
        result
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
        let steps = _parsed_input;
        Self::total_focusing_power(Self::boxes(steps)).to_string()
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
