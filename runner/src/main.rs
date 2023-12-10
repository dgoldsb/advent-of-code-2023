use days::days_module::day::Day;
use days::days_module::day_01::Day01;
use days::days_module::day_02::Day02;
use days::days_module::day_03::Day03;
use days::days_module::day_04::Day04;
use days::days_module::day_05::Day05;
use days::days_module::day_06::Day06;
use days::days_module::day_07::Day07;
use days::days_module::day_08::Day08;
use days::days_module::day_09::Day09;
use days::days_module::day_10::Day10;
use helpers::read_file;
use std::time::Instant;

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
    days.push(Box::new(Day02 {}));
    days.push(Box::new(Day03 {}));
    days.push(Box::new(Day04 {}));
    days.push(Box::new(Day05 {}));
    days.push(Box::new(Day06 {}));
    days.push(Box::new(Day07 {}));
    days.push(Box::new(Day08 {}));
    days.push(Box::new(Day09 {}));
    days.push(Box::new(Day10 {}));

    let start = Instant::now();
    println!(
        "{0: <4}   | {1: <20} | {2: <20} | {3: <20}",
        "Day", "Part A", "Part B", "Runtime"
    );
    for day in days {
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
