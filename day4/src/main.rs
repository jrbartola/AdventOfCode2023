mod scratch_card;

use crate::scratch_card::{ScratchCard, ScratchCardBoard};
use common::filereader;
use regex::Regex;
use std::collections::HashSet;

fn parse_line(line: &String) -> ScratchCard {
    let scratch_card_regex = Regex::new(r"^Card +\d+: +((\d+ +)+)\| +((\d+ *)+)").unwrap();

    if let Some(caps) = scratch_card_regex.captures(line) {
        let winners = caps.get(1).unwrap().as_str();

        let winners_set: HashSet<u32> = winners
            .split_whitespace()
            .map(|maybe_num| maybe_num.parse::<u32>().unwrap())
            .collect();

        let drawn = caps.get(3).unwrap().as_str();

        let drawn_set: HashSet<u32> = drawn
            .split_whitespace()
            .map(|maybe_num| maybe_num.parse::<u32>().unwrap())
            .collect();

        return ScratchCard::new(winners_set, drawn_set);
    }

    panic!("Bad line for capture: {}", line);
}

fn solve(lines: Vec<String>) -> u32 {
    lines
        .iter()
        .map(|line| parse_line(line))
        .map(|scratch_card| scratch_card.get_value())
        .sum()
}

fn solve2(lines: Vec<String>) -> u32 {
    let scratch_card_board =
        ScratchCardBoard::new(lines.iter().map(|line| parse_line(line)).collect());

    scratch_card_board.compute_winners()
}

fn main() {
    match filereader::read_file("./day4/resources/input.txt") {
        Ok(lines) => {
            let result = solve2(lines);
            println!("{:?}", result)
        }
        Err(e) => panic!("{}", e),
    }
}
