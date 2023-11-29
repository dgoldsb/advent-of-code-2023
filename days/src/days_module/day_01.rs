use crate::days_module::day::Day;
use helpers::ints_from_string;

pub struct Day01 {}

impl Day for Day01 {
    fn get_id(&self) -> String {
        "day_01".to_string()
    }

    fn part_a(&self, input: &String) -> String {
        let window = 1;
        let inputs = ints_from_string(input);

        let mut count: usize = 0;

        for i in 1..(inputs.len() + 1 - window) {
            let mut cum_a = 0;
            for j in (i - 1)..(i - 1 + window) {
                cum_a += inputs[j];
            }

            let mut cum_b = 0;
            for k in i..(i + window) {
                cum_b += inputs[k];
            }

            if cum_b > cum_a {
                count += 1;
            }
        }
        count.to_string()
    }

    fn part_b(&self, input: &String) -> String {
        let window = 3;
        let inputs = ints_from_string(input);

        let mut count: usize = 0;

        for i in 1..(inputs.len() + 1 - window) {
            let mut cum_a = 0;
            for j in (i - 1)..(i - 1 + window) {
                cum_a += inputs[j];
            }

            let mut cum_b = 0;
            for k in i..(i + window) {
                cum_b += inputs[k];
            }

            if cum_b > cum_a {
                count += 1;
            }
        }
        count.to_string()
    }
}
