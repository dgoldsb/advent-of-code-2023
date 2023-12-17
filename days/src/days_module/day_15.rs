use crate::days_module::day::Day;
use std::collections::HashMap;

fn calculate_focusing_power(lens_array: HashMap<usize, Vec<(String, usize)>>) -> usize {
    let mut sum = 0;
    for (box_, lenses) in lens_array.iter() {
        for (slot, lens) in lenses.iter().enumerate() {
            let lens_power = (1 + box_) * lens.1 * (slot + 1);
            sum += lens_power;
        }
    }
    sum
}

fn hash(string: &str) -> usize {
    let mut current_value = 0;
    for c in string.chars() {
        let ascii_value = c as usize;
        current_value += ascii_value;
        current_value *= 17;
        current_value %= 256;
    }
    current_value
}

pub struct Day15 {}

impl Day for Day15 {
    fn get_id(&self) -> String {
        "day_15".to_string()
    }

    fn part_a(&self, input: &String) -> String {
        input.split(",").map(hash).sum::<usize>().to_string()
    }

    fn part_b(&self, input: &String) -> String {
        let empty = Vec::new();
        let mut map: HashMap<usize, Vec<(String, usize)>> = HashMap::new();

        for value in input.split(",") {
            if value.contains("-") {
                // Unpack.
                let mut split = value.split("-");
                let label = split.next().unwrap();
                let box_ = hash(label);

                // Remove the lens with the given label if it is present in the box.
                let vec = match map.get(&box_) {
                    Some(v) => v,
                    None => &empty,
                };
                let new_vec = vec
                    .iter()
                    .filter(|(l, _)| label != l)
                    .map(|t| t.clone())
                    .collect();

                // Collect.
                map.insert(box_, new_vec);
            }

            if value.contains("=") {
                // Unpack.
                let mut split = value.split("=");
                let label = split.next().unwrap();
                let box_ = hash(label);
                let lens_strength: usize = (split.next().unwrap()).parse().unwrap();

                let vec = match map.get(&box_) {
                    Some(v) => v,
                    None => &empty,
                };

                let mut replaced = false;
                // If there is already a lens in the box with the same label, replace the old lens
                // with the new lens: remove the old lens and put the new lens in its place,
                // not moving any other lenses in the box.
                let mut new_vec: Vec<(String, usize)> = vec
                    .iter()
                    .map(|(l, v)| {
                        if l == label {
                            replaced = true;
                            return (l.clone(), lens_strength);
                        } else {
                            return (l.clone(), v.clone());
                        }
                    })
                    .collect();

                // If there is not already a lens in the box with the same label, add the lens to
                // the box immediately behind any lenses already in the box. Don't move any of the
                // other lenses when you do this. If there aren't any lenses in the box, the new
                // lens goes all the way to the front of the box.
                if !replaced {
                    new_vec.push((label.to_string(), lens_strength))
                }

                // Collect.
                map.insert(box_, new_vec);
            }
        }

        return calculate_focusing_power(map).to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_a() -> Result<(), String> {
        Day15 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day15 {}.test_day_part(&'b')
    }
}
