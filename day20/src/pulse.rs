use crate::pulse::FlipFlopState::Off;
use crate::pulse::PulseLevel::{High, Low};
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub(crate) enum PulseLevel {
    High,
    Low,
}

impl PulseLevel {
    pub fn flip(&self) -> PulseLevel {
        match self {
            PulseLevel::High => PulseLevel::Low,
            PulseLevel::Low => PulseLevel::High,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) struct Pulse {
    level: PulseLevel,
    destination: String,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) enum FlipFlopState {
    On,
    Off,
}

impl FlipFlopState {
    pub fn flip(&self) -> FlipFlopState {
        match self {
            FlipFlopState::On => FlipFlopState::Off,
            FlipFlopState::Off => FlipFlopState::On,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) enum Module {
    Broadcaster {
        name: String,
        destinations: Vec<String>,
    },
    FlipFlop {
        name: String,
        destinations: Vec<String>,
        state: FlipFlopState,
    },
    Conjunction {
        name: String,
        destinations: Vec<String>,
        sources: Vec<String>,
    },
}

impl Module {
    pub fn get_name(&self) -> &str {
        match self {
            Module::Broadcaster { name, .. } => name,
            Module::FlipFlop { name, .. } => name,
            Module::Conjunction { name, .. } => name,
        }
    }

    pub fn receive_pulse(&self, pulse_level: PulseLevel) -> Vec<(String, PulseLevel)> {
        match self {
            Module::Broadcaster { destinations, .. } => destinations
                .iter()
                .map(|d| (d.to_owned(), pulse_level))
                .collect(),
            Module::FlipFlop {
                state,
                destinations,
                ..
            } => {
                if pulse_level == Low {
                    let pulse_level_to_send = if state == Off { High } else { Low };
                    destinations
                        .iter()
                        .map(|d| (d.to_owned(), pulse_level_to_send))
                        .collect()
                } else {
                    Vec::new()
                }
            }
            Module::Conjunction { .. } => {}
        }
    }
}

pub(crate) struct Mediator {
    modules: HashMap<String, Module>,
    pulses: HashMap<PulseLevel, u32>,
}

impl Mediator {
    pub fn new(modules: Vec<Module>) -> Self {
        Mediator {
            modules: modules.into_iter().fold(HashMap::new(), |mut acc, module| {
                acc.insert(module.get_name().to_owned(), module);
                acc
            }),
            pulses: HashMap::new(),
        }
    }

    pub fn send_pulse(&mut self, module: String, pulse_level: PulseLevel) {
        let mut pulse_queue = VecDeque::new();
        pulse_queue.push_back((module, pulse_level));

        while let Some((module, pulse_level)) = pulse_queue.pop_front() {
            let module_obj = self.modules.get(module.as_str()).unwrap();

            match module_obj {
                Module::Broadcaster { destinations, .. } => {
                    for dest in destinations {
                        pulse_queue.push_back((dest.to_owned(), pulse_level));
                    }
                }
                Module::FlipFlop {
                    name,
                    destinations,
                    state,
                } => {
                    if pulse_level == Low {
                        self.modules.insert(
                            module,
                            Module::FlipFlop {
                                name: name.to_owned(),
                                destinations: destinations.clone(),
                                state: state.flip(),
                            },
                        )
                    }
                }
                Module::Conjunction {
                    destinations,
                    sources,
                } => {}
            }
        }
    }

    pub fn get_pulses(&self) -> &HashMap<PulseLevel, u32> {
        &self.pulses
    }
}
