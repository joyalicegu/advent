use crate::Solution;
use std::collections::{HashMap, VecDeque};

pub struct Day20;

#[derive(Debug, PartialEq, Eq)]
pub enum ModuleState {
    Broadcast,
    FlipFlop { on: bool },
    Conjunction { inputs: HashMap<String, bool> },
}

impl Day20 {
    fn simulate(
        module_states: &mut HashMap<String, ModuleState>,
        module_dests: &HashMap<String, Vec<String>>,
        presses: usize,
    ) -> Vec<bool> {
        let mut sent = Vec::new();
        for _ in 0..presses {
            let mut queue = VecDeque::from([(false, "".to_string(), "broadcaster".to_string())]);
            sent.push(false);
            while let Some((pulse, from, current)) = queue.pop_front() {
                let Some(state) = module_states.get_mut(&current) else {
                    continue;
                };
                let Some(dests) = module_dests.get(&current) else {
                    continue;
                };
                let new_pulse = match state {
                    ModuleState::Broadcast => pulse,
                    ModuleState::FlipFlop { ref mut on } => {
                        if pulse {
                            continue;
                        }
                        *on = !*on;
                        *on
                    }
                    ModuleState::Conjunction { ref mut inputs } => {
                        inputs.insert(from, pulse);
                        !inputs.values().all(|&b| b)
                    }
                };
                for dest in dests.iter() {
                    queue.push_back((new_pulse, current.to_string(), dest.to_string()));
                    sent.push(new_pulse);
                }
            }
        }
        sent
    }
}

impl Solution for Day20 {
    type ParsedInput = (HashMap<String, ModuleState>, HashMap<String, Vec<String>>);

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        let (mut module_states, module_dests): (
            HashMap<String, ModuleState>,
            HashMap<String, Vec<String>>,
        ) = input_lines
            .lines()
            .map(|line| {
                let (module_str, dest_str) = line.split_once(" -> ").unwrap();
                let module_state = match module_str.chars().nth(0).unwrap() {
                    'b' => ModuleState::Broadcast,
                    '%' => ModuleState::FlipFlop { on: false },
                    '&' => ModuleState::Conjunction {
                        inputs: HashMap::new(),
                    },
                    _ => panic!("Day 20: Invalid module type"),
                };
                let module_name = if module_state == ModuleState::Broadcast {
                    &module_str[..]
                } else {
                    &module_str[1..]
                };
                let destination_modules = dest_str.split(", ").map(|s| s.to_string()).collect();
                (
                    (module_name.to_string(), module_state),
                    (module_name.to_string(), destination_modules),
                )
            })
            .unzip();
        // populate conjunction inputs
        for (input, dests) in module_dests.iter() {
            for dest in dests.iter() {
                if let Some(ModuleState::Conjunction { ref mut inputs }) =
                    module_states.get_mut(dest)
                {
                    inputs.insert(input.clone(), false);
                }
            }
        }
        (module_states, module_dests)
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        let (ref mut module_states, module_dests) = _parsed_input;
        let pulses = Self::simulate(module_states, module_dests, 1000);
        let low_pulses = pulses.iter().filter(|&p| *p == false).count();
        let high_pulses = pulses.iter().filter(|&p| *p == true).count();
        (low_pulses * high_pulses).to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        "0".to_string() // TODO
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day20_part1_case1() {
        assert_eq!(
            Day20::solve_part_one(
                "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a"
            ),
            "32000000".to_string()
        )
    }

    #[test]
    fn check_day20_part1_case2() {
        assert_eq!(
            Day20::solve_part_one(
                "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output"
            ),
            "11687500".to_string()
        )
    }
}
