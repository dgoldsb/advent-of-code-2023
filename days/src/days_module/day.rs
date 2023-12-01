use helpers::read_file;

pub trait Day {
    fn get_id(&self) -> String;

    // An Advent of Code day consists of two parts: A and B.
    fn part_a(&self, input: &String) -> String;
    fn part_b(&self, input: &String) -> String;

    fn test_a(&self, input: &String, output_a: &String) -> Result<(), String> {
        let solution = &Self::part_a(self, input);
        if solution != output_a {
            return Err(format!("Expected {} but found {}", output_a, solution));
        }
        Ok(())
    }

    fn test_b(&self, input: &String, output_b: &String) -> Result<(), String> {
        let solution = &Self::part_b(self, input);
        if solution != output_b {
            return Err(format!("Expected {} but found {}", output_b, solution));
        }
        Ok(())
    }

    fn test_day_part(&self, part: &char) -> Result<(), String> {
        // Get the input.
        let input_file = format!("example/input/{}{}", self.get_id(), part);
        let input;
        match read_file(input_file) {
            Ok(contents) => input = contents,
            Err(reason) => return Err(reason),
        }

        // Get the output.
        let output_file = format!("example/output/{}{}", self.get_id(), part);
        return match read_file(output_file) {
            Ok(output) => match part {
                'a' => self.test_a(&input, &output),
                'b' => self.test_b(&input, &output),
                _ => return Err("Unknown day".to_string()),
            },
            Err(reason) => return Err(reason),
        };
    }
}
