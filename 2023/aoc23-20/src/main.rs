use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
};

use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    Problem::go()
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
enum ModuleType {
    Broadcaster,
    FlipFlop,
    Conjunction,
}

use ModuleType::*;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
enum Pulse {
    Low,
    High,
}

use Pulse::*;

struct Module {
    name: String,
    module_type: ModuleType,
    destinations: Vec<String>,
}

struct Problem {
    modules: Vec<Module>,
}

struct System<'a> {
    modules: HashMap<&'a str, &'a Module>,
    ff_states: HashMap<&'a str, bool>,
    con_states: HashMap<&'a str, HashMap<&'a str, Pulse>>,
    low_pulse_count: usize,
    high_pulse_count: usize,
    push_count: usize,
    trackers: HashMap<&'a str, usize>,
}

impl<'a> System<'a> {
    fn new(modules: &'a [Module]) -> Self {
        let modules: HashMap<&str, &Module> =
            modules.iter().map(|m| (m.name.as_str(), m)).collect();
        let ff_states = modules
            .values()
            .filter_map(|m| {
                if m.module_type == FlipFlop {
                    Some((m.name.as_str(), false))
                } else {
                    None
                }
            })
            .collect();
        let con_states = modules
            .values()
            .filter_map(|m| {
                if m.module_type == Conjunction {
                    Some((
                        m.name.as_str(),
                        modules
                            .values()
                            .filter_map(|input_module| {
                                if input_module.destinations.contains(&m.name) {
                                    Some((input_module.name.as_str(), Low))
                                } else {
                                    None
                                }
                            })
                            .collect(),
                    ))
                } else {
                    None
                }
            })
            .collect();
        Self {
            modules,
            ff_states,
            con_states,
            low_pulse_count: 0,
            high_pulse_count: 0,
            push_count: 0,
            trackers: HashMap::new(),
        }
    }

    fn track_module_cycle(&mut self, module_name: &'a str) {
        self.trackers.insert(module_name, 0);
    }

    fn push_button(&mut self) {
        self.push_count += 1;
        let mut signals = VecDeque::new();
        signals.push_back(("button", "broadcaster", Low));
        while let Some((source, target_name, pulse_in)) = signals.pop_front() {
            match pulse_in {
                Low => self.low_pulse_count += 1,
                High => self.high_pulse_count += 1,
            };
            if !self.modules.contains_key(target_name) {
                continue;
            }
            let target = &self.modules[target_name];
            let pulse_out = match (target.module_type, pulse_in) {
                (Broadcaster, _) => Some(Low),
                (FlipFlop, High) => None,
                (FlipFlop, Low) => {
                    let on = self.ff_states[target_name];
                    self.ff_states.insert(target_name, !on);
                    if on {
                        Some(Low)
                    } else {
                        Some(High)
                    }
                }
                (Conjunction, pulse) => {
                    self.con_states
                        .get_mut(target_name)
                        .unwrap()
                        .insert(source, pulse);
                    if self.con_states[target_name].values().all(|&p| p == High) {
                        Some(Low)
                    } else {
                        Some(High)
                    }
                }
            };

            if let Some(pulse) = pulse_out {
                for destination in &target.destinations {
                    if pulse == Low && self.trackers.get(destination.as_str()).is_some() {
                        self.trackers.insert(destination, self.push_count);
                    }

                    signals.push_back((target_name, destination.as_str(), pulse));
                }
            }
        }
    }
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        Ok(Self {
            modules: contents.parse_lines()?,
        })
    }
}

impl FromStr for Module {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        let (name_and_type, destinations) = contents.split_once(" -> ").ok_or_invalid()?;

        let (name, module_type) = if name_and_type == "broadcaster" {
            (name_and_type.to_string(), Broadcaster)
        } else if let Some(name) = name_and_type.strip_prefix('%') {
            (name.to_string(), FlipFlop)
        } else if let Some(name) = name_and_type.strip_prefix('&') {
            (name.to_string(), Conjunction)
        } else {
            bail!("invalid module line");
        };

        let destinations = destinations
            .split(',')
            .map(|d| d.trim().to_string())
            .collect();

        Ok(Self {
            name,
            module_type,
            destinations,
        })
    }
}

impl Solution for Problem {
    type Part1 = usize;
    type Part2 = usize;

    fn part1(&mut self) -> Result<Self::Part1> {
        let mut system = System::new(&self.modules);

        for _ in 0..1000 {
            system.push_button();
        }

        Ok(system.low_pulse_count * system.high_pulse_count)
    }

    fn part2(&self) -> Result<Self::Part2> {
        let mut system = System::new(&self.modules);

        for rx_source in self
            .modules
            .iter()
            .filter(|module| module.destinations.contains(&"rx".to_string()))
        {
            for rx_source_source in self
                .modules
                .iter()
                .filter(|module| module.destinations.contains(&rx_source.name))
            {
                system.track_module_cycle(rx_source_source.name.as_str());
            }
        }

        while system.trackers.values().any(|&v| v == 0) {
            system.push_button();
        }

        Ok(least_common_multiple(
            &system.trackers.values().copied().collect::<Vec<_>>(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_sample_part1() -> Result<()> {
        let mut problem = Problem::from_str(SIMPLE_SAMPLE)?;

        let result = problem.part1()?;

        assert_eq!(32000000, result);

        Ok(())
    }

    #[test]
    fn complex_sample_part1() -> Result<()> {
        let mut problem = Problem::from_str(COMPLEX_SAMPLE)?;

        let result = problem.part1()?;

        assert_eq!(11687500, result);

        Ok(())
    }

    const SIMPLE_SAMPLE: &str = "\
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
    const COMPLEX_SAMPLE: &str = "\
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";
}
