pub trait Day {
    fn get_id(&self) -> String;

    // An Advent of Code day consists of two parts: A and B.
    fn part_a(&self, input: &String) -> String;
    fn part_b(&self, input: &String) -> String;

    fn test_a(&self, input: &String, output_a: &String) {
        let solution = &Self::part_a(self, input);
        assert!(
            solution == output_a,
            "expected {} but found {}",
            output_a,
            solution
        );
    }

    fn test_b(&self, input: &String, output_b: &String) {
        let solution = &Self::part_b(self, input);
        assert!(
            solution == output_b,
            "expected {} but found {}",
            output_b,
            solution
        );
    }
}
