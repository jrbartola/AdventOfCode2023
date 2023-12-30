use common::filereader;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use rayon::ThreadPoolBuilder;
use std::cmp::max;
use std::ptr::read_unaligned;
use std::sync::Mutex;

fn is_valid_line(line: &[char], requirements: &Vec<u32>) -> bool {
    let mut line_idx = 0;
    let mut req_idx = 0;

    let mut curr_str = String::new();

    while line_idx < line.len() {
        let curr_char = line[line_idx];

        match curr_char {
            '.' => {
                if !curr_str.is_empty() {
                    if curr_str.len() as u32 != requirements[req_idx] {
                        return false;
                    }
                    curr_str = String::new();
                    req_idx += 1;
                }
            }
            '#' => {
                let curr_requirement = requirements.get(req_idx);

                match curr_requirement {
                    Some(&curr_requirement) => {
                        curr_str.push('#');

                        if curr_str.len() as u32 > curr_requirement {
                            return false;
                        }
                    }
                    None => {
                        return false;
                    }
                }
            }
            _ => panic!("Invalid character at index {line_idx}"),
        }

        line_idx += 1;
    }

    req_idx == requirements.len()
        || (req_idx + 1 == requirements.len() && curr_str.len() == requirements[req_idx] as usize)
}

fn get_combos(line: String, requirements: &Vec<u32>, line_index: usize) -> u32 {
    // if rand::thread_rng().gen_range(0..10000000) < 2 {
    //     println!("{line}");
    // }

    if line_index >= line.len() {
        return if is_valid_line(&line.chars().collect::<Vec<char>>(), requirements) {
            1
        } else {
            0
        };
    }

    if line.chars().nth(line_index).unwrap() == '?' {
        let mut string_with_bust: Vec<char> = line.chars().collect();
        string_with_bust[line_index] = '#';

        let mut string_with_fix: Vec<char> = line.chars().collect();
        string_with_fix[line_index] = '.';

        return get_combos(
            string_with_bust.into_iter().collect(),
            requirements,
            line_index + 1,
        ) + get_combos(
            string_with_fix.into_iter().collect(),
            requirements,
            line_index + 1,
        );
    } else {
        return get_combos(line, requirements, line_index + 1);
    }
}

fn parse_line(line: String) -> (String, Vec<u32>) {
    let splitted: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();

    let requirements: Vec<u32> = splitted[1]
        .split(",")
        .map(|r| r.parse::<u32>().unwrap())
        .collect();

    (splitted[0].to_owned(), requirements)
}

fn parse_line_mult(line: String, multiplier: usize) -> (String, Vec<u32>) {
    let splitted: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();

    let requirements: Vec<u32> = splitted[1]
        .split(',')
        .flat_map(|part| part.chars())
        .map(|r| r.to_digit(10).unwrap())
        .collect();

    let requirements: Vec<_> = requirements
        .iter()
        .map(|d| *d)
        .cycle()
        .take(requirements.len() * multiplier)
        .collect();

    let mut repeated_line = String::new();

    for i in 0..multiplier {
        if i > 0 {
            repeated_line.push('?');
        }
        repeated_line.push_str(&splitted[0]);
    }

    (repeated_line, requirements)
}

fn solve(lines: Vec<String>) -> u32 {
    lines
        .into_iter()
        .map(|line| parse_line(line))
        .enumerate()
        .map(|(idx, (line, requirements))| {
            println!("Running {line}: {idx}");
            get_combos(line, &requirements, 0)
        })
        .sum()
}

fn solve2(lines: Vec<String>) -> u128 {
    // let s = "???.### 1,1,3";
    // let (l, r) = parse_line_mult(s.to_owned(), 5);
    //
    // let combos = get_combos(l.clone(), &r, 0);
    //
    // println!("{}: {}", l.clone(), combos);
    //
    // combos as u128

    // Build a custom thread pool
    let pool = ThreadPoolBuilder::new().num_threads(10).build().unwrap();

    let mapped_lines: Vec<_> = lines
        .into_iter()
        .map(|line| parse_line(line.clone()))
        .enumerate()
        .collect();

    let total_sum = Mutex::new(0);

    pool.install(|| {
        let local_sum: u128 = mapped_lines
            .par_iter()
            .map(|(idx, (line, requirements))| {
                let first_combos = get_combos(line.to_string(), &requirements, 0);
                let second_combos = get_combos(format!("?{line}"), &requirements, 0);
                let third_comboas = get_combos(format!("{line}?"), &requirements, 0);

                let multiplier = if line.ends_with("#") {
                    let multi_combos = get_combos(
                        format!("{line}?{line}"),
                        &requirements
                            .iter()
                            .cycle()
                            .take(requirements.len() * 2)
                            .map(|v| *v)
                            .collect(),
                        0,
                    );
                    multi_combos / first_combos
                } else {
                    max(second_combos, third_comboas)
                };

                println!("Running {line}: {idx}. Multiplier: {multiplier}");

                (first_combos as u128) * ((multiplier as u128).pow(4))
            })
            .sum();

        // Update the global sum value
        let mut sum = total_sum.lock().unwrap();
        *sum = *sum + local_sum;
    });

    let x = total_sum.lock().unwrap();

    *x
}

fn main() {
    match filereader::read_file("./day12/resources/input.txt") {
        Ok(lines) => {
            let result = solve2(lines);
            println!("{:?}", result);
        }
        Err(e) => panic!("{}", e),
    }
}
