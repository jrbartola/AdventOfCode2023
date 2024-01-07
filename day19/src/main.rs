mod workflow;

use crate::workflow::{Instruction, Operator, Part, PartRange, Workflow};
use common::filereader;
use regex::Regex;
use std::collections::{HashMap, VecDeque};

fn parse_instructions(line: &str) -> (Vec<Instruction>, String) {
    let instruction_regex = Regex::new(r"(a|m|s|x)(>|<)(\d+):([A-z]+)").unwrap();
    let instruction_strings = line.split(",");
    let default = instruction_strings.clone().last().unwrap();

    let instructions: Vec<Instruction> = instruction_strings
        .clone()
        .into_iter()
        .take(instruction_strings.count() - 1)
        .map(|segment| {
            if let Some(captures) = instruction_regex.captures(segment) {
                let part_attribute = captures.get(1).unwrap().as_str().chars().next().unwrap();
                let operator = match captures.get(2).unwrap().as_str().chars().next().unwrap() {
                    '<' => Operator::LessThan,
                    '>' => Operator::GreaterThan,
                    _ => unreachable!(),
                };
                let value = captures.get(3).unwrap().as_str().parse::<usize>().unwrap();
                let destination = captures.get(4).unwrap().as_str();

                return Instruction::new(destination.to_string(), operator, value, part_attribute);
            }

            unreachable!();
        })
        .collect();

    (instructions, default.to_string())
}

fn parse_lines(lines: Vec<String>) -> (Vec<Workflow>, Vec<Part>) {
    let workflow_regex = Regex::new(r"([a-z]+)\{(.*)}").unwrap();
    let part_regex = Regex::new(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)}").unwrap();
    let mut line_idx = 0;

    let mut workflows = Vec::new();
    let mut parts = Vec::new();

    while line_idx < lines.len() {
        let curr_line = lines[line_idx].as_str();

        if curr_line.len() == 0 {
            line_idx += 1;
            break;
        }

        let captures = workflow_regex.captures(curr_line).unwrap();
        let workflow_name = captures.get(1).unwrap().as_str().to_owned();
        let (instructions, default) = parse_instructions(captures.get(2).unwrap().as_str());

        workflows.push(Workflow::new(workflow_name, instructions, default));

        line_idx += 1;
    }

    while line_idx < lines.len() {
        let captures = part_regex.captures(lines[line_idx].as_str()).unwrap();

        parts.push(Part::new(
            captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
            captures.get(2).unwrap().as_str().parse::<usize>().unwrap(),
            captures.get(3).unwrap().as_str().parse::<usize>().unwrap(),
            captures.get(4).unwrap().as_str().parse::<usize>().unwrap(),
        ));

        line_idx += 1;
    }

    (workflows, parts)
}

fn run_workflows(workflows: Vec<Workflow>, parts: Vec<Part>) -> HashMap<Part, bool> {
    let workflows_by_name = workflows.into_iter().fold(HashMap::new(), |mut hmap, wf| {
        hmap.insert(wf.name.clone(), wf);
        hmap
    });

    let mut parts_to_paths: HashMap<Part, Vec<&str>> = HashMap::new();
    let mut parts_to_acceptance = HashMap::new();

    'part: for part in parts {
        let mut curr_flow = workflows_by_name.get(&String::from("in")).unwrap();
        let mut curr_path = vec!["in"];

        'flows: loop {
            for instruction in &curr_flow.instructions {
                match instruction.matches(part) {
                    true => {
                        if instruction.destination == "A" {
                            parts_to_acceptance.insert(part, true);
                            curr_path.push(&instruction.destination);
                            parts_to_paths.insert(part, curr_path);
                            continue 'part;
                        } else if instruction.destination == "R" {
                            parts_to_acceptance.insert(part, false);
                            curr_path.push(&instruction.destination);
                            parts_to_paths.insert(part, curr_path);
                            continue 'part;
                        }
                        curr_flow = workflows_by_name.get(&instruction.destination).unwrap();
                        curr_path.push(&instruction.destination);
                        continue 'flows;
                    }
                    false => {}
                }
            }

            if curr_flow.default == "A" {
                parts_to_acceptance.insert(part, true);
                curr_path.push("A");
                parts_to_paths.insert(part, curr_path);
                continue 'part;
            } else if curr_flow.default == "R" {
                parts_to_acceptance.insert(part, false);
                curr_path.push("R");
                parts_to_paths.insert(part, curr_path);
                continue 'part;
            }

            curr_path.push(&curr_flow.default);
            curr_flow = workflows_by_name.get(&curr_flow.default).unwrap();
        }
    }

    // println!("Paths: {:?}", parts_to_paths);

    parts_to_acceptance
}

fn compute_acceptable_ranges(workflows: Vec<Workflow>) -> Vec<PartRange> {
    let workflows_by_name = workflows.into_iter().fold(HashMap::new(), |mut hmap, wf| {
        hmap.insert(wf.name.clone(), wf);
        hmap
    });

    let mut accepted_ranges = Vec::new();
    let mut stack = VecDeque::new();

    stack.push_front((
        workflows_by_name.get(&String::from("in")).unwrap(),
        PartRange::default(),
    ));

    while let Some((wf, range)) = stack.pop_front() {
        let mut curr_range = range.clone();

        for instruction in &wf.instructions {
            if instruction.operator == Operator::LessThan {
                let (bottom_range, top_range) = curr_range.split_at(
                    instruction.part_attribute,
                    instruction.value,
                    Operator::LessThan,
                );

                // Bottom range is destination
                curr_range = top_range;

                if instruction.destination == "A" {
                    accepted_ranges.push(bottom_range);
                } else if instruction.destination == "R" {
                    continue;
                } else {
                    stack.push_front((
                        workflows_by_name.get(&instruction.destination).unwrap(),
                        bottom_range,
                    ));
                }
            } else {
                let (bottom_range, top_range) = curr_range.split_at(
                    instruction.part_attribute,
                    instruction.value,
                    Operator::GreaterThan,
                );

                // Top range is destination
                curr_range = bottom_range;

                if instruction.destination == "A" {
                    accepted_ranges.push(top_range);
                } else if instruction.destination == "R" {
                    continue;
                } else {
                    stack.push_front((
                        workflows_by_name.get(&instruction.destination).unwrap(),
                        top_range,
                    ));
                }
            }
        }

        if wf.default == "A" {
            accepted_ranges.push(curr_range);
        } else if wf.default != "R" {
            stack.push_front((workflows_by_name.get(&wf.default).unwrap(), curr_range));
        }
    }

    for range in accepted_ranges.iter() {
        // println!("{:?}", range);
    }
    accepted_ranges
}

fn solve(lines: Vec<String>) -> u64 {
    let (workflows, parts) = parse_lines(lines);
    let results = run_workflows(workflows, parts);

    results.iter().fold(0, |acc, (part, accepted)| {
        if *accepted {
            acc + part.sum() as u64
        } else {
            acc
        }
    })
}

fn solve2(lines: Vec<String>) -> u64 {
    let (workflows, _) = parse_lines(lines);

    let accepted_ranges = compute_acceptable_ranges(workflows);

    accepted_ranges
        .iter()
        .map(|range| range.compute_combos())
        .sum()
}

fn main() {
    match filereader::read_file("./day19/resources/input.txt") {
        Ok(lines) => {
            let result = solve2(lines);
            println!("{:?}", result);
        }
        Err(e) => panic!("{}", e),
    }
}

#[cfg(test)]
mod tests {
    use crate::solve2;

    #[test]
    fn compute_acceptable_ranges_small() {
        let lines = vec![
            String::from("in{s<1351:px,R}"),
            String::from("px{a<2006:R,m>2090:A,rfg}"),
            String::from("rfg{s<537:R,x>2440:R,A}"),
            String::from(""),
            String::from("{x=787,m=2655,a=1222,s=2876}"),
        ];

        assert_eq!(solve2(lines), 28857823428000)
    }

    #[test]
    fn compute_test_example_ranges() {
        let lines = vec![
            String::from("px{a<2006:qkq,m>2090:A,rfg}"),
            String::from("pv{a>1716:R,A}"),
            String::from("lnx{m>1548:A,A}"),
            String::from("rfg{s<537:gd,x>2440:R,A}"),
            String::from("qs{s>3448:A,lnx}"),
            String::from("qkq{x<1416:A,crn}"),
            String::from("crn{x>2662:A,R}"),
            String::from("in{s<1351:px,qqz}"),
            String::from("qqz{s>2770:qs,m<1801:hdj,R}"),
            String::from("gd{a>3333:R,R}"),
            String::from("hdj{m>838:A,pv}"),
            String::from(""),
            String::from("{x=787,m=2655,a=1222,s=2876}"),
            String::from("{x=1679,m=44,a=2067,s=496}"),
            String::from("{x=2036,m=264,a=79,s=2244}"),
            String::from("{x=2461,m=1339,a=466,s=291}"),
            String::from("{x=2127,m=1623,a=2188,s=1013}"),
        ];

        assert_eq!(solve2(lines), 167409079868000)
    }
}
