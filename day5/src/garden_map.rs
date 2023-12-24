use std::collections::HashMap;
use std::fs::read_to_string;
use std::ops::Range;

pub struct GardenMap {
    seed_to_soil_map: RangeMap,
    soil_to_fertilizer: RangeMap,
    fertilizer_to_water_map: RangeMap,
    water_to_light_map: RangeMap,
    light_to_temperature_map: RangeMap,
    temperature_to_humidity_map: RangeMap,
    humidity_to_location_map: RangeMap,
}

impl GardenMap {
    pub fn new(
        seed_to_soil_map: RangeMap,
        soil_to_fertilizer: RangeMap,
        fertilizer_to_water_map: RangeMap,
        water_to_light_map: RangeMap,
        light_to_temperature_map: RangeMap,
        temperature_to_humidity_map: RangeMap,
        humidity_to_location_map: RangeMap,
    ) -> Self {
        GardenMap {
            seed_to_soil_map,
            soil_to_fertilizer,
            fertilizer_to_water_map,
            water_to_light_map,
            light_to_temperature_map,
            temperature_to_humidity_map,
            humidity_to_location_map,
        }
    }

    // This can be abstracted, but for the sake of Advent of Code I don't think it's worth it :P
    pub fn find_location_for_seed(&self, seed: u64) -> u64 {
        let soil_for_seed = self.seed_to_soil_map.get(seed);
        let fertilizer_for_soil = self.soil_to_fertilizer.get(soil_for_seed);
        let water_for_fertilizer = self.fertilizer_to_water_map.get(fertilizer_for_soil);
        let light_for_water = self.water_to_light_map.get(water_for_fertilizer);
        let temperature_for_light = self.light_to_temperature_map.get(light_for_water);
        let humidity_for_temperature = self.temperature_to_humidity_map.get(temperature_for_light);
        let location_for_humidity = self.humidity_to_location_map.get(humidity_for_temperature);

        // println!(
        //     "Found location for seed {}: {}",
        //     seed, location_for_humidity
        // );

        location_for_humidity
    }
}

pub struct RangeMap {
    range_mappings: HashMap<Range<u64>, Range<u64>>,
}

impl RangeMap {
    pub fn new(mappings_vec: Vec<(Range<u64>, Range<u64>)>) -> Self {
        RangeMap {
            range_mappings: mappings_vec.iter().fold(
                HashMap::new(),
                |mut acc, (input_range, output_range)| {
                    acc.insert(input_range.to_owned(), output_range.to_owned());

                    acc
                },
            ),
        }
    }

    // Obtains the mapping for a given initial value. Search through all provided ranges
    // to determine if we have a mapping, otherwise use the initial value provided.
    pub fn get(&self, value: u64) -> u64 {
        for (input_range, output_range) in &self.range_mappings {
            if input_range.contains(&value) {
                return (value - input_range.start) + output_range.start;
            }
        }

        value
    }
}
