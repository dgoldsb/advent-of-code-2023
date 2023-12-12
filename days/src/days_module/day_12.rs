use crate::days_module::day::Day;
use rayon::prelude::*;

// TODO: Caching pays for sure.
// TODO: Clean up the cloning, all slice based.
// Recursive function.
// TODO: Some of these clauses may be redundant.
fn count_configurations(template: &str, remaining_groups: &[usize]) -> usize {
    // If we run out of groups, we check if the remainder contains no springs.
    if remaining_groups.is_empty() {
        if template.chars().filter(|c| *c == '#').count() == 0 {
            return 1;
        } else {
            return 0;
        }
    }

    if template.chars().all(|c| c != '.')
        && template.chars().filter(|c| *c != '.').count() == remaining_groups.iter().sum()
    {
        // If we have one left, we win!
        // If we have multiple groups that fit exactly, return 0.
        if remaining_groups.len() == 1 {
            return 1;
        } else {
            return 0;
        }
    }

    // If there are less `#` + `?` left than group sizes, we fail.
    if template.chars().filter(|c| *c != '.').count() < remaining_groups.iter().sum() {
        return 0;
    }

    // We have two options: consume a group at the next `?`, or place a `.` spring.
    let mut remaining_groups_iter = remaining_groups.iter();
    let mut next_group_size = remaining_groups_iter.next().unwrap();
    let mut active_group = 0;

    // Fast forwarding we do after we place a group.
    let mut fast_forwarding = false;

    for (index, char) in template.chars().enumerate() {
        let placing_group = active_group > 0;

        // Leading empty spaces we might as well skip...
        if char == '.' {
            // ... unless we are grouping, then we fail!
            if placing_group {
                // ... unless our group is the right size, then we fast-forward!
                if *next_group_size == active_group {
                    active_group = 0;
                    fast_forwarding = true;
                } else {
                    return 0;
                }
            }
            continue;
        }
        // A `#` has us consume our next group.
        else if char == '#' || (placing_group && char == '?' && active_group < *next_group_size) {
            // If we are consuming, but our next group is too small, we messed up.
            if active_group == *next_group_size {
                return 0;
            }

            // We can go back from fastforwarding to consuming.
            if fast_forwarding {
                fast_forwarding = false;
                let next_group_size_option = remaining_groups_iter.next();
                // But if we have no groups left, then there is no options!
                if next_group_size_option.is_none() {
                    return 0;
                } else {
                    next_group_size = next_group_size_option.unwrap()
                }
            }

            // Always consume.
            active_group += 1;
        }
        // A `?` finally has us go into recursion, potentially.
        else if char == '?' {
            let mut sum = 0;

            // Collect back all remaining groups.
            let foo = remaining_groups_iter.map(|u| *u).collect::<Vec<usize>>();
            let recurse_remaining_groups = vec![next_group_size.clone()]
                .iter()
                .chain(foo.iter())
                .map(|u| *u)
                .collect::<Vec<usize>>();

            // Place a `.`.
            if active_group == 0 {
                let increase_a =
                    count_configurations(&template[(index + 1)..], &recurse_remaining_groups);
                sum += increase_a;
            }
            // Definitely place a `.` if we can finish the active group.
            else if active_group == *next_group_size {
                let increase_b =
                    count_configurations(&template[(index + 1)..], &recurse_remaining_groups[1..]);
                sum += increase_b;
            }

            // Try to place the next group, or extend one.
            let mut success = true;
            for i in 0..(*next_group_size - active_group) {
                if template.chars().nth(index + i).unwrap() == '.' {
                    success = false
                }
            }
            if template
                .chars()
                .nth(index + next_group_size - active_group)
                .is_some()
                && template
                    .chars()
                    .nth(index + next_group_size - active_group)
                    .unwrap()
                    == '#'
            {
                success = false
            }

            // Second clause to avoid duplicate counting.
            if success && !(active_group == *next_group_size) {
                let increase_c = count_configurations(
                    &template[(index + next_group_size + 1)..],
                    &recurse_remaining_groups[1..],
                );
                sum += increase_c;
            }

            return sum;
        }
    }
    1
}

fn parse_line<'a>(line: &'a str) -> (&'a str, Vec<usize>) {
    let mut split = line.split(" ");
    let configuration = split.next().unwrap();
    let reference = split
        .next()
        .unwrap()
        .split(",")
        .map(|i| i.parse().unwrap())
        .collect::<Vec<usize>>();
    (configuration, reference)
}

fn multiply_vector<T>(input: Vec<T>, multiplier: usize) -> Vec<T>
where
    T: Copy,
{
    let mut result: Vec<T> = Vec::new();
    for _ in 0..multiplier {
        result.extend(input.clone())
    }
    result
}

pub struct Day12 {}

impl Day for Day12 {
    fn get_id(&self) -> String {
        "day_12".to_string()
    }

    fn part_a(&self, input: &String) -> String {
        let inputs = input
            .split("\n")
            .map(parse_line)
            .collect::<Vec<(&str, Vec<usize>)>>();
        inputs
            .par_iter()
            .map(|t| count_configurations(&t.0, &t.1))
            .sum::<usize>()
            .to_string()
    }

    fn part_b(&self, input: &String) -> String {
        let inputs = input
            .split("\n")
            .map(parse_line)
            .map(|(c, r)| (c.repeat(5), multiply_vector(r, 5)))
            .collect::<Vec<(String, Vec<usize>)>>();
        inputs
            .par_iter()
            .map(|t| count_configurations(&t.0, &t.1))
            .sum::<usize>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_a() -> Result<(), String> {
        Day12 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day12 {}.test_day_part(&'b')
    }
}
