use common::filereader;

fn get_distance_for_time_held(time_held: u64, time_to_race: u64) -> u64 {
    let remaining_time = time_to_race - time_held;
    let mut distance = time_held * remaining_time;

    distance
}

fn get_ways_for_race(time: u64, distance: u64) -> u64 {
    // Iterate through the number of seconds we could hold the button down
    let winning_distances: Vec<u64> = (0..time)
        .map(|n| get_distance_for_time_held(n, time))
        .filter(|&d| d > distance)
        .collect();

    winning_distances.len() as u64
}

fn get_races_for_lines(lines: &Vec<String>) -> Vec<(u64, u64)> {
    let times_line: Vec<&str> = lines[0].split_whitespace().collect();
    let distances_line: Vec<&str> = lines[1].split_whitespace().collect();

    let zipped: Vec<(u64, u64)> = times_line[1..]
        .iter()
        .zip(distances_line[1..].iter())
        .map(|(&time, &distance)| {
            (
                time.parse::<u64>().unwrap(),
                distance.parse::<u64>().unwrap(),
            )
        })
        .collect();

    zipped
}

fn get_race_for_lines(lines: &Vec<String>) -> (u64, u64) {
    let times_line: Vec<&str> = lines[0].split_whitespace().collect();
    let distances_line: Vec<&str> = lines[1].split_whitespace().collect();

    let time = times_line[1..].join("").parse::<u64>().unwrap();
    let distance = distances_line[1..].join("").parse::<u64>().unwrap();

    (time, distance)
}

fn solve(lines: Vec<String>) -> u64 {
    let races = get_races_for_lines(&lines);

    races
        .iter()
        .map(|&(time, distance)| get_ways_for_race(time, distance))
        .product()
}

fn solve2(lines: Vec<String>) -> u64 {
    let (time, distance) = get_race_for_lines(&lines);

    get_ways_for_race(time, distance)
}

fn main() {
    match filereader::read_file("./day6/resources/input.txt") {
        Ok(lines) => {
            let result = solve2(lines);
            println!("{:?}", result);
        }
        Err(e) => panic!("{}", e),
    }
}
