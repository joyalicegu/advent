use crate::Solution;
use itertools::iproduct;
use regex::Regex;
use std::cmp;
use std::collections::HashMap;

pub struct Day03;

#[derive(Eq, Hash, PartialEq)]
pub struct Symbol {
    value: char,
    row: usize,
    col: usize,
}

pub struct PartNumber {
    value: u32,
    row: usize,
    col: usize,
    len: usize,
}

impl Day03 {
    fn neighbors(number: &PartNumber, schematic: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
        let (row, col, len) = (number.row, number.col, number.len);
        let (rows, cols) = (schematic.len(), schematic[0].len());
        let rmax = cmp::min(row + 1, rows.saturating_sub(1));
        let cmax = cmp::min(col + len, cols.saturating_sub(1));
        iproduct!(row.saturating_sub(1)..=rmax, col.saturating_sub(1)..=cmax)
            .filter(|(r, c)| !(*r == row && col <= *c && *c < col + len))
            .collect()
    }

    fn symbols(number: &PartNumber, schematic: &Vec<Vec<char>>) -> Vec<Symbol> {
        Self::neighbors(number, schematic)
            .iter()
            .map(|(r, c)| (r, c, schematic[*r][*c]))
            .filter(|(_, _, v)| *v != '.' && !v.is_digit(10))
            .map(|(r, c, v)| Symbol {
                value: v,
                row: *r,
                col: *c,
            })
            .collect()
    }
}

impl Solution for Day03 {
    type ParsedInput = (Vec<PartNumber>, Vec<u32>);

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        // could make this nicer
        let schematic: Vec<Vec<char>> = input_lines
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        let mut numbers = Vec::new();
        let mut gears = HashMap::<Symbol, Vec<u32>>::new();
        let number_re = Regex::new(r"[0-9]+").unwrap();
        for (r, line) in input_lines.lines().enumerate() {
            for m in number_re.find_iter(line) {
                let value = m.as_str().parse::<u32>().unwrap();
                let number = PartNumber {
                    value: value,
                    row: r,
                    col: m.start(),
                    len: m.end() - m.start(),
                };
                let symbols = Self::symbols(&number, &schematic);
                if symbols.len() > 0 {
                    numbers.push(number);
                }
                for symbol in symbols {
                    if symbol.value != '*' {
                        continue;
                    }
                    if let Some(adjacent_numbers) = gears.get_mut(&symbol) {
                        adjacent_numbers.push(value);
                    } else {
                        gears.insert(symbol, Vec::from([value]));
                    }
                }
            }
        }
        let gear_ratios = gears
            .iter()
            .filter(|(_, v)| v.len() == 2)
            .map(|(_, v)| v.iter().product())
            .collect();
        (numbers, gear_ratios)
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        let (numbers, _) = _parsed_input;
        numbers.iter().map(|n| n.value).sum::<u32>().to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        let (_, gear_ratios) = _parsed_input;
        gear_ratios.iter().sum::<u32>().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn check_day03_part1_case1() {
        assert_eq!(Day03::solve_part_one(TEST_INPUT), "4361".to_string())
    }

    #[test]
    fn check_day03_part2_case1() {
        assert_eq!(Day03::solve_part_two(TEST_INPUT), "467835".to_string())
    }
}
