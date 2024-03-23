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
use days::days_module::day_11::Day11;
use days::days_module::day_12::Day12;
use days::days_module::day_13::Day13;
use days::days_module::day_14::Day14;
use days::days_module::day_15::Day15;
use days::days_module::day_16::Day16;
use days::days_module::day_17::Day17;
use days::days_module::day_18::Day18;
use days::days_module::day_19::Day19;
use days::days_module::day_20::Day20;
use days::days_module::day_21::Day21;
use days::days_module::day_22::Day22;
use days::days_module::day_23::Day23;
use days::days_module::day_24::Day24;
use days::days_module::day_25::Day25;
use helpers::read_file;
use std::time::Instant;

fn execute_day(day: &Box<dyn Day>) -> Result<(String, String), String> {
    // Get the input for this day.
    let input_result = read_file(day.get_id());

    if input_result.is_err() {
        return Err("Expected input file to be present".to_string());
    }

    let input = input_result.unwrap();
    let solution_a = day.part_a(&input);
    let solution_b = day.part_b(&input);
    return Ok((solution_a, solution_b));
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
    days.push(Box::new(Day09 {}));
    days.push(Box::new(Day10 {}));
    days.push(Box::new(Day11 {}));
    days.push(Box::new(Day12 {}));
    days.push(Box::new(Day13 {}));
    days.push(Box::new(Day15 {}));
    days.push(Box::new(Day18 {}));
    days.push(Box::new(Day19 {}));
    days.push(Box::new(Day21 {}));
    days.push(Box::new(Day24 {}));
    days.push(Box::new(Day25 {}));
    // Slow days...
    days.push(Box::new(Day14 {}));
    days.push(Box::new(Day16 {}));
    days.push(Box::new(Day17 {}));
    days.push(Box::new(Day22 {}));
    // Unfinished days...
    days.push(Box::new(Day08 {}));
    days.push(Box::new(Day20 {}));
    days.push(Box::new(Day23 {}));

    let start = Instant::now();
    println!(
        "{0: <4}   | {1: <20} | {2: <20} | {3: <20}",
        "Day", "Part A", "Part B", "Runtime"
    );
    for day in days {
        // Start the timer!
        let now = Instant::now();

        // Run the parts.
        let solutions = execute_day(&day).unwrap_or_else(|_| ("".to_string(), "".to_string()));

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
