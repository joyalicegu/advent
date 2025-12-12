use crate::Solution;
use itertools::Itertools;

pub struct Day02;

impl Day02 {
    fn sum_invalid_ids(ranges: &Vec<(usize, usize)>) -> usize {
        // i threw away my notes oops
        // 123123 + 987987 = 1111110
        let mut result = 0;
        for (lo, hi) in ranges.iter() {
            // do this the dumb way first
            // generate them, and count them one at a time
            // println!("\nlo {:?} hi {:?}", lo, hi);
            let digits_low = (lo.to_string().len() / 2) * 2;
            let digits_high = (hi.to_string().len() / 2) * 2;
            for digits in digits_low..=digits_high {
                // println!("digits {:?}", digits);
                let multiplier = 10_u32.pow(digits as u32 / 2) as usize;
                let mut n = multiplier / 10;
                let mut nn = n * multiplier + n;
                while nn < *lo {
                    n += 1;
                    if n > multiplier - 1 {
                        break;
                    }
                    nn = n * multiplier + n;
                }
                while *lo <= nn && nn <= *hi {
                    // println!("{:?}", nn);
                    result += nn;
                    n += 1;
                    if n > multiplier - 1 {
                        break;
                    }
                    nn = n * multiplier + n;
                }
            }
        }
        result
    }
}

impl Solution for Day02 {
    type ParsedInput = Vec<(usize, usize)>;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        input_lines
            .lines()
            .nth(0)
            .unwrap()
            .split(",")
            .map(|l| {
                l.split("-")
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect_tuple()
                    .unwrap()
            })
            .collect()
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        Self::sum_invalid_ids(_parsed_input).to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        "0".to_string()
        // TODO
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn check_day02_part1_case1() {
        assert_eq!(Day02::solve_part_one(TEST_INPUT), "1227775554".to_string())
    }
}
