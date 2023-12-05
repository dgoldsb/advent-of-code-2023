use crate::days_module::day::Day;
use helpers::ints_from_string;
use rayon::prelude::*;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::ops::Range;
use std::str::FromStr;

struct AlmanacRule {
    input_range: Range<isize>,
    delta: isize,
}

impl AlmanacRule {
    // Returns a series of original ranges, and the new range with the delta applied.
    fn transform_range(&self, range: &Range<isize>) -> Result<(Range<isize>, Range<isize>), ()> {
        let range_start = max(range.start, self.input_range.start);
        let range_end = min(range.end, self.input_range.end);
        if range_start < range_end {
            Ok((
                range_start..range_end,
                (range_start - self.input_range.start + self.delta)
                    ..(range_end - self.input_range.start + self.delta),
            ))
        } else {
            Err(())
        }
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
    fn transform_range(&self, range: Range<isize>) -> Vec<Range<isize>> {
        let mut result = Vec::new();

        // Check which sub-ranges are mappable.
        let mut mapped_ranges = self
            .rules
            .iter()
            .map(|r| r.transform_range(&range))
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap())
            .collect::<Vec<(Range<isize>, Range<isize>)>>();

        // Create identity ranges for the non-mappable pieces.
        mapped_ranges.sort_by(|a, b| a.0.start.partial_cmp(&b.0.start).unwrap());

        // Initial value chosen to check for an open head range.
        let mut previous_end = range.start;
        for (mapped_range, mapped_transformed_range) in mapped_ranges {
            if mapped_range.start > previous_end {
                result.push(previous_end..mapped_range.start);
            }

            previous_end = range.end;
            result.push(mapped_transformed_range);
        }

        // Check for an open tail range.
        if previous_end < range.end {
            result.push(previous_end..range.end);
        }

        // Return the new collection of ranges.
        result
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

fn find_lowest_location(
    range: Range<isize>,
    key: String,
    almanac_registry: &HashMap<String, AlmanacTable>,
) -> isize {
    if key == "location" {
        // The end of recursion!
        return range.start;
    }

    // Get 1 or more ranges from our almanacs.
    let current_almanac = &almanac_registry[&key];
    let ranges: Vec<Range<isize>> = current_almanac.transform_range(range);

    // Find the lowest of all the range starts.
    let next_location = current_almanac.to.clone();
    ranges
        .iter()
        .map(|r| find_lowest_location(r.clone(), next_location.clone(), almanac_registry))
        .min()
        .unwrap()
}

fn solve(input: &String, part_a: bool) -> String {
    let mut input_iterator = input.split("\n\n");

    // Find the seed ranges.
    let mut seeds = ints_from_string(input_iterator.next().unwrap());
    let mut seed_ranges = Vec::new();
    while !seeds.is_empty() {
        let length = seeds.pop().unwrap();
        if part_a {
            seed_ranges.push(length..(length + 1));
        } else {
            let start = seeds.pop().unwrap();
            seed_ranges.push(start..(start + length));
        }
    }

    // Parse the almanac.
    let almanac_registry = input_iterator
        .map(|s| AlmanacTable::from_str(s))
        .filter(|r| r.is_ok())
        .map(|r| r.unwrap())
        .map(|a| (a.from.clone(), a))
        .collect::<HashMap<String, AlmanacTable>>();

    // Solve.
    seed_ranges
        .par_iter()
        .map(|x| find_lowest_location(x.clone(), "seed".to_string(), &almanac_registry))
        .min()
        .unwrap()
        .to_string()
}

pub struct Day05 {}

impl Day for Day05 {
    fn get_id(&self) -> String {
        "day_05".to_string()
    }

    fn part_a(&self, input: &String) -> String {
        solve(input, true)
    }

    fn part_b(&self, input: &String) -> String {
        solve(input, false)
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
