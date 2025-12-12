use crate::Solution;
use itertools::Itertools;

pub struct Day02;

impl Day02 {
    fn sum_invalid_ids(ranges: &Vec<(u32, u32)>) -> u32 {
        let mut result = 0;
        for (lo, hi) in ranges.iter() {
            // do this the dumb way first
            // generate them, and count them one at a time
            println!("\nlo {:?} hi {:?}", lo, hi);
            let digits_low = lo.to_string().len();
            let digits_high = hi.to_string().len();
            for digits in digits_low..=digits_high {
                for repeats in 2..=digits {
                    if digits % repeats != 0 {
                        continue;
                    }
                    println!("digits {:?} repeats {:?}", digits, repeats);
                    let multiplier = 10_u32.pow((digits / repeats) as u32);
                    let mut n = multiplier / 10;
                    let mut nn = (0..repeats).map(|p| n * multiplier.pow(p as u32)).sum();
                    while nn < *lo {
                        n += 1;
                        if n > multiplier - 1 {
                            break;
                        }
                        nn = (0..repeats).map(|p| n * multiplier.pow(p as u32)).sum();
                    }
                    while *lo <= nn && nn <= *hi {
                        // println!("{:?}", nn);
                        result += nn;
                        n += 1;
                        if n > multiplier - 1 {
                            break;
                        }
                        nn = (0..repeats).map(|p| n * multiplier.pow(p as u32)).sum();
                    }
                }
            }
        }
        result
    }
}

impl Solution for Day02 {
    type ParsedInput = Vec<(u32, u32)>;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        input_lines
            .lines()
            .nth(0)
            .unwrap()
            .split(",")
            .map(|l| {
                l.split("-")
                    .map(|n| n.parse::<u32>().unwrap())
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
