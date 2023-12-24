use crate::garden_map::{GardenMap, RangeMap};
use std::ops::Range;

fn parse_ranges(dest_start: u64, source_start: u64, length: u64) -> (Range<u64>, Range<u64>) {
    (
        source_start..(source_start + length),
        dest_start..(dest_start + length),
    )
}

fn parse_for_header(lines: &Vec<String>, name: &str) -> Vec<(Range<u64>, Range<u64>)> {
    // Start at index 1 because seeds are on the first line
    let mut line_index = 1;
    let mut results: Vec<(Range<u64>, Range<u64>)> = Vec::new();

    while line_index < lines.len() && !lines[line_index].contains(name) {
        line_index += 1;
    }

    if lines[line_index].contains(name) {
        line_index += 1;

        // Keep parsing while we still have ranges to parse
        while line_index < lines.len() && lines[line_index].len() > 0 {
            let range_points: Vec<u64> = lines[line_index]
                .split_whitespace()
                .map(|str_num| str_num.parse::<u64>().unwrap())
                .collect();

            results.push(parse_ranges(
                range_points[0],
                range_points[1],
                range_points[2],
            ));

            line_index += 1;
        }

        return results;
    }

    panic!("Header name not found: {}", name);
}

pub fn parse_garden_map(lines: &Vec<String>) -> GardenMap {
    let seed_to_soil_ranges = parse_for_header(lines, "seed-to-soil map:");
    let soil_to_fertilizer_ranges = parse_for_header(lines, "soil-to-fertilizer map:");
    let fertilizer_to_water_ranges = parse_for_header(lines, "fertilizer-to-water map:");
    let water_to_light_ranges = parse_for_header(lines, "water-to-light map:");
    let light_to_temperature_ranges = parse_for_header(lines, "light-to-temperature map:");
    let temperature_to_humidity_ranges = parse_for_header(lines, "temperature-to-humidity map:");
    let humidity_to_location_ranges = parse_for_header(lines, "humidity-to-location map:");

    GardenMap::new(
        RangeMap::new(seed_to_soil_ranges),
        RangeMap::new(soil_to_fertilizer_ranges),
        RangeMap::new(fertilizer_to_water_ranges),
        RangeMap::new(water_to_light_ranges),
        RangeMap::new(light_to_temperature_ranges),
        RangeMap::new(temperature_to_humidity_ranges),
        RangeMap::new(humidity_to_location_ranges),
    )
}
