use crate::Solution;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub struct Day25;

impl Day25 {
    // TODO helpers
}

impl Solution for Day25 {
    type ParsedInput = (HashSet<(usize, usize)>, HashMap<usize, HashSet<usize>>);

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        let mut edges = HashSet::<(usize, usize)>::new();
        let mut adj = HashMap::new();
        let mut indices = HashMap::<String, usize>::new();
        for line in input_lines.lines() {
            let (u_str, etc) = line.split_once(": ").unwrap();
            for v_str in etc.split_whitespace() {
                let i = indices.len();
                let mut u = *indices.entry(u_str.to_string()).or_insert(i);
                let i = indices.len();
                let mut v = *indices.entry(v_str.to_string()).or_insert(i);
                if v > u {
                    (u, v) = (v, u);
                }
                edges.insert((u, v));
                adj.entry(u).or_insert(HashSet::new()).insert(v);
                adj.entry(v).or_insert(HashSet::new()).insert(u);
            }
        }
        println!("adj:");
        for src in adj.keys().sorted() {
            let v = adj.get(src).unwrap();
            println!("{:?}: {:?}", src, v);
        }
        (edges, adj)
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        "0".to_string()
        // TODO
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        "0".to_string()
        // TODO
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";

    #[test]
    fn check_day25_part1_case1() {
        assert_eq!(Day25::solve_part_one(TEST_INPUT), "54".to_string())
    }
}
