use crate::Solution;
use itertools::Itertools;
use nalgebra::{Matrix3, Vector3};

pub struct Day24;

#[derive(Debug, Clone, Copy)]
pub struct Hailstone {
    position: (i128, i128, i128),
    velocity: (i128, i128, i128),
}

impl Day24 {
    fn paths_intersect_2d(a: Hailstone, b: Hailstone, test_area: (i128, i128, i128, i128)) -> bool {
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

    fn intersections_2d(hailstones: &Vec<Hailstone>, test_area: (i128, i128, i128, i128)) -> usize {
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

    fn exterior3(u: (i128, i128, i128), v: (i128, i128, i128), w: (i128, i128, i128)) -> i128 {
        // like the scalar triple product
        (u.0 * v.1 * w.2) + (u.1 * v.2 * w.0) + (u.2 * v.0 * w.1)
            - (u.0 * v.2 * w.1)
            - (u.1 * v.0 * w.2)
            - (u.2 * v.1 * w.0)
    }

    fn exterior2(u: (i128, i128, i128), v: (i128, i128, i128)) -> (i128, i128, i128) {
        // like the cross product
        (
            u.0 * v.1 - u.1 * v.0,
            u.1 * v.2 - u.2 * v.1,
            u.2 * v.0 - u.0 * v.2,
        )
    }

    fn sub(u: (i128, i128, i128), v: (i128, i128, i128)) -> (i128, i128, i128) {
        (u.0 - v.0, u.1 - v.1, u.2 - v.2)
    }

    fn calculate_rock_position(hailstones: &Vec<Hailstone>) -> (i128, i128, i128) {
        // i gave up on this one, so this is not my solution:
        // https://www.reddit.com/r/adventofcode/comments/18qgf10/2023_day_24_part_2_solved_as_a_3x3_linear_system/
        //
        // where Pi and Vi are the position and velocity of hailstone i
        // and P and V are the position and velocity of the rock
        // Pi + Vi * ti = P + V * ti
        // =>
        // (Pi - P) - (V - Vi) * ti = 0
        //
        // (Pi - P) and (V - Vi) are vectors, and for (Pi - P) and (V - Vi)
        // to equal a scalar product of one another, they must be parallel,
        // and wedge products of parallel vectors are zero, so:
        // (Pi - P) ^ (V - Vi) = 0
        //
        // multiply it out
        // (Pi ^ V) - (P ^ V) - (Pi ^ Vi) + (P ^ Vi) = 0
        //
        // remove (P ^ V) by subtracting with the equation for hailstone j
        // ((Pi - Pj) ^ V) - (Pi ^ Vi) + (Pj ^ Vj) + (P ^ (Vi - Vj)) = 0
        //
        // get rid of ((Pi - Pj) ^ V) (because we don't care about the velocity)
        // since u ^ v ^ u = 0 (since u is parallel to itself)
        // =>
        // (P ^ (Vi - Vj) ^ (Pi - Pj)) + Pi ^ Vi ^ Pj + Pj ^ Vj ^ Pi = 0
        //
        // 3 unknowns (the 3 dimensions of P) so we need 3 pairs
        let mut equations = Vec::new();
        for pair in (0..3).into_iter().combinations(2) {
            let &[i, j] = pair.as_slice() else {
                continue;
            };
            let (pi, vi, pj, vj) = (
                hailstones[i].position,
                hailstones[i].velocity,
                hailstones[j].position,
                hailstones[j].velocity,
            );
            let (a, b, c) = Self::exterior2(Self::sub(vi, vj), Self::sub(pi, pj));
            let d = -(Self::exterior3(pi, vi, pj) + Self::exterior3(pj, vj, pi));
            let equation = (a, b, c, d); // ax + by + cz = d
            println!(
                "{:?}x + {:?}y + {:?}z = {:?}",
                equation.0, equation.1, equation.2, equation.3
            );
            equations.push(equation);
        }
        // then i actually just plugged these into wolfram alpha:
        // https://www.wolframalpha.com/input?i=system+equation+calculator

        // solve system of equations
        let abc = Matrix3::new(
            equations[0].0 as f64,
            equations[0].1 as f64,
            equations[0].2 as f64,
            equations[1].0 as f64,
            equations[1].1 as f64,
            equations[1].2 as f64,
            equations[2].0 as f64,
            equations[2].1 as f64,
            equations[2].2 as f64,
        );
        let d = Vector3::new(
            equations[0].3 as f64,
            equations[1].3 as f64,
            equations[2].3 as f64,
        );
        let p: Vector3<f64> = abc.qr().solve(&d).unwrap();
        println!("x={:?} y={:?} z={:?} (as f64s)", p.x, p.y, p.z);
        // despite floating point error, we still get the right answer
        let (x, y, z) = (
            p.x.round() as i128,
            p.y.round() as i128,
            p.z.round() as i128,
        );
        println!("x={:?} y={:?} z={:?}", x, y, z);
        (x, y, z)
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
                        .map(|s| s.trim().parse::<i128>().unwrap())
                        .collect_tuple()
                        .unwrap(),
                    velocity: v_str
                        .split(", ")
                        .map(|s| s.trim().parse::<i128>().unwrap())
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
        let hailstones = _parsed_input;
        let (x, y, z) = Self::calculate_rock_position(&hailstones);
        (x + y + z).to_string()
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

    #[test]
    fn check_day24_part2_case1() {
        assert_eq!(Day24::solve_part_two(TEST_INPUT), "47".to_string())
    }
}
