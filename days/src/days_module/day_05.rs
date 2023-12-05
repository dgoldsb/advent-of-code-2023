use crate::days_module::day::Day;
use helpers::ints_from_string;
use std::collections::HashMap;
use std::ops::Range;
use std::str::FromStr;

struct AlmanacRule {
    input_range: Range<isize>,
    delta: isize,
}

impl AlmanacRule {
    fn transform(&self, input: &isize) -> Result<isize, ()> {
        if self.input_range.contains(input) {
            return Ok(input - self.input_range.start + self.delta);
        }
        Err(())
    }
}

impl FromStr for AlmanacRule {
    type Err = ();

    fn from_str(input: &str) -> Result<AlmanacRule, Self::Err> {
        let mut integers = ints_from_string(input);
        let end = integers.pop().unwrap();
        let start = integers.pop().unwrap();
        let delta = integers.pop().unwrap();
        return Ok(AlmanacRule {
            input_range: start..(start + end),
            delta,
        });
    }
}

struct AlmanacTable {
    from: String,
    to: String,
    rules: Vec<AlmanacRule>,
}

impl AlmanacTable {
    fn transform(&self, i: &isize) -> Result<isize, ()> {
        for rule in &self.rules {
            let result = rule.transform(i);
            if result.is_ok() {
                return result;
            }
        }
        Err(())
    }
}

impl FromStr for AlmanacTable {
    type Err = ();

    fn from_str(input: &str) -> Result<AlmanacTable, Self::Err> {
        let mut input_iterator = input.split("\n");

        // First grab the from and to.
        let header = input_iterator.next().unwrap();
        let binding = header.replace("-", " ");
        let mut header_iterator = binding.split(" ");
        let from = header_iterator.next().unwrap().to_string();
        _ = header_iterator.next();
        let to = header_iterator.next().unwrap().to_string();

        // Now extract the rules!
        let rules = input_iterator
            .map(|r| AlmanacRule::from_str(r).unwrap())
            .collect::<Vec<AlmanacRule>>();

        return Ok(AlmanacTable { from, to, rules });
    }
}

fn exhaust_maps(seed: &isize, almanac_registry: &HashMap<String, AlmanacTable>) -> isize {
    let mut result = seed.clone();
    let mut state = "seed".to_string();
    while state != "location" {
        let current_almanac = &almanac_registry[&state];
        state = current_almanac.to.clone();
        result = current_almanac.transform(&result).unwrap_or(result);
    }
    result
}

pub struct Day05 {}

impl Day for Day05 {
    fn get_id(&self) -> String {
        "day_05".to_string()
    }

    fn part_a(&self, input: &String) -> String {
        let mut input_iterator = input.split("\n\n");

        let seeds = ints_from_string(input_iterator.next().unwrap());

        let almanac_registry = input_iterator
            .map(|s| AlmanacTable::from_str(s))
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap())
            .map(|a| (a.from.clone(), a))
            .collect::<HashMap<String, AlmanacTable>>();

        // Find the lowest location map. location
        seeds
            .iter()
            .map(|s| exhaust_maps(s, &almanac_registry))
            .min()
            .unwrap()
            .to_string()
    }

    fn part_b(&self, input: &String) -> String {
        "Not implemented".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_a() -> Result<(), String> {
        Day05 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day05 {}.test_day_part(&'b')
    }
}
