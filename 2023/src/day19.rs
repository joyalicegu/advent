use crate::Solution;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

pub struct Day19;

#[derive(Copy, Clone, Debug)]
pub struct Part {
    x: isize,
    m: isize,
    a: isize,
    s: isize,
}

impl Part {
    fn get_category(&self, category: char) -> isize {
        match category {
            'x' => self.x,
            'm' => self.m,
            'a' => self.a,
            's' => self.s,
            _ => panic!(),
        }
    }

    fn set_category(&self, category: char, value: isize) -> Part {
        let mut modified = self.clone();
        match category {
            'x' => modified.x = value,
            'm' => modified.m = value,
            'a' => modified.a = value,
            's' => modified.s = value,
            _ => panic!(),
        }
        modified
    }
}

#[derive(Debug)]
pub struct Step {
    condition: Option<(char, char, isize)>,
    destination: String,
}

#[derive(Debug)]
pub struct Workflow {
    name: String,
    steps: Vec<Step>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseDay19Error;

impl FromStr for Part {
    type Err = ParseDay19Error;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let number_re = Regex::new(r"[0-9]+").unwrap();
        let (x, m, a, s) = number_re
            .find_iter(line)
            .map(|n| n.as_str().parse::<isize>().unwrap())
            .collect_tuple()
            .unwrap();
        Ok(Part {
            x: x,
            m: m,
            a: a,
            s: s,
        })
    }
}

impl FromStr for Step {
    type Err = ParseDay19Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((cond_str, dest)) = s.split_once(":") {
            Ok(Step {
                condition: Some((
                    cond_str.chars().nth(0).unwrap(),
                    cond_str.chars().nth(1).unwrap(),
                    cond_str[2..].parse::<isize>().unwrap(),
                )),
                destination: dest.to_string(),
            })
        } else {
            Ok(Step {
                condition: None,
                destination: s.to_string(),
            })
        }
    }
}

impl FromStr for Workflow {
    type Err = ParseDay19Error;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (name, steps_str, _) = line.split(&['{', '}']).collect_tuple().unwrap();
        let steps = steps_str
            .split(',')
            .map(|step_str| step_str.parse::<Step>().unwrap())
            .collect();
        Ok(Workflow {
            name: name.to_string(),
            steps: steps,
        })
    }
}

impl Day19 {
    fn apply_condition(part: Part, condition: Option<(char, char, isize)>) -> bool {
        if let Some((category, compare_op, value)) = condition {
            let rating = part.get_category(category);
            return match compare_op {
                '>' => rating > value,
                '<' => rating < value,
                _ => panic!(),
            };
        }
        return true;
    }

    fn process_part(workflows: &HashMap<String, Workflow>, part: Part, start: String) -> bool {
        let mut state = &start;
        loop {
            let steps = &workflows.get(state).unwrap().steps;
            for step in steps.iter() {
                if Self::apply_condition(part, step.condition) {
                    state = &step.destination;
                    if state == "A" {
                        return true;
                    } else if state == "R" {
                        return false;
                    }
                    break;
                }
            }
        }
    }

    fn apply_condition_to_range(
        part_range: (Part, Part),
        condition: Option<(char, char, isize)>,
    ) -> (Option<(Part, Part)>, Option<(Part, Part)>) {
        let Some((category, compare_op, value)) = condition else {
            return (Some(part_range), None);
        };
        let mid = if compare_op == '>' { value + 1 } else { value };
        let (lo, hi) = part_range;

        // really not sure why i can get away with this
        assert!(mid - 1 <= hi.get_category(category));
        assert!(mid >= lo.get_category(category));
        assert!(lo.get_category(category) <= mid - 1);
        assert!(mid <= hi.get_category(category));

        let lo_range = Some((lo, hi.set_category(category, mid - 1)));
        let hi_range = Some((lo.set_category(category, mid), hi));
        if compare_op == '>' {
            (hi_range, lo_range)
        } else {
            (lo_range, hi_range)
        }
    }

    fn accepted_part_ranges(
        workflows: &HashMap<String, Workflow>,
        part_range: (Part, Part),
        start: String,
    ) -> Vec<(Part, Part)> {
        let mut to_process = Vec::from([(&start, part_range)]);
        let mut accepted_ranges = Vec::new();
        while let Some((state, mut range)) = to_process.pop() {
            let steps = &workflows.get(state).unwrap().steps;
            for step in steps.iter() {
                let (going, staying) = Self::apply_condition_to_range(range, step.condition);
                if let Some(next_range) = going {
                    let next_state = &step.destination;
                    if next_state == "A" {
                        accepted_ranges.push(next_range);
                    } else if next_state != "R" {
                        to_process.push((next_state, next_range));
                    }
                }
                if let Some(next_range) = staying {
                    range = next_range;
                } else {
                    break;
                }
            }
        }
        accepted_ranges
    }
}

impl Solution for Day19 {
    type ParsedInput = (HashMap<String, Workflow>, Vec<Part>);

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        let (workflows_str, parts_str) = input_lines.split_once("\n\n").unwrap();
        let workflows = workflows_str
            .lines()
            .map(|l| l.parse::<Workflow>().unwrap())
            .map(|w| (w.name.to_string(), w))
            .collect::<HashMap<String, Workflow>>();
        let parts = parts_str
            .lines()
            .map(|l| l.parse::<Part>().unwrap())
            .collect();
        (workflows, parts)
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        let (workflows, parts) = _parsed_input;
        parts
            .iter()
            .filter(|&part| Self::process_part(&workflows, *part, "in".to_string()))
            .map(|part| part.x + part.m + part.a + part.s)
            .sum::<isize>()
            .to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        let (workflows, _) = _parsed_input;
        let start = "in".to_string();
        let part_range = (
            Part {
                x: 1,
                m: 1,
                a: 1,
                s: 1,
            },
            Part {
                x: 4000,
                m: 4000,
                a: 4000,
                s: 4000,
            },
        );
        Self::accepted_part_ranges(&workflows, part_range, start)
            .iter()
            .map(|&(min_part, max_part)| {
                (max_part.x - min_part.x + 1)
                    * (max_part.m - min_part.m + 1)
                    * (max_part.a - min_part.a + 1)
                    * (max_part.s - min_part.s + 1)
            })
            .sum::<isize>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn check_day19_part1_case1() {
        assert_eq!(Day19::solve_part_one(TEST_INPUT), "19114".to_string())
    }

    #[test]
    fn check_day19_part2_case1() {
        assert_eq!(
            Day19::solve_part_two(TEST_INPUT),
            "167409079868000".to_string()
        )
    }
}
