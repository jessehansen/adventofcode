use console::{style, Term};
use std::fmt;
use std::fmt::Display;
use std::fs;
use std::time::{Duration, Instant};

mod grid;
pub use grid::*;

mod tree;
pub use tree::*;

pub fn run<T, U, V, FParse, F1, F2>(parse: FParse, part1: F1, part2: F2)
where
    U: Display,
    V: Display,
    FParse: Fn(&str) -> T,
    F1: Fn(&T) -> U,
    F2: Fn(&T) -> V,
{
    let (input, parse_time) = read_and_parse(parse);

    let part1_time = print_and_time("Part 1", || part1(&input));
    let part2_time = print_and_time("Part 2", || part2(&input));

    print_stats(parse_time, part1_time, part2_time);
}

pub fn run_raw<U, V, F1, F2>(part1: F1, part2: F2)
where
    U: Display,
    V: Display,
    F1: Fn(&str) -> U,
    F2: Fn(&str) -> V,
{
    let (input, parse_time) = read_and_parse(trim);

    let part1_time = print_and_time("Part 1", || part1(&input));
    let part2_time = print_and_time("Part 2", || part2(&input));

    print_stats(parse_time, part1_time, part2_time);
}

// because I'm tired of clippy warnings
pub fn run_vec<T, U, V, FParse, F1, F2>(parse: FParse, part1: F1, part2: F2)
where
    U: Display,
    V: Display,
    FParse: Fn(&str) -> Vec<T>,
    F1: Fn(&[T]) -> U,
    F2: Fn(&[T]) -> V,
{
    let (input, parse_time) = read_and_parse(parse);

    let part1_time = print_and_time("Part 1", || part1(&input));
    let part2_time = print_and_time("Part 2", || part2(&input));

    print_stats(parse_time, part1_time, part2_time);
}

pub fn run_progressive_vec<T, T2, U, V, FParse, F1, F2>(parse: FParse, part1: F1, part2: F2)
where
    U: Display,
    V: Display,
    FParse: Fn(&str) -> Vec<T>,
    F1: Fn(&[T]) -> (U, T2),
    F2: Fn(&T2) -> V,
{
    let (input, parse_time) = read_and_parse(parse);

    let start = Instant::now();
    let (result, data_for_next) = part1(&input);
    let part1_time = start.elapsed();

    print!("Part 1 - ");
    let result = format!("{}", result);
    if result.len() > 20 || result.contains('\n') {
        println!();
    }
    println!("{}", style(result).bold());

    let part2_time = print_and_time("Part 2", || part2(&data_for_next));

    print_stats(parse_time, part1_time, part2_time);
}

fn read_and_parse<T, F>(parse: F) -> (T, Duration)
where
    F: Fn(&str) -> T,
{
    let input = fs::read_to_string("./input.txt").expect("Something went wrong reading input.txt");

    let start = Instant::now();
    let input = parse(&input);
    let parse_time = start.elapsed();

    (input, parse_time)
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
    println!("{}", style(result).bold());

    elapsed
}

fn print_stats(parse_time: Duration, part1_time: Duration, part2_time: Duration) {
    let term = &Term::stderr();
    term.write_line("").unwrap();
    term.write_line("Stats:").unwrap();
    print_time(term, "Parse", parse_time);
    print_time(term, "Part 1", part1_time);
    print_time(term, "Part 2", part2_time);
}

fn print_time(term: &Term, description: &str, time: Duration) {
    term.write_line(&format!("{}: {}", description, HumanDuration(time)))
        .unwrap();
}

pub fn hex_to_binary_string(hex: &str) -> String {
    hex.trim()
        .chars()
        .map(|x| {
            match x {
                '0' => "0000",
                '1' => "0001",
                '2' => "0010",
                '3' => "0011",
                '4' => "0100",
                '5' => "0101",
                '6' => "0110",
                '7' => "0111",
                '8' => "1000",
                '9' => "1001",
                'A' => "1010",
                'B' => "1011",
                'C' => "1100",
                'D' => "1101",
                'E' => "1110",
                'F' => "1111",
                c => panic!("unexpected char '{}' in hex string", c),
            }
            .to_string()
        })
        .collect()
}

pub fn binary_string_to_hex(binary: &str) -> String {
    if binary.len() % 4 != 0 {
        panic!("unpadded binary string");
    }
    (0..(binary.len() / 4))
        .map(|x| &binary[x * 4..x * 4 + 4])
        .map(|x| {
            match x {
                "0000" => "0",
                "0001" => "1",
                "0010" => "2",
                "0011" => "3",
                "0100" => "4",
                "0101" => "5",
                "0110" => "6",
                "0111" => "7",
                "1000" => "8",
                "1001" => "9",
                "1010" => "A",
                "1011" => "B",
                "1100" => "C",
                "1101" => "D",
                "1110" => "E",
                "1111" => "F",
                nib => panic!("unexpected nibble sequence \"{}\" in binary string", nib),
            }
            .to_string()
        })
        .collect()
}

pub fn trim(contents: &str) -> String {
    contents.trim().to_string()
}

pub fn pad_left_for_multiple(some_str: &mut String, padding: char, multiple: usize) {
    let pad_len = some_str.len() % multiple;
    if pad_len != 0 {
        some_str.insert_str(
            0,
            &(0..pad_len)
                .map(|_| padding.to_string())
                .collect::<String>(),
        );
    }
}

pub fn pad_right_for_multiple(some_str: &mut String, padding: char, multiple: usize) {
    let pad_len = some_str.len() % multiple;
    if pad_len != 0 {
        some_str.push_str(
            &(0..pad_len)
                .map(|_| padding.to_string())
                .collect::<String>(),
        );
    }
}

struct HumanDuration(Duration);
impl fmt::Display for HumanDuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let secs = self.0.as_secs();

        if secs > 3600 {
            write!(
                f,
                "{}:{:02}:{:02}",
                secs / 3600,
                (secs % 3600) / 60,
                secs % 60
            )
        } else if secs > 60 {
            write!(f, "{}:{:02}", secs / 60, secs % 60)
        } else if secs >= 1 {
            write!(f, "{:.3}s ({}ms)", self.0.as_secs_f32(), self.0.as_millis())
        } else if self.0.as_millis() > 1 {
            write!(f, "{}ms ({}µs)", self.0.as_millis(), self.0.as_micros())
        } else {
            write!(f, "{}µs ({}ns)", self.0.as_micros(), self.0.as_nanos())
        }
    }
}
