mod garden_map;
mod garden_map_parser;

use crate::garden_map_parser::parse_garden_map;
use common::filereader;
use rand::Rng;
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use regex::Regex;
use std::cmp::min;
use std::sync::Mutex;

fn extract_seeds(line: &String) -> Vec<u64> {
    let seed_regex = Regex::new(r"^seeds: (.*)").unwrap();

    if let Some(caps) = seed_regex.captures(line) {
        return caps
            .get(1)
            .unwrap()
            .as_str()
            .split_whitespace()
            .map(|seed_str| seed_str.parse::<u64>().unwrap())
            .collect();
    }

    panic!("Bad input for seed extraction: {}", line);
}

fn extract_seed_ranges(line: &String) -> Vec<(u64, u64)> {
    let seed_regex = Regex::new(r"^seeds: (.*)").unwrap();

    if let Some(caps) = seed_regex.captures(line) {
        let seed_range_coords: Vec<u64> = caps
            .get(1)
            .unwrap()
            .as_str()
            .split_whitespace()
            .map(|seed_str| seed_str.parse::<u64>().unwrap())
            .collect();

        let mut seed_ranges: Vec<(u64, u64)> = Vec::new();

        for i in (0..seed_range_coords.len()).step_by(2) {
            let start = seed_range_coords[i];
            let len = seed_range_coords[i + 1];

            seed_ranges.push((start, start + len));
        }

        return seed_ranges;
    }

    panic!("Bad input for seed extraction: {}", line);
}

fn solve(lines: Vec<String>) -> u64 {
    let seeds = extract_seeds(&lines[0]);
    let garden_map = parse_garden_map(&lines);

    seeds
        .iter()
        .map(|&seed| garden_map.find_location_for_seed(seed))
        .min()
        .unwrap()
}

fn solve2(lines: Vec<String>) -> u64 {
    let seeds = extract_seed_ranges(&lines[0]);
    let garden_map = parse_garden_map(&lines);

    println!("Parsed seeds: {:?}", seeds);

    let min_value = Mutex::new(u64::MAX);

    let mut iterations_processed = Mutex::new(0_u128);

    // Build a custom thread pool
    let pool = ThreadPoolBuilder::new().num_threads(10).build().unwrap();

    // Use the custom pool for parallel operations
    pool.install(|| {
        seeds.par_iter().for_each(|&(range_start, range_end)| {
            let mut local_min: u64 = u64::MAX;
            let mut local_iterations: u128 = 0;

            for i in range_start..range_end {
                local_min = min(local_min, garden_map.find_location_for_seed(i));
                local_iterations += 1;

                if rand::thread_rng().gen_range(0..1000000) < 2 {
                    println!(
                        "Iteration: {}%",
                        (local_iterations as f32) / ((range_end - range_start) as f32)
                    );
                }
            }

            // Update the global minimum value
            let mut min_value = min_value.lock().unwrap();
            *min_value = min(*min_value, local_min);

            *iterations_processed.lock().unwrap() += local_iterations;
            println!("Thread finished with {} iterations.", local_iterations);
        });
    });

    let total_min = *min_value.lock().unwrap();

    total_min
}

fn main() {
    match filereader::read_file("./day5/resources/input.txt") {
        Ok(lines) => {
            let result = solve2(lines);
            println!("{:?}", result)
        }
        Err(e) => panic!("{}", e),
    }
}
