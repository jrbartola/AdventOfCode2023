mod sack;

use common::filereader;
use regex::Regex;
use sack::Sack;

const REFERENCE_SACK: Sack = Sack::new(12, 13, 14);

fn is_valid(sacks: Vec<Sack>) -> bool {
    REFERENCE_SACK.is_valid(&sacks)
}

fn parse_to_sacks(line: &String) -> (u8, Vec<Sack>) {
    let sack_regex = Regex::new(r"^Game (\d+): (.*)$").unwrap();

    if let Some(caps) = sack_regex.captures(line) {
        let game_number = caps.get(1).unwrap().as_str().parse::<u8>().unwrap();
        let sack_sequence = caps.get(2).unwrap().as_str();

        return (
            game_number,
            sack_sequence
                .split("; ")
                .map(|sack_str| Sack::from(sack_str).unwrap())
                .collect(),
        );
    }

    panic!("Bad input string: {}", line);
}

fn solve_line(line: &String) -> u8 {
    let mut result: u8 = 0;

    let (game_number, sacks) = parse_to_sacks(line);

    if is_valid(sacks) {
        result += game_number;
    }

    result
}

fn solve_line_min(line: &String) -> u32 {
    let (_, sacks) = parse_to_sacks(line);
    let min_sack = Sack::get_min_sack(sacks);

    min_sack.power() as u32
}

fn solve(lines: Vec<String>) -> u32 {
    lines
        .iter()
        .fold(0, |acc, line| acc + solve_line(line) as u32)
}

fn solve_min(lines: Vec<String>) -> u32 {
    lines.iter().fold(0, |acc, line| acc + solve_line_min(line))
}

fn main() {
    match filereader::read_file("./day2/resources/input.txt") {
        Ok(lines) => {
            let result = solve_min(lines);
            println!("{:?}", result)
        }
        Err(e) => panic!("{}", e),
    }
}
