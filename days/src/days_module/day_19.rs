use crate::days_module::day::Day;
use std::collections::{HashMap, VecDeque};
use std::ops::Range;
use std::str::FromStr;

#[derive(Clone)]
struct MachinePartSpace {
    x: Range<usize>,
    m: Range<usize>,
    a: Range<usize>,
    s: Range<usize>,
}

impl MachinePartSpace {
    fn sum(&self) -> usize {
        self.x.len() * self.m.len() * self.a.len() * self.s.len()
    }

    fn access(&self, char_: &char) -> Range<usize> {
        match char_ {
            'x' => self.x.clone(),
            'm' => self.m.clone(),
            'a' => self.a.clone(),
            's' => self.s.clone(),
            _ => panic!("Invalid property {}", char_),
        }
    }

    fn copy_with(&self, char_: &char, value: Range<usize>) -> MachinePartSpace {
        match char_ {
            'x' => self.with_x(value),
            'm' => self.with_m(value),
            'a' => self.with_a(value),
            's' => self.with_s(value),
            _ => panic!("Invalid property {}", char_),
        }
    }

    // Function to create a new instance with the 'x' attribute overridden
    fn with_x(&self, x: Range<usize>) -> MachinePartSpace {
        let mut cloned = self.clone();
        cloned.x = x;
        cloned
    }

    // Function to create a new instance with the 'm' attribute overridden
    fn with_m(&self, m: Range<usize>) -> MachinePartSpace {
        let mut cloned = self.clone();
        cloned.m = m;
        cloned
    }

    // Function to create a new instance with the 'a' attribute overridden
    fn with_a(&self, a: Range<usize>) -> MachinePartSpace {
        let mut cloned = self.clone();
        cloned.a = a;
        cloned
    }

    // Function to create a new instance with the 's' attribute overridden
    fn with_s(&self, s: Range<usize>) -> MachinePartSpace {
        let mut cloned = self.clone();
        cloned.s = s;
        cloned
    }
}

struct MachinePart {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl MachinePart {
    fn sum(&self) -> usize {
        self.x + self.m + self.a + self.s
    }

    fn access(&self, char_: &char) -> usize {
        match char_ {
            'x' => self.x,
            'm' => self.m,
            'a' => self.a,
            's' => self.s,
            _ => panic!("Invalid property {}", char_),
        }
    }
}

impl FromStr for MachinePart {
    type Err = ();

    fn from_str(input: &str) -> Result<MachinePart, Self::Err> {
        let cleaned_raw_part = input
            .replace("{", "")
            .replace("}", "")
            .replace("x=", "")
            .replace("m=", "")
            .replace("a=", "")
            .replace("s=", "");
        let mut xmas_split = cleaned_raw_part.split(",");
        return Ok(MachinePart {
            x: xmas_split.next().unwrap().parse().unwrap(),
            m: xmas_split.next().unwrap().parse().unwrap(),
            a: xmas_split.next().unwrap().parse().unwrap(),
            s: xmas_split.next().unwrap().parse().unwrap(),
        });
    }
}

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
struct Rule {
    property: char,
    sign: char,
    value: usize,
}

impl Rule {
    fn test(&self, value: &usize) -> bool {
        match self.sign {
            '<' => value < &self.value,
            '>' => value > &self.value,
            _ => panic!("Invalid operator {}", self.sign),
        }
    }

    // First value is the matching range, second is the non-matching range.
    fn test_range(&self, input_range: Range<usize>) -> (Range<usize>, Range<usize>) {
        let mut split_value = self.value;

        // Because ranges are exclusive at the end, adjust the returned ranged in case of `>`.
        if self.sign == '>' {
            split_value += 1;
        }

        let first_range;
        let second_range;
        if split_value < input_range.start {
            // Split value is before the input range, return an empty range and the original range.
            first_range = input_range.start..input_range.start;
            second_range = input_range;
        } else if split_value >= input_range.end {
            // Split value is after or at the end of the input range, return the original range and
            // an empty range.
            first_range = input_range.clone();
            second_range = input_range.end..input_range.end;
        } else {
            // Split value is within the input range, create two ranges accordingly.
            first_range = input_range.start..split_value;
            second_range = split_value..input_range.end;
        }
        match self.sign {
            '<' => (first_range, second_range),
            '>' => (second_range, first_range),
            _ => panic!("Invalid operator {}", self.sign),
        }
    }
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(input: &str) -> Result<Rule, Self::Err> {
        let sign: char;
        if input.contains("<") {
            sign = '<';
        } else if input.contains(">") {
            sign = '>';
        } else {
            return Err(());
        }

        let mut split = input.split(sign);
        let property = split.next().unwrap().chars().next().unwrap();
        let value: usize = split.next().unwrap().parse().unwrap();
        return Ok(Rule {
            property,
            sign,
            value,
        });
    }
}

struct WorkflowRule {
    name: String,
    rules: Vec<(char, Rule)>,
    rule_result_map: HashMap<Rule, Result<bool, String>>,
    default: Result<bool, String>,
}

impl WorkflowRule {
    fn add_rule(&mut self, char_: char, rule: Rule, result: Result<bool, String>) {
        self.rules.push((char_, rule.clone()));
        self.rule_result_map.insert(rule, result);
    }

    fn test_part(&self, part: &MachinePart, workflow_map: &HashMap<String, WorkflowRule>) -> bool {
        for (property, rule) in &self.rules {
            if rule.test(&part.access(&property)) {
                return match self.rule_result_map.get(&rule).unwrap() {
                    Ok(accepted) => *accepted,
                    Err(other_rule) => workflow_map
                        .get(other_rule)
                        .unwrap()
                        .test_part(part, workflow_map),
                };
            }
        }
        return match &self.default {
            Ok(accepted) => *accepted,
            Err(other_rule) => workflow_map
                .get(other_rule)
                .unwrap()
                .test_part(part, workflow_map),
        };
    }

    // Return the accepted spaces.
    fn test_space(
        &self,
        space: MachinePartSpace,
        workflow_map: &HashMap<String, WorkflowRule>,
    ) -> (Vec<MachinePartSpace>, Vec<MachinePartSpace>) {
        let mut remaining_spaces = vec![space];
        let mut accepted_spaces = Vec::new();
        let mut rejected_spaces = Vec::new();

        for (property, rule) in &self.rules {
            let mut new_remaining_spaces = Vec::new();

            for remaining_space in remaining_spaces {
                let (target_range, remain_range) =
                    rule.test_range(remaining_space.access(property));

                new_remaining_spaces.push(remaining_space.copy_with(property, remain_range));

                match self.rule_result_map.get(&rule).unwrap() {
                    Ok(true) => {
                        accepted_spaces.push(remaining_space.copy_with(property, target_range))
                    }
                    Ok(false) => {
                        rejected_spaces.push(remaining_space.copy_with(property, target_range))
                    }
                    Err(other_rule) => {
                        let rule = workflow_map.get(other_rule).unwrap();

                        let (accepted, rejected) = rule.test_space(
                            remaining_space.copy_with(property, target_range),
                            workflow_map,
                        );
                        accepted_spaces.extend(accepted.iter().map(|c| c.clone()));
                        rejected_spaces.extend(rejected.iter().map(|c| c.clone()));
                    }
                };
            }

            remaining_spaces = new_remaining_spaces;
        }

        match &self.default {
            Ok(true) => accepted_spaces.extend(remaining_spaces.iter().map(|c| c.clone())),
            Ok(false) => rejected_spaces.extend(remaining_spaces.iter().map(|c| c.clone())),
            Err(other_rule) => {
                let rule = workflow_map.get(other_rule).unwrap();

                for remaining_space in remaining_spaces {
                    let (accepted, rejected) = rule.test_space(remaining_space, workflow_map);
                    accepted_spaces.extend(accepted.iter().map(|c| c.clone()));
                    rejected_spaces.extend(rejected.iter().map(|c| c.clone()));
                }
            }
        };

        return (accepted_spaces, rejected_spaces);
    }
}

// Parser from hell, never look at this again.
fn parse(input: &String) -> (HashMap<String, WorkflowRule>, Vec<MachinePart>) {
    let mut input_split = input.split("\n\n");
    let raw_rule = input_split.next().unwrap();
    let raw_parts = input_split.next().unwrap();

    // Parse the raw workflow rules into `WorkflowRule` objects.
    let mut cache: HashMap<String, WorkflowRule> = HashMap::new();
    let mut raw_workflow_rule_queue = raw_rule
        .split("\n")
        .map(|s| s.to_string())
        .collect::<VecDeque<String>>();
    while !raw_workflow_rule_queue.is_empty() {
        let raw_workflow_rule = raw_workflow_rule_queue.pop_front().unwrap();
        let mut workflow_rule;

        // Parse name out.
        let mut name_split = raw_workflow_rule.split("{");
        let name = name_split.next().unwrap().to_string();
        let rest = name_split.next().unwrap().replace("}", "");

        let rest_raw_strings = rest.split(",").collect::<Vec<&str>>();

        // Parse default.
        let default_string = rest_raw_strings[rest_raw_strings.len() - 1];

        let default;

        if default_string == "A" {
            default = Ok(true);
        } else if default_string == "R" {
            default = Ok(false);
        } else {
            default = Err(default_string.to_string())
        }

        // Prepare a rule.
        let rules = Vec::new();
        let rule_result_map = HashMap::new();
        workflow_rule = WorkflowRule {
            name: name.clone(),
            rules,
            rule_result_map,
            default,
        };

        // Parse rules.
        let rule_raw_strings = &rest_raw_strings[0..(rest_raw_strings.len() - 1)];
        for raw_rule in rule_raw_strings.iter() {
            let mut raw_rule_split = raw_rule.split(":");

            let rule = Rule::from_str(raw_rule_split.next().unwrap()).unwrap();
            let raw_result = raw_rule_split.next().unwrap();

            // Try to find the referenced workflow.
            let result;
            if raw_result == "A" {
                result = Ok(true);
            } else if raw_result == "R" {
                result = Ok(false);
            } else {
                result = Err(raw_result.to_string())
            }

            let char_ = rule.property;
            workflow_rule.add_rule(char_, rule, result);
        }

        cache.insert(name.clone(), workflow_rule);
    }

    let parts = raw_parts
        .split("\n")
        .map(|s| MachinePart::from_str(s).unwrap())
        .collect::<Vec<MachinePart>>();

    (cache, parts)
}

pub struct Day19 {}

impl Day for Day19 {
    fn get_id(&self) -> String {
        "day_19".to_string()
    }

    fn part_a(&self, input: &String) -> String {
        let (cache, parts) = parse(input);

        // Parse the raw machine parts into `MachinePart` objects and compute result.
        let in_workflow: &WorkflowRule = cache.get("in").unwrap();
        parts
            .iter()
            .filter(|p| in_workflow.test_part(p, &cache))
            .map(|p| p.sum())
            .sum::<usize>()
            .to_string()
    }

    // We are not iterating over the possible parts, instead we take a space as input and return the
    // subspaces that are accepted. We then sum the sizes of the returned spaces.
    fn part_b(&self, input: &String) -> String {
        let (cache, _) = parse(input);

        let input_space = MachinePartSpace {
            x: 1..4001,
            m: 1..4001,
            a: 1..4001,
            s: 1..4001,
        };
        let in_workflow: &WorkflowRule = cache.get("in").unwrap();
        let (accepted, _) = in_workflow.test_space(input_space, &cache);
        accepted
            .iter()
            .map(|s| s.sum().clone())
            .sum::<usize>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_a() -> Result<(), String> {
        Day19 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day19 {}.test_day_part(&'b')
    }
}
