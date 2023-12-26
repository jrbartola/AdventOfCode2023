mod map_node;

use crate::map_node::{Direction, MapGraph, NodePath};
use common::filereader;
use regex::Regex;
use std::collections::HashMap;

fn get_directions(line: &str) -> Vec<Direction> {
    line.chars()
        .map(|c| match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            other => panic!("Bad character for directions: {}", other),
        })
        .collect()
}

fn build_graph(lines: &[String]) -> MapGraph {
    let node_regex = Regex::new(r"([A-Z0-9]{3}) = \(([A-Z0-9]{3}), ([A-Z0-9]{3})\)").unwrap();
    let mut nodes: HashMap<String, NodePath> = HashMap::new();

    // First iterate through the list and create a node for each line
    for line in lines {
        if let Some(caps) = node_regex.captures(line) {
            let node_name = caps.get(1).unwrap().as_str();
            let node_left = caps.get(2).unwrap().as_str();
            let node_right = caps.get(3).unwrap().as_str();

            nodes.insert(
                node_name.to_string(),
                NodePath::new(node_name, node_left, node_right),
            );
        }
    }

    MapGraph::new(nodes)
}

fn solve(lines: Vec<String>) -> u64 {
    let directions = get_directions(&lines[0]);
    let map_graph = build_graph(&lines[2..]);

    map_graph.compute_distance("AAA", "ZZZ", &directions)
}

fn solve2(lines: Vec<String>) -> u128 {
    let directions = get_directions(&lines[0]);
    let map_graph = build_graph(&lines[2..]);

    map_graph.compute_simul_distances(&directions)
}

fn main() {
    match filereader::read_file("./day8/resources/input.txt") {
        Ok(lines) => {
            let result = solve2(lines);
            println!("{:?}", result);
        }
        Err(e) => panic!("{}", e),
    }
}
