use crate::Solution;
use itertools::Itertools;

pub struct Day24;

#[derive(Debug, Clone, Copy)]
pub struct Hailstone {
    position: (isize, isize, isize),
    velocity: (isize, isize, isize),
}

impl Day24 {
    fn paths_intersect_2d(
        a: Hailstone,
        b: Hailstone,
        test_area: (isize, isize, isize, isize),
    ) -> bool {
        /* don't look

        p.x + v.x * t = q.x + w.x * s = x
        p.y + v.y * t = q.y + w.y * s = y

        t = (q.x + w.x * s - p.x) / v.x = (x - p.x) / v.x
        t = (q.y + w.y * s - p.y) / v.y = (y - p.y) / v.y

        s = (p.x + v.x * t - q.x) / w.x = (x - q.x) / w.x
        s = (p.y + v.y * t - q.y) / w.y = (y - q.y) / w.y

        (x - p.x) / v.x = (y - p.y) / v.y
        (x - q.x) / w.x = (y - q.y) / w.y

        x = p.x + v.x * (y - p.y) / v.y
        x = q.x + w.x * (y - q.y) / w.y

        y = p.y + v.y * (x - p.x) / v.x
        y = q.y + w.y * (x - q.x) / w.x

        p.x + v.x * (y - p.y) / v.y = q.x + w.x * (y - q.y) / w.y
        p.y + v.y * (x - p.x) / v.x = q.y + w.y * (x - q.x) / w.x

          p.x + (v.x / v.y) * y - v.x * p.y / v.y
        = q.x + (w.x / w.y) * y - w.x * q.y / w.y

          p.y + (v.y / v.x) * x - v.y * p.x / v.x
        = q.y + (w.y / w.x) * x - w.y * q.x / w.x

        x = (((v.y / v.x) * p.x) + q.y - ((w.y / w.x) * q.x) - p.y)
          / ((v.y / v.x) - (w.y / w.x))
        y = (((v.x / v.y) * p.y) + q.x - ((w.x / w.y) * q.y) - p.x)
          / ((v.x / v.y) - (w.x / w.y))
        */

        let (min_x, max_x, min_y, max_y) = test_area;
        let (p, v, q, w) = (a.position, a.velocity, b.position, b.velocity);
        // parallel
        if v.0 * w.1 == v.1 * w.0 {
            return false;
        }
        let (vx, vy) = (v.0 as f64, v.1 as f64);
        let (wx, wy) = (w.0 as f64, w.1 as f64);
        let (px, py) = (p.0 as f64, p.1 as f64);
        let (qx, qy) = (q.0 as f64, q.1 as f64);
        let x = (((vy / vx) * px) + qy - ((wy / wx) * qx) - py) / ((vy / vx) - (wy / wx));
        let y = (((vx / vy) * py) + qx - ((wx / wy) * qy) - px) / ((vx / vy) - (wx / wy));

        // in the past
        let t = (x - px) / vx;
        let s = (x - qx) / wx;
        if t < 0.0 || s < 0.0 {
            return false;
        }

        // in the test area
        return min_x as f64 <= x && x <= max_x as f64 && min_y as f64 <= y && y <= max_y as f64;
    }

    fn intersections_2d(
        hailstones: &Vec<Hailstone>,
        test_area: (isize, isize, isize, isize),
    ) -> usize {
        let mut count = 0;
        for i in 0..hailstones.len() {
            for j in (i + 1)..hailstones.len() {
                if Self::paths_intersect_2d(hailstones[i], hailstones[j], test_area) {
                    count += 1;
                }
            }
        }
        count
    }
}

impl Solution for Day24 {
    type ParsedInput = Vec<Hailstone>;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        input_lines
            .lines()
            .map(|l| {
                let (p_str, v_str) = l.split_once(" @ ").unwrap();
                Hailstone {
                    position: p_str
                        .split(", ")
                        .map(|s| s.trim().parse::<isize>().unwrap())
                        .collect_tuple()
                        .unwrap(),
                    velocity: v_str
                        .split(", ")
                        .map(|s| s.trim().parse::<isize>().unwrap())
                        .collect_tuple()
                        .unwrap(),
                }
            })
            .collect()
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        let hailstones = _parsed_input;
        let test_area = (
            200000000000000,
            400000000000000,
            200000000000000,
            400000000000000,
        );
        Self::intersections_2d(&hailstones, test_area).to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        "0".to_string()
        // TODO
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";

    #[test]
    fn check_day24_part1_case1() {
        let hailstones = Day24::parse_input(TEST_INPUT);
        assert_eq!(Day24::intersections_2d(&hailstones, (7, 27, 7, 27)), 2)
    }
}
