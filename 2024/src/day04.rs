use crate::Solution;
use itertools::iproduct;

pub struct Day04;

impl Day04 {
    fn word_search(grid: &Vec<Vec<char>>, word: String) -> usize {
        let (rows, cols) = (grid.len() as isize, grid[0].len() as isize);
        iproduct!(
            0..rows,
            0..cols,
            [
                (-1, 0),
                (-1, 1),
                (0, 1),
                (1, 1),
                (1, 0),
                (1, -1),
                (0, -1),
                (-1, -1),
            ]
            .into_iter()
        )
        .filter(|(ir, ic, (dr, dc))| {
            word.chars().enumerate().all(|(i, x)| {
                let r = ir + dr * (i as isize);
                let c = ic + dc * (i as isize);
                0 <= r && r < rows && 0 <= c && c < cols && grid[r as usize][c as usize] == x
            })
        })
        .count()
    }

    fn x_word_search(grid: &Vec<Vec<char>>, word: String) -> usize {
        let (rows, cols) = (grid.len() as isize, grid[0].len() as isize);
        let (ne, se, sw, nw) = ((-1, 1), (1, 1), (1, -1), (-1, -1));
        iproduct!(
            0..rows,
            0..cols,
            [(ne, se), (ne, nw), (sw, se), (sw, nw)].into_iter()
        )
        .filter(|(ir, ic, ((ur, uc), (vr, vc)))| {
            word.chars().enumerate().all(|(i, x)| {
                let j = (i as isize) - (word.len() / 2) as isize;
                let ur = ir + ur * j;
                let uc = ic + uc * j;
                let vr = ir + vr * j;
                let vc = ic + vc * j;
                (0 <= ur && ur < rows && 0 <= uc && uc < cols)
                    && grid[ur as usize][uc as usize] == x
                    && (0 <= vr && vr < rows && 0 <= vc && vc < cols)
                    && grid[vr as usize][vc as usize] == x
            })
        })
        .count()
    }
}

impl Solution for Day04 {
    type ParsedInput = Vec<Vec<char>>;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        input_lines
            .lines()
            .map(|l| l.chars().collect())
            .collect::<Vec<Vec<char>>>()
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        Self::word_search(_parsed_input, "XMAS".to_string()).to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        Self::x_word_search(_parsed_input, "MAS".to_string()).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn check_day04_part1_case1() {
        assert_eq!(Day04::solve_part_one(TEST_INPUT), "18".to_string())
    }

    #[test]
    fn check_day04_part2_case1() {
        assert_eq!(Day04::solve_part_two(TEST_INPUT), "9".to_string())
    }
}
