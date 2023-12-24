mod hand;
mod jhand;

use crate::hand::Hand;
use crate::jhand::JHand;
use common::filereader;

fn make_hand_from_line(line: &String) -> Hand {
    let split_line: Vec<&str> = line.split_whitespace().collect();

    let cards = &split_line[0];
    let bet = split_line[1].parse::<u64>().unwrap();

    Hand::new(cards, bet)
}

fn make_jhand_from_line(line: &String) -> JHand {
    let split_line: Vec<&str> = line.split_whitespace().collect();

    let cards = &split_line[0];
    let bet = split_line[1].parse::<u64>().unwrap();

    JHand::new(cards, bet)
}

fn solve(lines: Vec<String>) -> u64 {
    let mut hands: Vec<Hand> = lines.iter().map(|line| make_hand_from_line(line)).collect();

    hands.sort();

    hands.iter().enumerate().fold(0, |acc, (index, hand)| {
        acc + (index as u64 + 1u64) * hand.bet()
    })
}

fn solve2(lines: Vec<String>) -> u64 {
    let mut hands: Vec<JHand> = lines
        .iter()
        .map(|line| make_jhand_from_line(line))
        .collect();

    hands.sort();

    println!("Sorted results: {:?}", hands);

    hands.iter().enumerate().fold(0, |acc, (index, hand)| {
        acc + (index as u64 + 1u64) * hand.bet()
    })
}

fn main() {
    match filereader::read_file("./day7/resources/input.txt") {
        Ok(lines) => {
            let result = solve2(lines);
            println!("{:?}", result);
        }
        Err(e) => panic!("{}", e),
    }
}
