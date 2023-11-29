use days::days_module::day::Day;
use days::days_module::day_01::Day01;
use helpers::read_file;
use std::time::Instant;

fn test_day(day: &Box<dyn Day>) {
    // Get the test input for this day.
    let input_file = format!("example/input/{}", day.get_id());
    let input;
    match read_file(input_file) {
        Ok(contents) => input = contents,
        Err(_) => return,
    }

    // Get the first output.
    let output_a_file = format!("example/output/{}a", day.get_id());
    match read_file(output_a_file) {
        Ok(output_a) => day.test_a(&input, &output_a),
        Err(_) => {}
    }

    // Get the second output
    let output_b_file = format!("example/output/{}b", day.get_id());
    match read_file(output_b_file) {
        Ok(output_b) => day.test_b(&input, &output_b),
        Err(_) => {}
    }
}

fn execute_day(day: &Box<dyn Day>) -> (String, String) {
    // Get the input for this day.
    let input = read_file(day.get_id()).expect("Expected input to exist");

    let solution_a = day.part_a(&input);
    let solution_b = day.part_b(&input);
    return (solution_a, solution_b);
}

fn main() {
    let mut days: Vec<Box<dyn Day>> = Vec::new();
    days.push(Box::new(Day01 {}));

    let start = Instant::now();
    println!(
        "{0: <4}   | {1: <20} | {2: <20} | {3: <20}",
        "Day", "Part A", "Part B", "Runtime"
    );
    for day in days {
        // First run tests, if any.
        test_day(&day);

        // Start the timer!
        let now = Instant::now();

        // Run the parts.
        let solutions = execute_day(&day);

        let runtime = format!(
            "{}.{} ms",
            now.elapsed().as_millis(),
            now.elapsed().as_nanos() % 1000000
        );
        println!(
            "{0: <4} | {1: <20} | {2: <20} | {3: <20}",
            day.get_id(),
            solutions.0,
            solutions.1,
            runtime,
        );
    }
    println!("\nTotal {} ms", start.elapsed().as_millis());
}
