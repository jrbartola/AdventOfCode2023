mod pulse;

use crate::pulse::PulseLevel::{High, Low};
use crate::pulse::{
    BroadcasterModule, ConjunctionModule, FlipFlopModule, Mediator, Module, ModuleType,
    CRUCIAL_MODULES,
};
use common::filereader;
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::thread;
use std::time::Duration;

fn parse_lines(lines: Vec<String>) -> Vec<Box<dyn Module>> {
    let line_regex = Regex::new(r"(%|&)?([a-z]+) -> (.*)").unwrap();

    let mut modules: Vec<Box<dyn Module>> = Vec::new();
    let mut modules_by_sources = HashMap::new();

    for line in lines.iter() {
        if let Some(captures) = line_regex.captures(line) {
            let maybe_type = captures.get(1).map(|r| r.as_str());
            let name = captures.get(2).unwrap().as_str();
            let destinations = captures
                .get(3)
                .unwrap()
                .as_str()
                .split(", ")
                .map(|s| s.to_owned())
                .collect::<Vec<String>>();

            match maybe_type {
                Some("%") => {
                    destinations.iter().for_each(|d| {
                        modules_by_sources
                            .entry(d.clone())
                            .or_insert(vec![])
                            .push(name)
                    });

                    modules.push(Box::new(FlipFlopModule::new(
                        name.to_string(),
                        destinations,
                    )));
                }
                Some("&") => {
                    destinations.iter().for_each(|d| {
                        modules_by_sources
                            .entry(d.clone())
                            .or_insert(vec![])
                            .push(name)
                    });

                    modules.push(Box::new(ConjunctionModule::new(
                        name.to_string(),
                        destinations,
                        vec![],
                    )));
                }
                None => {
                    if name != "broadcaster" {
                        panic!("Only broadcaster should have no module type");
                    }

                    destinations.iter().for_each(|d| {
                        modules_by_sources
                            .entry(d.clone())
                            .or_insert(vec![])
                            .push(name)
                    });

                    modules.push(Box::new(BroadcasterModule::new(
                        name.to_string(),
                        destinations,
                    )));
                }
                _ => unreachable!(),
            }
        }
    }

    for i in 0..modules.len() {
        match modules[i].get_type() {
            ModuleType::Conjunction => {
                modules[i] = Box::new(ConjunctionModule::new(
                    modules[i].get_name(),
                    modules[i].get_destinations().clone(),
                    modules_by_sources
                        .get(&modules[i].get_name())
                        .unwrap()
                        .clone()
                        .iter()
                        .map(|&s| s.to_string())
                        .collect(),
                ));
            }
            _ => {}
        }
    }

    modules
}

fn print_module_states(modules: &Vec<&Box<dyn Module>>) {
    let flipflops: Vec<_> = modules
        .iter()
        .filter(|m| m.get_type() == ModuleType::FlipFlop)
        .map(|m| m.as_any().downcast_ref::<FlipFlopModule>().unwrap())
        .collect();

    let conjunctions: Vec<_> = modules
        .iter()
        .filter(|m| m.get_type() == ModuleType::Conjunction)
        .map(|m| m.as_any().downcast_ref::<ConjunctionModule>().unwrap())
        .collect();

    flipflops.iter().for_each(|f| println!("{:?}", f));
    conjunctions.iter().for_each(|c| println!("{:?}", c));
}

fn print_module_state(modules: &Vec<&Box<dyn Module>>, module: String) {
    if let Some(matching_module) = modules.iter().find(|m| m.get_name() == module) {
        match matching_module.get_type() {
            ModuleType::FlipFlop => {
                let casted = matching_module
                    .as_any()
                    .downcast_ref::<FlipFlopModule>()
                    .unwrap();
                println!("{:?}", casted);
            }
            ModuleType::Conjunction => {
                let casted = matching_module
                    .as_any()
                    .downcast_ref::<ConjunctionModule>()
                    .unwrap();
                println!("{:?}", casted);
            }
            ModuleType::Broadcaster => {
                let casted = matching_module
                    .as_any()
                    .downcast_ref::<BroadcasterModule>()
                    .unwrap();
                println!("{:?}", casted);
            }
        };
    }
}

fn find_all_nodes_on_path_to_module(
    modules: HashMap<String, Box<dyn Module>>,
    start_module: String,
    goal_module: String,
) -> HashSet<String> {
    let mut relevant: HashSet<String> = HashSet::new();
    let mut path_stack: VecDeque<(String, Vec<String>)> = VecDeque::new();
    path_stack.push_back((start_module, Vec::new()));

    while let Some((name, path)) = path_stack.pop_front() {
        println!("{:?}: {:?}", name, path);
        if path.contains(&name) {
            continue;
        }

        if goal_module == name.as_str() {
            path.iter().for_each(|p| {
                relevant.insert(p.clone());
            });

            relevant.insert(name);
            continue;
        }

        for dest in modules.get(&name).unwrap().get_destinations() {
            let mut updated_path = path.clone();
            updated_path.push(name.to_owned());
            path_stack.push_back((dest.to_string(), updated_path));
        }
    }

    relevant
}

fn solve(lines: Vec<String>) -> u32 {
    let modules = parse_lines(lines);

    let mut mediator = Mediator::new(modules);

    for _ in 0..1000 {
        mediator.send_pulse(String::from("broadcaster"), Low);
    }

    print_module_states(&mediator.modules.values().collect());

    let pulses = mediator.get_pulses();

    pulses.values().product()
}

fn solve2(lines: Vec<String>) -> u32 {
    let modules = parse_lines(lines);

    let mut mediator = Mediator::new(modules);
    let mut i = 1_u64;

    loop {
        println!("----------Button press {i}----------");
        mediator.send_pulse(String::from("broadcaster"), Low);

        i += 1;
    }

    234
}

fn main() {
    match filereader::read_file("./day20/resources/input.txt") {
        Ok(lines) => {
            let result = solve2(lines);
            println!("{:?}", result);
        }
        Err(e) => panic!("{}", e),
    }
}
