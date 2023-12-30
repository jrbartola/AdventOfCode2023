use common::filereader;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use rayon::ThreadPoolBuilder;
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

fn get_combos_optimized(
    line: &Vec<char>,
    requirements: &Vec<u32>,
    line_index: usize,
    req_index: usize,
    curr_req_size: usize,
) -> u32 {
    if line_index >= line.len() {
        let is_valid = req_index == requirements.len()
            || req_index + 1 == requirements.len()
                && curr_req_size == requirements[req_index] as usize;

        return if is_valid { 1 } else { 0 };
    }

    if req_index >= requirements.len() {
        let curr_char = line[line_index];
        return match curr_char {
            '.' | '?' => get_combos_optimized(line, requirements, line_index + 1, req_index, 0),
            '#' => 0,
            other => panic!("Invalid character in string {other}"),
        };
    }

    let curr_char = line[line_index];
    let curr_requirement = requirements[req_index];

    let handle_fixed_case = || {
        if curr_req_size > 0 {
            if (curr_req_size as u32) != curr_requirement {
                return 0;
            }
            get_combos_optimized(line, requirements, line_index + 1, req_index + 1, 0)
        } else {
            get_combos_optimized(line, requirements, line_index + 1, req_index, curr_req_size)
        }
    };

    match curr_char {
        '.' => handle_fixed_case(),
        '?' => {
            // First, check if placing a broken sprocket is valid. If it is, add it to the results
            if (curr_req_size as u32) < curr_requirement {
                return get_combos_optimized(
                    line,
                    requirements,
                    line_index + 1,
                    req_index,
                    curr_req_size + 1,
                ) + handle_fixed_case();
            } else {
                handle_fixed_case()
            }
        }
        '#' => {
            if (curr_req_size as u32) >= curr_requirement {
                return 0;
            }

            get_combos_optimized(
                line,
                requirements,
                line_index + 1,
                req_index,
                curr_req_size + 1,
            )
        }
        other => panic!("Invalid character in string {other}"),
    }
}

fn get_combos(line: String, requirements: &Vec<u32>, line_index: usize) -> u32 {
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
            get_combos_optimized(&line.chars().collect(), &requirements, 0, 0, 0)
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
                let first_combos =
                    get_combos_optimized(&line.chars().collect(), &requirements, 0, 0, 0);
                let second_combos = get_combos_optimized(
                    &format!("{line}?{line}").chars().collect(),
                    &requirements
                        .iter()
                        .cycle()
                        .take(requirements.len() * 2)
                        .map(|v| *v)
                        .collect(),
                    0,
                    0,
                    0,
                );

                let multiplier = second_combos / first_combos;

                println!("Running {}: multiplier={multiplier}", line);

                // second_combos as u128

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
