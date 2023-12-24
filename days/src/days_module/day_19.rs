use crate::days_module::day::Day;
use std::collections::{HashMap, VecDeque};
use std::str::FromStr;

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
        "".to_string()
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
