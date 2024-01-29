use crate::Solution;
use std::collections::{HashMap, HashSet};

pub struct Day25;

impl Day25 {
    fn count(
        v: &String,
        graph: &HashMap<String, HashSet<String>>,
        vertices: &HashSet<String>,
    ) -> usize {
        let neighbors = graph.get(v).unwrap();
        neighbors.difference(vertices).count()
    }

    fn find_component_sizes(graph: &HashMap<String, HashSet<String>>) -> (usize, usize) {
        // from https://www.reddit.com/r/adventofcode/comments/18qbsxs/comment/ketzp94/
        let mut vertices = graph.keys().map(|s| s.clone()).collect::<HashSet<String>>();
        while vertices
            .iter()
            .map(|v| Self::count(v, graph, &vertices))
            .sum::<usize>()
            != 3
        {
            let (_, v) = vertices
                .iter()
                .map(|v| (Self::count(v, graph, &vertices), v))
                .max()
                .unwrap();
            vertices.remove(&(v.clone()));
        }
        (vertices.len(), graph.keys().count() - vertices.len())
    }
}

impl Solution for Day25 {
    type ParsedInput = HashMap<String, HashSet<String>>;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        let mut adj = HashMap::new();
        for line in input_lines.lines() {
            let (u, etc) = line.split_once(": ").unwrap();
            for v in etc.split_whitespace() {
                adj.entry(u.to_string())
                    .or_insert(HashSet::new())
                    .insert(v.to_string());
                adj.entry(v.to_string())
                    .or_insert(HashSet::new())
                    .insert(u.to_string());
            }
        }
        adj
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        let graph = _parsed_input;
        let (a, b) = Self::find_component_sizes(&graph);
        (a * b).to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        "merry christmas".to_string()
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
