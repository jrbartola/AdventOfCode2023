mod galaxies;
mod unordered_pair;

use crate::galaxies::{Galaxies, GalaxyCell};
use crate::unordered_pair::UnorderedPair;
use common::filereader;
use std::collections::HashMap;

fn get_cumulative_distances(
    mappings: HashMap<(usize, usize), HashMap<(usize, usize), u32>>,
) -> u128 {
    let mut distances: HashMap<UnorderedPair<(usize, usize)>, u128> = HashMap::new();

    mappings.iter().for_each(|(&from_coords, to_mapping)| {
        to_mapping.iter().for_each(|(&to_coords, &dist)| {
            let pair = UnorderedPair(from_coords, to_coords);

            if !distances.contains_key(&pair) {
                // println!(
                //     "Shortest dist between {:?} and {:?} is {}",
                //     from_coords, to_coords, dist
                // );
                distances.insert(pair, dist as u128);
            }
        })
    });

    distances.values().sum()
}

fn expand_galaxies(lines: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut expanded = Vec::new();

    for line in lines.iter() {
        expanded.push(line.clone());

        if line.iter().all(|&c| c == '.') {
            expanded.push(line.clone());
        }
    }

    // Don't mutate directly since the loop is actively iterating over indices
    let mut indices_to_insert = Vec::new();

    for col in 0..expanded[0].len() {
        if (0..expanded.len()).all(|row| expanded[row][col] == '.') {
            for row in 0..expanded.len() {
                indices_to_insert.push((row, col));
            }
        }
    }

    // Reverse the iterator so we don't screw up the indices as we modify in-place and
    // all elements are shifted to the right
    indices_to_insert
        .iter()
        .rev()
        .for_each(|&(r, c)| expanded[r].insert(c, '.'));

    expanded
}

fn solve(lines: Vec<String>) -> u128 {
    let char_vecs: Vec<Vec<char>> = lines
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let expanded_galaxies_vec = expand_galaxies(char_vecs)
        .iter()
        .map(|v| v.iter().map(|&c| GalaxyCell::new(c, 1)).collect())
        .collect();
    let galaxies = Galaxies::new(expanded_galaxies_vec);
    println!("{:?}", galaxies);

    let bfs_mappings = galaxies.get_shortest_distances();

    get_cumulative_distances(bfs_mappings)
}

fn get_adjusted_distance_for_line_and_col(line: &String, col: String) -> u32 {
    if line
        .chars()
        .all(|c| c == '.' || col.chars().all(|c| c == '.'))
    {
        1_000_000
    } else {
        1
    }
}

fn solve2(lines: Vec<String>) -> u128 {
    let galaxy_cells = lines
        .iter()
        .map(|line| {
            line.chars()
                .enumerate()
                .map(|(char_idx, c)| {
                    GalaxyCell::new(
                        c,
                        get_adjusted_distance_for_line_and_col(
                            line,
                            lines.iter().fold(String::new(), |mut acc, line| {
                                acc.push(line.chars().nth(char_idx).unwrap());
                                acc
                            }),
                        ),
                    )
                })
                .collect()
        })
        .collect();

    let galaxies = Galaxies::new(galaxy_cells);

    let bfs_mappings = galaxies.get_shortest_distances();

    get_cumulative_distances(bfs_mappings) as u128
}

fn main() {
    match filereader::read_file("./day11/resources/input.txt") {
        Ok(lines) => {
            let result = solve2(lines);
            println!("{:?}", result);
        }
        Err(e) => panic!("{}", e),
    }
}
