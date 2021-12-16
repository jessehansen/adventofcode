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
    let input = fs::read_to_string("./input.txt").expect("Something went wrong reading input.txt");

    let start = Instant::now();
    let input = parse(&input);
    let parse_time = start.elapsed();

    let part1_time = print_and_time("Part 1", || part1(&input));
    let part2_time = print_and_time("Part 2", || part2(&input));

    println!();

    println!("Stats:");
    println!(
        "Parse: {}ms ({}µs)",
        parse_time.as_millis(),
        parse_time.as_micros()
    );
    println!(
        "Part 1: {}ms ({}µs)",
        part1_time.as_millis(),
        part1_time.as_micros()
    );
    println!(
        "Part 2: {}ms ({}µs)",
        part2_time.as_millis(),
        part2_time.as_micros()
    );
}

fn print_and_time<F, T>(description: &str, runner: F) -> Duration
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
