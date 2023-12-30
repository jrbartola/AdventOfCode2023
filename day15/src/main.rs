use common::filereader;
use std::ops::Index;

struct Lense<'a> {
    label: &'a str,
    focal_length: u32,
}

enum LenseCommand<'a> {
    Remove(&'a str),
    Add(Lense<'a>),
}

fn reindeer_hash(str: &str) -> u32 {
    let mut current_value = 0;

    for c in str.chars() {
        let unicode_value = c as u32;

        current_value += unicode_value;
        current_value *= 17;
        current_value %= 256;
    }

    current_value
}

fn map_to_command(line: &str) -> LenseCommand {
    if line.contains("=") {
        let splitted: Vec<String> = line.split("=").map(|s| s.to_string()).collect();

        LenseCommand::Add(Lense {
            label: &line[..line.chars().position(|c| c == '=').unwrap()],
            focal_length: splitted[1].parse::<u32>().unwrap(),
        })
    } else {
        LenseCommand::Remove(&line[..line.len() - 1])
    }
}

fn solve(lines: Vec<String>) -> u32 {
    println!("{}", reindeer_hash("qp"));
    lines[0].split(",").map(|s| reindeer_hash(s)).sum()
}

fn solve2(lines: Vec<String>) -> u32 {
    let mut boxes: Vec<Vec<Lense>> = (0..256).map(|_| Vec::new()).collect();

    let lense_commands: Vec<LenseCommand> =
        lines[0].split(",").map(|str| map_to_command(str)).collect();

    for command in lense_commands {
        match command {
            LenseCommand::Add(lense) => {
                let box_num = reindeer_hash(lense.label) as usize;

                if let Some(pos) = boxes[box_num].iter().position(|l| l.label == lense.label) {
                    boxes[box_num][pos] = lense;
                } else {
                    boxes[box_num].push(lense);
                }
            }
            LenseCommand::Remove(label) => {
                let box_num = reindeer_hash(label) as usize;

                if let Some(pos) = boxes[box_num].iter().position(|l| l.label == label) {
                    boxes[box_num].remove(pos);
                }
            }
        }
    }

    boxes
        .iter()
        .enumerate()
        .map(|(i, lenses)| {
            lenses
                .iter()
                .enumerate()
                .map(|(l_index, lense)| (i + 1) as u32 * (l_index + 1) as u32 * lense.focal_length)
                .sum::<u32>()
        })
        .sum()
}

fn main() {
    match filereader::read_file("./day15/resources/input.txt") {
        Ok(lines) => {
            let result = solve2(lines);
            println!("{:?}", result);
        }
        Err(e) => panic!("{}", e),
    }
}
