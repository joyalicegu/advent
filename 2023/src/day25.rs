use crate::Solution;
use itertools::Itertools;
use petgraph::algo::has_path_connecting;
use petgraph::prelude::*;
use petgraph::Graph;
use std::collections::HashMap;

pub struct Day25;

impl Day25 {
    fn find_component_sizes(graph: &UnGraph<(), ()>) -> (usize, usize) {
        let edge_cut: Vec<(EdgeIndex, EdgeIndex, EdgeIndex)> = graph
            .edge_indices()
            .combinations(3)
            .map(|combo| {
                let (a, b, c) = combo.iter().collect_tuple().unwrap();
                (*a, *b, *c)
            })
            .collect();
        for (i, j, k) in edge_cut {
            let (iu, iv) = graph.edge_endpoints(i).unwrap();
            let (ju, jv) = graph.edge_endpoints(j).unwrap();
            let (ku, kv) = graph.edge_endpoints(k).unwrap();
            let mut cut_graph = graph.clone();
            cut_graph.remove_edge(cut_graph.find_edge(iu, iv).unwrap());
            cut_graph.remove_edge(cut_graph.find_edge(ju, jv).unwrap());
            cut_graph.remove_edge(cut_graph.find_edge(ku, kv).unwrap());
            if has_path_connecting(&cut_graph, iu, iv, None) {
                // not an edge cut
                continue;
            } else {
                println!("edge cut: {:?}", (i, j, k));
                let mut count = 0;
                let mut count2 = 0;
                for node in cut_graph.node_indices() {
                    if has_path_connecting(&cut_graph, iu, node, None) {
                        count += 1;
                    } else {
                        count2 += 1;
                    }
                }
                println!("component sizes: {:?}", (count, count2));
                return (count, count2);
            }
        }
        panic!("failed to find edge cut");
    }
}

impl Solution for Day25 {
    type ParsedInput = UnGraph<(), ()>;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        let mut graph = Graph::new_undirected();
        let mut nodes = HashMap::new();
        for line in input_lines.lines() {
            let (u_str, etc) = line.split_once(": ").unwrap();
            for v_str in etc.split_whitespace() {
                let u_key = u_str.to_string();
                if !nodes.contains_key(&u_key) {
                    nodes.insert(u_key.clone(), graph.add_node(()));
                }
                let u = *nodes.get(&u_key).unwrap();

                let v_key = v_str.to_string();
                if !nodes.contains_key(&v_key) {
                    nodes.insert(v_key.clone(), graph.add_node(()));
                }
                let v = *nodes.get(&v_key).unwrap();

                graph.add_edge(u, v, ());
            }
        }
        println!("graph: {:?}", graph);
        graph
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        let graph = _parsed_input;
        let (a, b) = Self::find_component_sizes(&graph);
        (a * b).to_string()
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
