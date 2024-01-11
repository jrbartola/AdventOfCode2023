use crate::pulse::FlipFlopState::{Off, On};
use crate::pulse::ModuleType::{Broadcaster, Conjunction, FlipFlop};
use crate::pulse::PulseLevel::{High, Low};
use std::any::Any;
use std::collections::{HashMap, VecDeque};

const BUTTON_SOURCE: &str = "button";
pub const CRUCIAL_MODULES: [&str; 4] = ["vz", "bq", "qh", "lt"];

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub(crate) enum PulseLevel {
    High,
    Low,
}

impl PulseLevel {
    pub fn flip(&self) -> PulseLevel {
        match self {
            High => Low,
            Low => High,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) struct Pulse {
    source: String,
    destination: String,
    level: PulseLevel,
}

impl Pulse {
    pub fn new(source: String, destination: String, level: PulseLevel) -> Self {
        Pulse {
            source,
            destination,
            level,
        }
    }
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

#[derive(Eq, PartialEq)]
pub(crate) enum ModuleType {
    Broadcaster,
    FlipFlop,
    Conjunction,
}

pub(crate) trait Module: Any {
    fn as_any(&self) -> &dyn Any;

    fn get_name(&self) -> String;

    fn get_destinations(&self) -> &Vec<String>;

    fn receive_pulse(&mut self, source: String, pulse_level: PulseLevel) -> Vec<Pulse>;

    fn get_type(&self) -> ModuleType;
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) struct BroadcasterModule {
    name: String,
    destinations: Vec<String>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) struct FlipFlopModule {
    name: String,
    destinations: Vec<String>,
    state: FlipFlopState,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) struct ConjunctionModule {
    name: String,
    destinations: Vec<String>,
    pub sources: HashMap<String, PulseLevel>,
    sources_to_pulses: HashMap<String, u64>,
}

impl BroadcasterModule {
    pub fn new(name: String, destinations: Vec<String>) -> Self {
        BroadcasterModule { name, destinations }
    }
}

impl Module for BroadcasterModule {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_name(&self) -> String {
        self.name.to_owned()
    }

    fn get_destinations(&self) -> &Vec<String> {
        &self.destinations
    }

    fn receive_pulse(&mut self, source: String, pulse_level: PulseLevel) -> Vec<Pulse> {
        self.destinations
            .iter()
            .map(|d| Pulse::new(self.get_name(), d.to_owned(), pulse_level))
            .collect()
    }

    fn get_type(&self) -> ModuleType {
        Broadcaster
    }
}

impl FlipFlopModule {
    pub fn new(name: String, destinations: Vec<String>) -> Self {
        FlipFlopModule {
            name,
            destinations,
            state: Off,
        }
    }

    pub fn flip(&mut self) {
        self.state = self.state.flip();
    }
}

impl Module for FlipFlopModule {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_name(&self) -> String {
        self.name.to_owned()
    }

    fn get_destinations(&self) -> &Vec<String> {
        &self.destinations
    }

    fn receive_pulse(&mut self, source: String, pulse_level: PulseLevel) -> Vec<Pulse> {
        if pulse_level == Low {
            self.state = self.state.flip();

            self.destinations
                .iter()
                .map(|d| {
                    Pulse::new(
                        self.get_name(),
                        d.to_owned(),
                        if self.state == On { High } else { Low },
                    )
                })
                .collect()
        } else {
            Vec::new()
        }
    }

    fn get_type(&self) -> ModuleType {
        FlipFlop
    }
}

impl ConjunctionModule {
    pub fn new(name: String, destinations: Vec<String>, sources: Vec<String>) -> Self {
        ConjunctionModule {
            name,
            destinations,
            sources: sources
                .clone()
                .into_iter()
                .fold(HashMap::new(), |mut acc, s| {
                    acc.insert(s, Low);
                    acc
                }),
            sources_to_pulses: sources.into_iter().fold(HashMap::new(), |mut acc, s| {
                acc.insert(s, 0);
                acc
            }),
        }
    }
}

impl Module for ConjunctionModule {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_name(&self) -> String {
        self.name.to_owned()
    }

    fn get_destinations(&self) -> &Vec<String> {
        &self.destinations
    }

    fn receive_pulse(&mut self, source: String, pulse_level: PulseLevel) -> Vec<Pulse> {
        self.sources_to_pulses
            .entry(source.clone())
            .and_modify(|e| *e += 1);

        // if self.get_name() == "ft" && pulse_level == High
        // {
        //     println!(
        //         "Got a HIGH pulse level from {source}. Full sources: {:?}",
        //         self.sources
        //     );
        // }

        if source != BUTTON_SOURCE {
            self.sources.insert(source, pulse_level);
        }

        let pulse_to_send = if self.sources.values().all(|&v| v == High) {
            Low
        } else {
            High
        };

        self.destinations
            .iter()
            .map(|d| Pulse::new(self.get_name(), d.to_owned(), pulse_to_send))
            .collect()
    }

    fn get_type(&self) -> ModuleType {
        Conjunction
    }
}

pub(crate) struct Mediator {
    pub modules: HashMap<String, Box<dyn Module>>,
    pulses: HashMap<PulseLevel, u32>,
}

impl Mediator {
    pub fn new(modules: Vec<Box<dyn Module>>) -> Self {
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

        pulse_queue.push_back(Pulse::new(
            String::from(BUTTON_SOURCE),
            module.clone(),
            pulse_level,
        ));

        while let Some(Pulse {
            source,
            destination,
            level,
        }) = pulse_queue.pop_front()
        {
            *self.pulses.entry(level).or_insert(0) += 1;
            // It's possible for the destination module not to exist in our mappings. Just ignore it
            if let Some(module_obj) = self.modules.get_mut(destination.as_str()) {
                let outbound_pulses = module_obj.receive_pulse(source, level);

                for pulse in outbound_pulses {
                    pulse_queue.push_back(pulse)
                }
            }
        }
    }

    pub fn get_pulses(&self) -> &HashMap<PulseLevel, u32> {
        &self.pulses
    }
}
