use crate::Solution;
use itertools::sorted;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub struct Day09;

impl Day09 {
    fn largest_area(corners: &Vec<(usize, usize)>) -> usize {
        corners
            .iter()
            .tuple_combinations()
            .map(|(ua, ub)| {
                let a = (ua.0 as isize, ua.1 as isize);
                let b = (ub.0 as isize, ub.1 as isize);
                ((a.0 - b.0 + 1) * (a.1 - b.1 + 1)).abs()
            })
            .max()
            .expect("corners.len() should be at least 2") as usize
    }

    fn is_inside(
        ca: (usize, usize),
        cb: (usize, usize),
        compressed_tiles: &Vec<Vec<char>>,
    ) -> bool {
        false
        // TODO is inside: and then just brute force check all tiles in each rectangle (compressed)
    }

    fn largest_area_in_polygon(corners: &Vec<(usize, usize)>) -> isize {
        let xs: HashSet<usize> = corners.iter().map(|(x, _)| *x).collect();
        let ys: HashSet<usize> = corners.iter().map(|(_, y)| *y).collect();
        let col_to_x: Vec<usize> = sorted(xs.into_iter()).collect();
        let row_to_y: Vec<usize> = sorted(ys.into_iter()).collect();
        let x_to_col: HashMap<usize, usize> = col_to_x
            .iter()
            .enumerate()
            .map(|(i, c)| (*c, i as usize))
            .collect();
        let y_to_row: HashMap<usize, usize> = row_to_y
            .iter()
            .enumerate()
            .map(|(i, c)| (*c, i as usize))
            .collect();
        let compressed_corners: Vec<(usize, usize)> = corners
            .iter()
            .map(|(x, y)| (*y_to_row.get(y).unwrap(), *x_to_col.get(x).unwrap()))
            .collect();
        let (cols, rows) = (x_to_col.len(), y_to_row.len());
        let mut compressed_tiles = vec![vec![' '; cols]; rows];
        // color tiles: corners and edges
        let segments: Vec<_> = compressed_corners
            .clone()
            .into_iter()
            .skip(1)
            .zip(compressed_corners.clone().into_iter())
            .collect();
        for &((r, c), (nr, nc)) in segments.iter() {
            compressed_tiles[r][c] = '#';
            compressed_tiles[nr][nc] = '#';
            if r < nr {
                for ir in (r + 1)..nr {
                    compressed_tiles[ir][c] = 'X';
                }
            } else if nr < r {
                for ir in (nr + 1)..r {
                    compressed_tiles[ir][c] = 'X';
                }
            }
            if c < nc {
                for ic in (c + 1)..nc {
                    compressed_tiles[r][ic] = 'X';
                }
            } else if nc < c {
                for ic in (nc + 1)..c {
                    compressed_tiles[r][ic] = 'X';
                }
            }
        }
        // color tiles: flood fill with Xs
        // TODO actually flood fill
        // debug: print the outline
        for ir in 0..rows {
            for ic in 0..cols {
                print!("{}", compressed_tiles[ir][ic]);
            }
            println!("");
        }
        // TODO implement the inside check
        // TODO map to real coordinates to get area
        corners
            .iter()
            .tuple_combinations()
            .filter(|(a, b)| Self::is_inside(**a, **b, &compressed_tiles))
            .map(|(ua, ub)| {
                let a = (ua.0 as isize, ua.1 as isize);
                let b = (ub.0 as isize, ub.1 as isize);
                ((a.0 - b.0 + 1) * (a.1 - b.1 + 1)).abs()
            })
            .max()
            .expect("corners.len() should be at least 2")
    }
}

impl Solution for Day09 {
    type ParsedInput = Vec<(usize, usize)>;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        input_lines
            .lines()
            .map(|line| {
                line.split(",")
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect_tuple()
                    .unwrap()
            })
            .collect()
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        let corners = _parsed_input;
        Self::largest_area(corners).to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        let corners = _parsed_input;
        Self::largest_area_in_polygon(corners).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn check_day09_part1_case1() {
        assert_eq!(Day09::solve_part_one(TEST_INPUT), "50".to_string())
    }

    #[test]
    fn check_day09_part2_case1() {
        assert_eq!(Day09::solve_part_two(TEST_INPUT), "24".to_string())
    }
}
