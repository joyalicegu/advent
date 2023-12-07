use crate::Solution;
use itertools::Itertools;
use std::iter::zip;

pub struct Day06;

pub struct Record {
    time: u64,
    distance: u64,
}

impl Day06 {
    fn ways_to_beat(record: &Record) -> u64 {
        // i don't hate it but it's not very nice
        let t = (record.time as f64
            - ((record.time * record.time).saturating_sub(4 * record.distance) as f64).sqrt())
            / 2.0;
        let (lo, mut hi) = (t.floor() as u64, t.ceil() as u64);
        if lo * (record.time - lo) == record.distance {
            hi += 1;
        }
        assert!(hi * (record.time - hi) > record.distance);
        assert!(lo * (record.time - lo) <= record.distance);
        record.time - 2 * hi + 1
    }
}

impl Solution for Day06 {
    type ParsedInput = (Vec<Record>, Record);

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        let (times, distances) = input_lines
            .split("\n")
            .map(|line| {
                line.split_once(": ")
                    .unwrap()
                    .1
                    .split_whitespace()
                    .map(|s| s.parse::<u64>().unwrap())
            })
            .next_tuple()
            .unwrap();
        let (time, distance) = input_lines
            .split("\n")
            .map(|line| {
                line.chars()
                    .filter(|c| c.to_digit(10).is_some())
                    .collect::<String>()
                    .parse::<u64>()
                    .unwrap()
            })
            .next_tuple()
            .unwrap();
        (
            zip(times, distances)
                .map(|(t, d)| Record {
                    time: t,
                    distance: d,
                })
                .collect(),
            Record {
                time: time,
                distance: distance,
            },
        )
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        let (records, _) = _parsed_input;
        records
            .iter()
            .map(Self::ways_to_beat)
            .product::<u64>()
            .to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        let (_, record) = _parsed_input;
        Self::ways_to_beat(&record).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn check_day06_part1_case1() {
        assert_eq!(Day06::solve_part_one(TEST_INPUT), "288".to_string())
    }

    #[test]
    fn check_day06_part2_case1() {
        assert_eq!(Day06::solve_part_two(TEST_INPUT), "71503".to_string())
    }
}
