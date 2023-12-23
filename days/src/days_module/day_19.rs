use crate::days_module::day::Day;
use std::collections::HashMap;
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
            .replace("x", "")
            .replace("m", "")
            .replace("s", "")
            .replace("s", "");
        let mut xmas_split = cleaned_raw_part.split(",");
        return Ok(MachinePart {
            x: xmas_split.next().unwrap().parse().unwrap(),
            m: xmas_split.next().unwrap().parse().unwrap(),
            a: xmas_split.next().unwrap().parse().unwrap(),
            s: xmas_split.next().unwrap().parse().unwrap(),
        });
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct Rule {
    property: char,
    sign: char,
    value: usize,
}

impl Rule {
    fn test(&self, value: &usize) -> bool {
        match self.sign {
            '<' => value < &self.value,
            '>' => value < &self.value,
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

struct WorkflowRule<'a> {
    rules: Vec<(char, Rule)>,
    rule_result_map: HashMap<Rule, Result<bool, &'a WorkflowRule<'a>>>,
    default: Result<bool, &'a WorkflowRule<'a>>,
}

impl<'a> WorkflowRule<'a> {
    fn add_rule(&mut self, char_: char, rule: Rule, result: Result<bool, &'a WorkflowRule>) {
        self.rules.push((char_, rule.clone()));
        self.rule_result_map.insert(rule, result);
    }

    fn test_part(&self, part: &MachinePart) -> bool {
        for (property, rule) in &self.rules {
            if rule.test(&part.access(&property)) {
                return match self.rule_result_map.get(&rule).unwrap() {
                    Ok(accepted) => *accepted,
                    Err(other_rule) => other_rule.test_part(part),
                };
            }
        }
        return match self.default {
            Ok(accepted) => accepted,
            Err(other_rule) => other_rule.test_part(part),
        };
    }
}

pub struct Day19 {}

impl Day for Day19 {
    fn get_id(&self) -> String {
        "day_19".to_string()
    }

    fn part_a(&self, input: &String) -> String {
        let mut input_split = input.split("\n\n");
        let raw_rule = input_split.next().unwrap();
        let raw_parts = input_split.next().unwrap();

        // Parse the raw workflow rules into `WorkflowRule` objects.
        let mut cache: HashMap<String, WorkflowRule> = HashMap::new();
        let mut raw_workflow_rule_queue = raw_rule
            .split("\n")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        while !raw_workflow_rule_queue.is_empty() {
            let raw_workflow_rule = raw_workflow_rule_queue.pop().unwrap();

            // Parse name out.
            let mut name_split = raw_workflow_rule.split("{");
            let name = name_split.next().unwrap().to_string();
            let rest = name_split.next().unwrap();

            // Parse rules.

            // Parse default.

            println!("{}", raw_workflow_rule);
            let foo = "";
        }
        let in_workflow: &WorkflowRule = cache.get("in").unwrap();

        // Parse the raw machine parts into `MachinePart` objects and compute result.
        raw_parts
            .split("\n")
            .map(|s| MachinePart::from_str(s).unwrap())
            .filter(|p| in_workflow.test_part(p))
            .map(|p| p.sum())
            .sum::<usize>()
            .to_string()
    }

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
