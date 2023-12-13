use crate::days_module::day::Day;
use rayon::prelude::*;

fn can_put_down_group(template: &str, group_size: usize) -> bool {
    if template.len() < group_size {
        return false;
    }
    let group_fits = template[0..group_size].chars().all(|c| c != '.');
    let terminator_fits = template.chars().nth(group_size).unwrap_or('.') != '#';
    group_fits && terminator_fits
}

// Recursive function.
// TODO: Cache.
// TODO: I could deduplicate code here, and make it more elegant.
fn count_configurations(template: &str, remaining_groups: &[usize]) -> usize {
    let next_group_size = remaining_groups.iter().next();
    match template.chars().nth(0) {
        Some('#') => {
            if next_group_size.is_none() {
                0
            } else {
                if can_put_down_group(template, *next_group_size.unwrap()) {
                    if template.len() > *next_group_size.unwrap() {
                        count_configurations(
                            &template[(next_group_size.unwrap() + 1)..],
                            &remaining_groups[1..],
                        )
                    } else {
                        if remaining_groups.len() == 1 {
                            1
                        } else {
                            0
                        }
                    }
                } else {
                    0
                }
            }
        }
        Some('.') => count_configurations(&template[1..], remaining_groups),
        Some('?') => {
            let fill_sum = if next_group_size.is_none() {
                0
            } else {
                if can_put_down_group(template, *next_group_size.unwrap()) {
                    if template.len() > *next_group_size.unwrap() {
                        count_configurations(
                            &template[(next_group_size.unwrap() + 1)..],
                            &remaining_groups[1..],
                        )
                    } else {
                        if remaining_groups.len() == 1 {
                            1
                        } else {
                            0
                        }
                    }
                } else {
                    0
                }
            };
            let empty_sum = count_configurations(&template[1..], remaining_groups);

            fill_sum + empty_sum
        }
        None => {
            // Terminal condition.
            if remaining_groups.is_empty() {
                1
            } else {
                0
            }
        }
        _ => panic!("Unknown character"),
    }
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

fn multiply_string(input: &str, multiplier: usize) -> String {
    let mut output = input.to_string();
    for _ in 1..multiplier {
        output += "?";
        output += input;
    }
    output
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
            .map(|(c, r)| (multiply_string(c, 5), multiply_vector(r, 5)))
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
