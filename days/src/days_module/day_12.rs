use crate::days_module::day::Day;
use rayon::prelude::*;

fn calculate_checksum(configuration: &Vec<char>) -> Vec<usize> {
    let mut streak = 0;
    let mut result = Vec::new();

    for char_ in configuration {
        match *char_ {
            '#' => streak += 1,
            '.' => {
                if streak > 0 {
                    result.push(streak.clone());
                    streak = 0;
                }
            }
            // Consider `?` in the least restraining way.
            '?' => {
                if streak > 0 {
                    result.push(streak.clone());
                    streak = 0;
                }
            }
            _ => panic!("Unknown character"),
        }
    }
    if streak > 0 {
        result.push(streak.clone());
    }

    result
}

fn verify_checksum(checksum: &Vec<usize>, reference: &Vec<usize>) -> Result<(), bool> {
    if checksum.len() > reference.len() {
        return Err(true);
    } else if checksum.len() < reference.len() {
        return Err(true);
    }

    for (check, reference) in checksum.iter().zip(reference.iter()) {
        if check < reference {
            return Err(true);
        } else if check > reference {
            return Err(true);
        }
    }

    Ok(())
}

fn count_configuration(
    configuration: &Vec<char>,
    reference: &Vec<usize>,
    last_mutated_index: Option<usize>,
) -> usize {
    match verify_checksum(&calculate_checksum(configuration), reference) {
        Ok(()) => return 1,
        Err(false) => return 0,
        Err(true) => {}
    }

    let next_mutated_index_result = find_unknown(configuration, last_mutated_index);

    if next_mutated_index_result.is_none() {
        return 0;
    }

    let next_mutated_index = next_mutated_index_result.unwrap();
    let mut cloned_configuration = configuration.clone();
    cloned_configuration[next_mutated_index] = '.';
    let defective_count =
        count_configuration(&cloned_configuration, reference, Some(next_mutated_index));
    cloned_configuration[next_mutated_index] = '#';
    let effective_count =
        count_configuration(&cloned_configuration, reference, Some(next_mutated_index));

    return effective_count + defective_count;
}

fn find_unknown(configuration: &Vec<char>, previous_index: Option<usize>) -> Option<usize> {
    configuration
        .iter()
        .enumerate()
        .filter(|(i, c)| **c == '?' && (previous_index.is_none() || (i > &previous_index.unwrap())))
        .map(|(i, _)| i)
        .next()
}

fn parse_line(line: &str) -> (Vec<char>, Vec<usize>) {
    let mut split = line.split(" ");
    let configuration = split.next().unwrap().chars().collect::<Vec<char>>();
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
            .collect::<Vec<(Vec<char>, Vec<usize>)>>();
        inputs
            .par_iter()
            .map(|t| count_configuration(&t.0, &t.1, None))
            .sum::<usize>()
            .to_string()
    }

    fn part_b(&self, input: &String) -> String {
        let inputs = input
            .split("\n")
            .map(parse_line)
            .map(|(c, r)| (multiply_vector(c, 5), multiply_vector(r, 5)))
            .collect::<Vec<(Vec<char>, Vec<usize>)>>();
        inputs
            .par_iter()
            .map(|t| count_configuration(&t.0, &t.1, None))
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
