use crate::Solution;
use itertools::Itertools;
use std::cmp;
use std::collections::HashMap;

pub struct Day08;

impl Day08 {
    fn circuits(
        junctions: &Vec<(usize, usize, usize)>,
        limit: Option<usize>,
    ) -> (
        Vec<Vec<(usize, usize, usize)>>,
        Vec<((usize, usize, usize), (usize, usize, usize))>,
    ) {
        let mut circuits: Vec<Vec<(usize, usize, usize)>> = junctions
            .iter()
            .map(|&junction| Vec::from([junction]))
            .collect();
        let mut junction_to_circuit: HashMap<(usize, usize, usize), usize> = HashMap::new();
        for (i, circuit) in circuits.iter().enumerate() {
            for &junction in circuit.iter() {
                junction_to_circuit.insert(junction, i);
            }
        }
        // println!("junction_to_circuit {:?}", junction_to_circuit);
        let mut pairs: Vec<(usize, (usize, usize, usize), (usize, usize, usize))> = Vec::new();
        for (&u, &v) in junctions.iter().tuple_combinations() {
            // distance squared
            let d = (((u.0 as isize - v.0 as isize) * (u.0 as isize - v.0 as isize))
                + ((u.1 as isize - v.1 as isize) * (u.1 as isize - v.1 as isize))
                + ((u.2 as isize - v.2 as isize) * (u.2 as isize - v.2 as isize)))
                as usize;
            pairs.push((d, u, v));
        }
        pairs.sort();
        pairs.reverse();
        let mut log = Vec::new();
        let mut total_circuits = circuits.len();
        loop {
            if limit.is_some() && log.len() >= limit.unwrap() {
                break;
            }
            if limit.is_none() && total_circuits == 1 {
                break;
            }
            let (_, u, v) = pairs.pop().unwrap();
            let i = junction_to_circuit[&u];
            let j = junction_to_circuit[&v];
            // println!("-------------------------------");
            // println!("connecting {:?} and {:?}", u, v);
            log.push((u, v));
            if i == j {
                continue;
            }
            let (dst, src) = (cmp::min(i, j), cmp::max(i, j));
            // println!("before merge:");
            // println!("circuits[dst]: {:?}", circuits[dst]);
            // println!("circuits[src]: {:?}", circuits[src]);
            let mut merged_circuit = circuits[dst].clone();
            merged_circuit.extend(circuits[src].clone());
            circuits[dst] = merged_circuit;
            circuits[src].clear();
            // println!("after merge:");
            // println!("{:?}", circuits[dst]);
            // println!("size: {:?}", circuits[dst].len());
            // println!("{:?}", circuits.iter().map(|c| c.len()).collect::<Vec<_>>());
            for junction in circuits[dst].iter() {
                junction_to_circuit.insert(*junction, dst);
            }
            total_circuits -= 1;
        }
        // println!("-------------------------------");
        circuits = circuits
            .into_iter()
            .filter(|circuit| circuit.len() != 0)
            .collect();
        // for circuit in circuits.iter() {
        // println!("{:?}", circuit);
        // }
        // println!("{:?} circuits", circuits.len());
        (circuits, log)
    }
}

impl Solution for Day08 {
    type ParsedInput = Vec<(usize, usize, usize)>;

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
        let junctions = _parsed_input;
        Self::circuits(junctions, Some(1000))
            .0
            .iter()
            .map(|circuit| circuit.len())
            .sorted()
            .rev()
            .take(3)
            .product::<usize>()
            .to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        let junctions = _parsed_input;
        Self::circuits(junctions, None)
            .1
            .iter()
            .rev()
            .take(1)
            .map(|(u, v)| u.0 * v.0)
            .next()
            .unwrap()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn check_day08_part1_case1() {
        let junctions = &mut Day08::parse_input(TEST_INPUT);
        assert_eq!(
            Day08::circuits(junctions, Some(10))
                .0
                .iter()
                .map(|circuit| circuit.len())
                .sorted()
                .rev()
                .take(3)
                .product::<usize>()
                .to_string(),
            "40".to_string()
        )
    }

    #[test]
    fn check_day08_part2_case1() {
        let junctions = &mut Day08::parse_input(TEST_INPUT);
        assert_eq!(
            Day08::circuits(junctions, None)
                .1
                .iter()
                .rev()
                .take(1)
                .map(|(u, v)| u.0 * v.0)
                .next()
                .unwrap()
                .to_string(),
            "25272".to_string()
        )
    }
}
