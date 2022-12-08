use std::fmt::{self, Display};
use std::fs;
use std::time::{Duration, Instant};

use anyhow::*;
use console::{style, Term};

mod grid;
pub use grid::*;

mod graph;
pub use graph::*;

mod tree;
pub use tree::*;

pub fn run<T, U, V, FParse, F1, F2>(parse: FParse, part1: F1, part2: F2) -> Result<()>
where
    U: Display,
    V: Display,
    FParse: Fn(&str) -> Result<T>,
    F1: Fn(&T) -> Result<U>,
    F2: Fn(&T) -> Result<V>,
{
    let (input, parse_time) = read_and_parse(parse)?;

    let part1_time = print_and_time("Part 1", || part1(&input)).context("failure in part 1")?;
    let part2_time = print_and_time("Part 2", || part2(&input)).context("failure in part 2")?;

    print_stats(parse_time, part1_time, part2_time);
    Ok(())
}

pub fn run_raw<U, V, F1, F2>(part1: F1, part2: F2) -> Result<()>
where
    U: Display,
    V: Display,
    F1: Fn(&str) -> Result<U>,
    F2: Fn(&str) -> Result<V>,
{
    let (input, parse_time) = read_and_parse(|x| Ok(trim(x)))?;

    let part1_time = print_and_time("Part 1", || part1(&input))?;
    let part2_time = print_and_time("Part 2", || part2(&input))?;

    print_stats(parse_time, part1_time, part2_time);

    Ok(())
}

// because I'm tired of clippy warnings
pub fn run_vec<T, U, V, FParse, F1, F2>(parse: FParse, part1: F1, part2: F2) -> Result<()>
where
    U: Display,
    V: Display,
    FParse: Fn(&str) -> Result<Vec<T>>,
    F1: Fn(&[T]) -> Result<U>,
    F2: Fn(&[T]) -> Result<V>,
{
    let (input, parse_time) = read_and_parse(parse)?;

    let part1_time = print_and_time("Part 1", || part1(&input)).context("failure in part 1")?;
    let part2_time = print_and_time("Part 2", || part2(&input)).context("failure in part 2")?;

    print_stats(parse_time, part1_time, part2_time);
    Ok(())
}

pub fn run_progressive<T, T2, U, V, FParse, F1, F2>(
    parse: FParse,
    part1: F1,
    part2: F2,
) -> Result<()>
where
    U: Display,
    V: Display,
    FParse: Fn(&str) -> Result<T>,
    F1: Fn(&T) -> Result<(U, T2)>,
    F2: Fn(&T, &T2) -> Result<V>,
{
    let (input, parse_time) = read_and_parse(parse)?;

    let (part1_time, data_for_next) = print_and_time_and_return("Part 1", || part1(&input))?;
    let part2_time = print_and_time("Part 2", || part2(&input, &data_for_next))?;

    print_stats(parse_time, part1_time, part2_time);

    Ok(())
}

pub fn run_progressive_vec<T, T2, U, V, FParse, F1, F2>(
    parse: FParse,
    part1: F1,
    part2: F2,
) -> Result<()>
where
    U: Display,
    V: Display,
    FParse: Fn(&str) -> Result<Vec<T>>,
    F1: Fn(&[T]) -> Result<(U, T2)>,
    F2: Fn(&[T], &T2) -> Result<V>,
{
    let (input, parse_time) = read_and_parse(parse)?;

    let start = Instant::now();
    let (result, data_for_next) = part1(&input).context("failure in part 1")?;
    let part1_time = start.elapsed();

    print!("Part 1 - ");
    let result = format!("{result}");
    if result.len() > 20 || result.contains('\n') {
        println!();
    }
    println!("{}", style(result).bold());

    let part2_time =
        print_and_time("Part 2", || part2(&input, &data_for_next)).context("failure in part 2")?;

    print_stats(parse_time, part1_time, part2_time);

    Ok(())
}

fn read_and_parse<T, F>(parse: F) -> Result<(T, Duration)>
where
    F: Fn(&str) -> Result<T>,
{
    let input = fs::read_to_string("./input.txt").context("could not read input.txt")?;

    let start = Instant::now();
    let input = parse(&input)?;
    let parse_time = start.elapsed();

    Ok((input, parse_time))
}

fn print_and_time<F, T>(description: &str, mut runner: F) -> Result<Duration>
where
    T: Display,
    F: FnMut() -> Result<T>,
{
    let start = Instant::now();
    let result = runner()?;
    let elapsed = start.elapsed();

    print!("{description} - ");
    let result = format!("{result}");
    if result.len() > 20 || result.contains('\n') {
        println!();
    }
    println!("{}", style(result).bold());

    Ok(elapsed)
}

fn print_and_time_and_return<F, T, T2>(description: &str, runner: F) -> Result<(Duration, T2)>
where
    T: Display,
    F: Fn() -> Result<(T, T2)>,
{
    let start = Instant::now();
    let (result, more_data) = runner()?;
    let elapsed = start.elapsed();

    print!("{description} - ");
    let result = format!("{result}");
    if result.len() > 20 || result.contains('\n') {
        println!();
    }
    println!("{}", style(result).bold());

    Ok((elapsed, more_data))
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

// common parsing helpers

pub fn trim(contents: &str) -> String {
    contents.trim().to_string()
}

pub fn wrap_parse_error<T, TErr>(result: std::result::Result<T, TErr>) -> Result<T>
where
    TErr: std::fmt::Display,
{
    match result {
        std::result::Result::Ok(value) => Ok(value),
        std::result::Result::Err(err) => Err(anyhow!("parse error {}", err)),
    }
}

pub fn parse_all<T>(contents: &str) -> Result<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    wrap_parse_error(contents.trim().parse())
}

pub fn parse_untrimmed<T>(contents: &str) -> Result<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    wrap_parse_error(contents.parse())
}

pub fn parse_lines<T>(contents: &str) -> Result<Vec<T>>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    contents
        .lines()
        .map(|x| wrap_parse_error(x.parse()))
        .collect()
}

pub fn parse_chars<T>(contents: &str) -> Result<Vec<T>>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    contents
        .trim()
        .chars()
        .map(|x| wrap_parse_error(x.to_string().parse()))
        .collect()
}

pub fn parse_line_groups<T, FParse>(contents: &str, parse_group: FParse) -> Result<Vec<T>>
where
    FParse: Fn(&str) -> Result<T>,
{
    contents.split("\n\n").map(parse_group).collect()
}

pub fn parse_line_pairs<T>(contents: &str, separator: &str) -> Result<Vec<(T, T)>>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    contents
        .lines()
        .map(|x| wrap_parse_error(parse_pair(x, separator)))
        .collect()
}

pub fn parse_pair<T0, T1>(contents: &str, separator: &str) -> Result<(T0, T1)>
where
    T0: std::str::FromStr,
    <T0 as std::str::FromStr>::Err: std::fmt::Display,
    T1: std::str::FromStr,
    <T1 as std::str::FromStr>::Err: std::fmt::Display,
{
    let mut parts = contents.split(separator);
    Ok((
        wrap_parse_error(
            parts
                .next()
                .ok_or_else(|| anyhow!("malformed pair"))?
                .parse(),
        )?,
        wrap_parse_error(
            parts
                .next()
                .ok_or_else(|| anyhow!("malformed pair"))?
                .parse(),
        )?,
    ))
}

pub fn parse_pair_by<T0, FParse0, T1, FParse1>(
    contents: &str,
    separator: &str,
    parse0: FParse0,
    parse1: FParse1,
) -> Result<(T0, T1)>
where
    FParse0: Fn(&str) -> Result<T0>,
    FParse1: Fn(&str) -> Result<T1>,
{
    let mut parts = contents.split(separator);
    Ok((
        parse0(parts.next().ok_or_else(|| anyhow!("malformed pair"))?)?,
        parse1(parts.next().ok_or_else(|| anyhow!("malformed pair"))?)?,
    ))
}

pub fn parse_triple<T0, T1, T2>(contents: &str, separator: &str) -> Result<(T0, T1, T2)>
where
    T0: std::str::FromStr,
    <T0 as std::str::FromStr>::Err: std::fmt::Display,
    T1: std::str::FromStr,
    <T1 as std::str::FromStr>::Err: std::fmt::Display,
    T2: std::str::FromStr,
    <T2 as std::str::FromStr>::Err: std::fmt::Display,
{
    let mut parts = contents.split(separator);
    Ok((
        wrap_parse_error(
            parts
                .next()
                .ok_or_else(|| anyhow!("malformed triple"))?
                .parse(),
        )?,
        wrap_parse_error(
            parts
                .next()
                .ok_or_else(|| anyhow!("malformed triple"))?
                .parse(),
        )?,
        wrap_parse_error(
            parts
                .next()
                .ok_or_else(|| anyhow!("malformed triple"))?
                .parse(),
        )?,
    ))
}

// grabs the 2 items at ix0 and ix1, in a string separated by separator
pub fn grab_2<T0, T1>(contents: &str, separator: &str, ix0: usize, ix1: usize) -> Result<(T0, T1)>
where
    T0: std::str::FromStr,
    <T0 as std::str::FromStr>::Err: std::fmt::Display,
    T1: std::str::FromStr,
    <T1 as std::str::FromStr>::Err: std::fmt::Display,
{
    let mut parts = contents.split(separator);
    let mut ix = 0;
    while ix < ix0 {
        parts
            .next()
            .ok_or_else(|| anyhow!("malformed line, could not get index {}", ix0))?;

        ix += 1;
    }
    let first = wrap_parse_error(
        parts
            .next()
            .ok_or_else(|| anyhow!("malformed line, could not get index {}", ix0))?
            .parse(),
    )?;
    ix += 1;
    while ix < ix1 {
        parts
            .next()
            .ok_or_else(|| anyhow!("malformed line, could not get index {}", ix1))?;

        ix += 1;
    }

    Ok((
        first,
        wrap_parse_error(
            parts
                .next()
                .ok_or_else(|| anyhow!("malformed line, could not get index {}", ix1))?
                .parse(),
        )?,
    ))
}

// grabs the 3 items at ix0, ix1, and ix2, in a string separated by separator
pub fn grab_3<T0, T1, T2>(
    contents: &str,
    separator: &str,
    ix0: usize,
    ix1: usize,
    ix2: usize,
) -> Result<(T0, T1, T2)>
where
    T0: std::str::FromStr,
    <T0 as std::str::FromStr>::Err: std::fmt::Display,
    T1: std::str::FromStr,
    <T1 as std::str::FromStr>::Err: std::fmt::Display,
    T2: std::str::FromStr,
    <T2 as std::str::FromStr>::Err: std::fmt::Display,
{
    let mut parts = contents.split(separator);
    let mut ix = 0;
    while ix < ix0 {
        parts
            .next()
            .ok_or_else(|| anyhow!("malformed line, could not get index {}", ix0))?;

        ix += 1;
    }
    let first = wrap_parse_error(
        parts
            .next()
            .ok_or_else(|| anyhow!("malformed line, could not get index {}", ix0))?
            .parse(),
    )?;
    ix += 1;
    while ix < ix1 {
        parts
            .next()
            .ok_or_else(|| anyhow!("malformed line, could not get index {}", ix1))?;

        ix += 1;
    }
    let second = wrap_parse_error(
        parts
            .next()
            .ok_or_else(|| anyhow!("malformed line, could not get index {}", ix1))?
            .parse(),
    )?;
    ix += 1;

    while ix < ix2 {
        parts
            .next()
            .ok_or_else(|| anyhow!("malformed line, could not get index {}", ix2))?;

        ix += 1;
    }

    Ok((
        first,
        second,
        wrap_parse_error(
            parts
                .next()
                .ok_or_else(|| anyhow!("malformed line, could not get index {}", ix2))?
                .parse(),
        )?,
    ))
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
                c => panic!("unexpected char '{c}' in hex string"),
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
                nib => panic!("unexpected nibble sequence \"{nib}\" in binary string"),
            }
            .to_string()
        })
        .collect()
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
impl Display for HumanDuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let secs = self.0.as_secs();

        if secs > 3600 {
            write!(f, "{}h {}m {}s", secs / 3600, (secs % 3600) / 60, secs % 60)
        } else if secs > 60 {
            write!(f, "{}m {}s", secs / 60, secs % 60)
        } else if secs >= 1 {
            write!(f, "{:.3}s ({}ms)", self.0.as_secs_f32(), self.0.as_millis())
        } else if self.0.as_millis() > 1 {
            write!(f, "{}ms ({}µs)", self.0.as_millis(), self.0.as_micros())
        } else {
            write!(f, "{}µs ({}ns)", self.0.as_micros(), self.0.as_nanos())
        }
    }
}

pub trait Solution<TPart1, TPart2>
where
    Self: std::marker::Sized,
    TPart1: Display,
    TPart2: Display,
{
    fn part1(&mut self) -> Result<TPart1>;
    fn part2(&self) -> Result<TPart2>;
}

pub fn go<FParse, TSolution, TPart1, TPart2>(parse: FParse) -> Result<()>
where
    FParse: Fn(&str) -> Result<TSolution>,
    TSolution: Solution<TPart1, TPart2>,
    TPart1: Display,
    TPart2: Display,
{
    let (mut solution, parse_time) = read_and_parse(parse)?;

    let part1_time = print_and_time("Part 1", || solution.part1()).context("failure in part 1")?;
    let part2_time = print_and_time("Part 2", || solution.part2()).context("failure in part 2")?;

    print_stats(parse_time, part1_time, part2_time);
    Ok(())
}
