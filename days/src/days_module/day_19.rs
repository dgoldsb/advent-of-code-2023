use crate::days_module::day::Day;
use std::collections::HashMap;

struct MachinePart {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl MachinePart {
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

#[derive(Clone, Eq, Hash, PartialEq)]
struct Rule {
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

    fn test_part(&self, part: MachinePart) -> bool {
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
        "".to_string()
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
