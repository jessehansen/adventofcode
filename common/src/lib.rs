use std::fmt::Display;
use std::fs;
use std::time::{Duration, Instant};

mod grid;

pub use grid::*;

pub fn run<T, U, V, FParse, F1, F2>(parse: FParse, part1: F1, part2: F2)
where
    U: Display,
    V: Display,
    FParse: Fn(&str) -> T,
    F1: Fn(&T) -> U,
    F2: Fn(&T) -> V,
{
    let sample =
        fs::read_to_string("./sample.txt").expect("Something went wrong reading sample.txt");
    let input = fs::read_to_string("./input.txt").expect("Something went wrong reading input.txt");

    let start = Instant::now();
    let sample = parse(&sample);
    let sample_parse_time = start.elapsed();

    let start = Instant::now();
    let input = parse(&input);
    let input_parse_time = start.elapsed();

    let part1_sample_time = print_result("Part 1 (sample)", || part1(&sample));
    let part2_sample_time = print_result("Part 2 (sample)", || part2(&sample));
    println!();

    let part1_input_time = print_result("Part 1  (input)", || part1(&input));
    let part2_input_time = print_result("Part 2  (input)", || part2(&input));

    println!();

    if std::env::var("PRINT_SAMPLE_STATS").is_ok() {
        println!("Stats (sample):");
        println!(
            "Parse: {}ms ({}µs)",
            sample_parse_time.as_millis(),
            sample_parse_time.as_micros()
        );
        println!(
            "Part 1: {}ms ({}µs)",
            part1_sample_time.as_millis(),
            part1_sample_time.as_micros()
        );
        println!(
            "Part 2: {}ms ({}µs)",
            part2_sample_time.as_millis(),
            part2_sample_time.as_micros()
        );
        println!();
    }
    println!("Stats (input):");
    println!(
        "Parse: {}ms ({}µs)",
        input_parse_time.as_millis(),
        input_parse_time.as_micros()
    );
    println!(
        "Part 1: {}ms ({}µs)",
        part1_input_time.as_millis(),
        part1_input_time.as_micros()
    );
    println!(
        "Part 2: {}ms ({}µs)",
        part2_input_time.as_millis(),
        part2_input_time.as_micros()
    );
}

pub fn run_sample<T, U, V, FParse, F1, F2>(parse: FParse, part1: F1, part2: F2)
where
    U: Display,
    V: Display,
    FParse: Fn(&str) -> T,
    F1: Fn(&T) -> U,
    F2: Fn(&T) -> V,
{
    let sample =
        fs::read_to_string("./sample.txt").expect("Something went wrong reading sample.txt");

    let sample = parse(&sample);

    print_result("Part 1 (sample)", || part1(&sample));
    print_result("Part 2 (sample)", || part2(&sample));
}

fn print_result<F, T>(description: &str, runner: F) -> Duration
where
    T: Display,
    F: Fn() -> T,
{
    let start = Instant::now();
    let result = runner();
    let elapsed = start.elapsed();

    print!("{} - ", description);
    let result = format!("{}", result);
    if result.len() > 20 || result.contains('\n') {
        println!();
    }
    println!("{}", result);

    elapsed
}
