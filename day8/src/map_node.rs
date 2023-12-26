use rand::Rng;
use std::collections::{HashMap, HashSet};

#[derive(Eq, PartialEq)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
pub struct NodePath {
    name: String,
    left: String,
    right: String,
    is_start: bool,
    is_end: bool,
}

impl NodePath {
    pub fn new(name: &str, left: &str, right: &str) -> Self {
        NodePath {
            name: name.to_string(),
            left: left.to_string(),
            right: right.to_string(),
            is_start: name.ends_with("A"),
            is_end: name.ends_with("Z"),
        }
    }

    pub fn is_start(&self) -> bool {
        self.is_start
    }

    pub fn is_end(&self) -> bool {
        self.is_end
    }
}

#[derive(Debug)]
pub struct MapGraph {
    nodes_to_paths: HashMap<String, NodePath>,
}

impl MapGraph {
    pub fn new(nodes: HashMap<String, NodePath>) -> Self {
        MapGraph {
            nodes_to_paths: nodes,
        }
    }

    pub fn compute_distance(&self, start: &str, end: &str, directions: &Vec<Direction>) -> u64 {
        // TODO: Track visited nodes if we need to...
        let mut curr_node = start;
        let mut iterations = 0_u64;

        while curr_node != end {
            println!("Iteration: {}", iterations);
            let bounded_iter_index = (iterations % (directions.len() as u64)) as usize;
            let next_direction = &directions[bounded_iter_index];

            let node_path = self.get(curr_node);

            let next_node_name = match next_direction {
                Direction::Left => node_path.left.as_str(),
                Direction::Right => node_path.right.as_str(),
            };

            curr_node = next_node_name;
            iterations += 1;
        }

        iterations
    }

    pub fn compute_simul_distances(&self, directions: &Vec<Direction>) -> u128 {
        let mut curr_nodes: Vec<&NodePath> = self
            .nodes_to_paths
            .values()
            .filter(|node_path| node_path.is_start)
            .collect();
        let mut iterations_vec: Vec<u128> = vec![0; curr_nodes.len()];
        let mut iterations = 0_u128;

        while iterations_vec.iter().any(|&it| it == 0) {
            if rand::thread_rng().gen_range(0..1000000) < 2 {
                println!("Iteration: {:?}", iterations_vec);
            }

            let bounded_iter_index = (iterations % (directions.len() as u128)) as usize;
            let next_direction = &directions[bounded_iter_index];

            curr_nodes = curr_nodes
                .iter()
                .map(|node| {
                    let node_path = self.get(node.name.as_str());

                    let next_node_name = match next_direction {
                        Direction::Left => node_path.left.as_str(),
                        Direction::Right => node_path.right.as_str(),
                    };

                    self.get(next_node_name)
                })
                .collect();

            curr_nodes.iter().enumerate().for_each(|(i, &&ref node)| {
                if iterations_vec[i] == 0 && node.is_end {
                    iterations_vec[i] = iterations + 1;
                }
            });

            iterations += 1;
        }

        println!("Finished: {:?}", iterations_vec);

        fn gcd(a: u128, b: u128) -> u128 {
            match b {
                0 => a,
                _ => gcd(b, a % b),
            }
        }

        fn lcm(a: u128, b: u128) -> u128 {
            (a / gcd(a, b)) * b
        }

        iterations_vec.iter().fold(1, |acc, &v| lcm(acc, v))
    }

    fn get(&self, name: &str) -> &NodePath {
        self.nodes_to_paths.get(name).unwrap()
    }
}
