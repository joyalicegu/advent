use crate::Solution;
use itertools::Itertools;
use std::cmp;

pub struct Day05;

type Range = (u32, u32);

pub struct Almanac {
    seeds: Vec<u32>,
    seed_ranges: Vec<Range>,
    maps: Vec<Vec<(u32, u32, u32)>>,
}

impl Day05 {
    fn apply_map(map: &Vec<(u32, u32, u32)>, range: Range) -> Vec<Range> {
        let (mut start, mut seeds) = range;
        let mut i = match map.binary_search_by_key(&start, |t| t.1) {
            Ok(i) => i,
            Err(i) => i.saturating_sub(1),
        };
        let mut result = Vec::new();
        loop {
            if i >= map.len() {
                result.push((start, seeds));
                break;
            }
            let (dst, src, len) = map[i];
            let mapped_seeds;
            if start < src {
                mapped_seeds = cmp::min(src - start, seeds);
                result.push((start, mapped_seeds));
            } else if start - src < len {
                mapped_seeds = cmp::min(len - (start - src), seeds);
                result.push(((start - src) + dst, mapped_seeds));
                i += 1;
            } else {
                // assume non-overlapping ranges
                result.push((start, seeds));
                break;
            }
            if seeds == mapped_seeds {
                break;
            }
            start += mapped_seeds;
            seeds -= mapped_seeds;
        }
        result
    }

    fn apply_maps(range: Range, maps: &Vec<Vec<(u32, u32, u32)>>) -> Vec<Range> {
        maps.iter()
            .fold(Vec::from([range]), |ranges: Vec<Range>, map| {
                ranges
                    .iter()
                    .map(|&r| Self::apply_map(map, r))
                    .flatten()
                    .collect()
            })
    }
}

impl Solution for Day05 {
    type ParsedInput = Almanac;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        let (line, lines) = input_lines.split_once("\n\n").unwrap();
        let seeds: Vec<u32> = line
            .split_once(": ")
            .unwrap()
            .1
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        let seed_ranges: Vec<Range> = seeds.clone().into_iter().tuples().collect();
        let mut maps = Vec::new();
        for p in lines.split("\n\n").map(|p| p.split_once("\n").unwrap().1) {
            let mut map = Vec::new();
            for l in p.lines() {
                let (dst, src, len) = l
                    .split_whitespace()
                    .map(|s| s.parse::<u32>().unwrap())
                    .collect_tuple()
                    .unwrap();
                map.push((dst, src, len));
            }
            map.sort_by_key(|t| t.1); // sort for binary search
            maps.push(map);
        }
        Almanac {
            seeds: seeds,
            seed_ranges: seed_ranges,
            maps: maps,
        }
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        let almanac = _parsed_input;
        let apply = |&seed| Self::apply_maps((seed, 1), &almanac.maps)[0].0;
        almanac.seeds.iter().map(apply).min().unwrap().to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        let almanac = _parsed_input;
        let apply = |&r| Self::apply_maps(r, &almanac.maps);
        almanac
            .seed_ranges
            .iter()
            .map(apply)
            .flatten()
            .min()
            .unwrap()
            .0
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn check_day05_part1_case1() {
        assert_eq!(Day05::solve_part_one(TEST_INPUT), "35".to_string())
    }

    #[test]
    fn check_day05_part2_case1() {
        assert_eq!(Day05::solve_part_two(TEST_INPUT), "46".to_string())
    }
}
