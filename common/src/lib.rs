use std::fs;
use std::time::{Instant, Duration};

pub fn run<T>(
    parse: &dyn Fn(&str) -> T,
    part1: &dyn Fn(&T) -> String,
    part2: &dyn Fn(&T) -> String,
) {
    let sample = fs::read_to_string("./sample.txt")
        .expect("Something went wrong reading sample.txt");
    let input = fs::read_to_string("./input.txt")
        .expect("Something went wrong reading input.txt");

    let start = Instant::now();
    let parsed_sample = parse(&sample);
    let sample_parse_time = start.elapsed();

    let start = Instant::now();
    let parsed_input = parse(&input);
    let input_parse_time = start.elapsed();

    let part1_sample_time = print_result("Part 1 (sample)", &|| part1(&parsed_sample));
    let part2_sample_time = print_result("Part 2 (sample)", &|| part2(&parsed_sample));
    println!("");

    let part1_input_time = print_result("Part 1  (input)", &|| part1(&parsed_input));
    let part2_input_time = print_result("Part 2  (input)", &|| part2(&parsed_input));

    println!("");
    println!("Stats (sample):");
    println!("Parse: {}ms ({}µs)", sample_parse_time.as_millis(), sample_parse_time.as_micros());
    println!("Part 1: {}ms ({}µs)", part1_sample_time.as_millis(), part1_sample_time.as_micros());
    println!("Part 2: {}ms ({}µs)", part2_sample_time.as_millis(), part2_sample_time.as_micros());
    println!("");
    println!("Stats (input):");
    println!("Parse: {}ms ({}µs)", input_parse_time.as_millis(), input_parse_time.as_micros());
    println!("Part 1: {}ms ({}µs)", part1_input_time.as_millis(), part1_input_time.as_micros());
    println!("Part 2: {}ms ({}µs)", part2_input_time.as_millis(), part2_input_time.as_micros());
}

pub fn run_sample<T>(
    parse: &dyn Fn(&str) -> T,
    part1: &dyn Fn(&T) -> String,
    part2: &dyn Fn(&T) -> String,
) {
    let sample = fs::read_to_string("./sample.txt")
        .expect("Something went wrong reading sample.txt");

    let parsed_sample: T = parse(&sample);

    print_result("Part 1 (sample)", &|| part1(&parsed_sample));
    print_result("Part 2 (sample)", &|| part2(&parsed_sample));
}

fn print_result(description: &str, runner: &dyn Fn() -> String) -> Duration {
    let start = Instant::now();

    let result = runner();
    println!("{} - {}", description, result);
    start.elapsed()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
