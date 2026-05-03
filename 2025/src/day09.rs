use crate::Solution;
use itertools::Itertools;
use std::cmp;

pub struct Day09;

impl Day09 {
    fn largest_area(corners: &Vec<(isize, isize)>) -> isize {
        corners
            .iter()
            .tuple_combinations()
            .map(|(a, b)| ((a.0 - b.0 + 1) * (a.1 - b.1 + 1)).abs())
            .max()
            .expect("corners.len() should be at least 2")
    }

    fn edges(polygon: &Vec<(isize, isize)>) -> Vec<((isize, isize), (isize, isize))> {
        // cribbed from area() from my 2023 day 18
        polygon
            .clone()
            .into_iter()
            .cycle()
            .tuple_windows()
            .take(polygon.len())
            .collect()
    }

    fn is_inside(
        a: (isize, isize),
        b: (isize, isize),
        boundary_edges: &Vec<((isize, isize), (isize, isize))>,
    ) -> bool {
        let (x_min, x_max) = (cmp::min(a.0, b.0), cmp::max(a.0, b.0));
        // bottom top
        let (y_min, y_max) = (cmp::min(a.1, b.1), cmp::max(a.1, b.1));
        for e in boundary_edges.iter() {
            let (e_x_min, e_x_max) = (cmp::min(e.0 .0, e.1 .0), cmp::max(e.0 .0, e.1 .0));
            let (e_y_min, e_y_max) = (cmp::min(e.0 .1, e.1 .1), cmp::max(e.0 .1, e.1 .1));
            if e_x_min == e_x_max {
                // boundary edge is vertical
                let e_x = e_x_min;
                if x_min < e_x && e_x < x_max && e_y_max > y_min && e_y_min < y_max {
                    return false;
                }
            } else if e_y_min == e_y_max {
                // boundary edge is horizontal
                let e_y = e_y_min;
                if y_min < e_y && e_y < y_max && e_x_max > x_min && e_x_min < x_max {
                    return false;
                }
            }
        }
        true
    }

    fn largest_area_in_polygon(corners: &Vec<(isize, isize)>) -> isize {
        let boundary_edges = Self::edges(corners);
        corners
            .iter()
            .tuple_combinations()
            .filter(|(a, b)| Self::is_inside(**a, **b, &boundary_edges))
            .map(|(a, b)| ((a.0 - b.0 + 1) * (a.1 - b.1 + 1)).abs())
            .max()
            .expect("corners.len() should be at least 2")
    }
}

impl Solution for Day09 {
    type ParsedInput = Vec<(isize, isize)>;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        input_lines
            .lines()
            .map(|line| {
                line.split(",")
                    .map(|s| s.parse::<isize>().unwrap())
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
