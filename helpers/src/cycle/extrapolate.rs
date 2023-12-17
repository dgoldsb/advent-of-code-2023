use std::collections::HashSet;

fn detect_cycle<T>(mut iter: impl Iterator<Item = T>) -> Result<(usize, usize, Vec<T>), Vec<T>>
where
    T: Eq + std::hash::Hash + Clone,
{
    let mut seen = HashSet::new();
    let mut values = Vec::new();

    loop {
        let value_option = iter.next();
        let value = match value_option {
            Some(v) => v,
            None => break,
        };

        if seen.contains(&value) {
            let cycle_start = values.iter().position(|c| *c == value).unwrap();
            return Ok((cycle_start, values.len() - cycle_start, values));
        }

        values.push(value.clone());
        seen.insert(value);
    }

    Err(values)
}

pub fn extrapolate_nth<T>(mut iter: impl Iterator<Item = T>, n: usize) -> T
where
    T: Eq + std::hash::Hash + Clone,
{
    match detect_cycle(iter) {
        Ok((cycle_start, cycle_length, values)) => {
            // Skip to the beginning of the cycle and adjust n.
            let steps_before_cycle = n - cycle_start - 1;
            let n_in_cycle = steps_before_cycle % cycle_length;
            let total_steps = cycle_start + n_in_cycle;

            let result = values.get(total_steps).unwrap().clone();
            return result;
        }
        Err(values) => {
            println!("{}", values.len());
            let result = values.get(n).unwrap().clone();
            return result;
        }
    }
}
