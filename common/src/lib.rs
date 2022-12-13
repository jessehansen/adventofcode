#![feature(pattern)]

use std::fmt::{self, Display};
use std::path::Path;
use std::time::{Duration, Instant};
use std::{env, fs};

use anyhow::*;
use console::{style, Term};
use reqwest::blocking::Client;

mod parse;
pub use parse::*;

mod grid;
pub use grid::*;

mod plot;
pub use plot::*;

mod tree;
pub use tree::*;

mod legacy;
pub use legacy::*;

pub trait Solution: std::str::FromStr
where
    <Self as std::str::FromStr>::Err: std::fmt::Display,
{
    type Part1: Display;
    type Part2: Display;

    fn part1(&mut self) -> Result<Self::Part1>;
    fn part2(&self) -> Result<Self::Part2>;

    fn go() -> Result<()> {
        let (mut solution, parse_time) = read_and_parse(parse_all::<Self>)?;

        let part1_time =
            print_and_time("Part 1", || solution.part1()).context("failure in part 1")?;
        let part2_time =
            print_and_time("Part 2", || solution.part2()).context("failure in part 2")?;

        print_stats(parse_time, part1_time, part2_time);
        Ok(())
    }
}

fn get_year_and_day() -> Result<(usize, usize)> {
    let binding = env::current_dir()?;
    let path = binding
        .file_name()
        .ok_or_else(|| anyhow!("missing current dir"))?
        .to_str()
        .ok_or_else(|| anyhow!("converting current dir to str failed"))?;
    // path should be aocyy-dd
    let year = 2000 + path[3..=4].parse::<usize>()?;
    let day = path[6..=7].parse::<usize>()?;

    Ok((year, day))
}

fn download_input() -> Result<String> {
    match env::var("AOC_COOKIE") {
        std::result::Result::Ok(cookie) => {
            println!("Input missing, attempting download");
            let (year, day) = get_year_and_day()?;

            let input_url = format!("https://adventofcode.com/{year}/day/{day}/input");
            println!("Year {year}, Day {day}: {input_url}");

            let client = Client::new();
            let res = client
                .get(input_url)
                .header(reqwest::header::COOKIE, cookie)
                .header(
                    reqwest::header::USER_AGENT,
                    "github.com/jessehansen/adventofcode by jesse@twindagger.com",
                )
                .send()?;

            if res.status().is_success() {
                println!("Success! Saving input.txt...");
                let input = res.text()?;

                fs::write("./input.txt", input.clone())?;
                Ok(input)
            } else {
                bail!("Could not download input - got response {:?}", res.status());
            }
        }
        Err(_) => {
            bail!("Missing AOC_COOKIE environment variable, couldn't download input");
        }
    }
}

fn read_and_parse<T, F>(parse: F) -> Result<(T, Duration)>
where
    F: Fn(&str) -> Result<T>,
{
    let input = if Path::new("./input.txt").is_file() {
        fs::read_to_string("./input.txt").context("could not read input.txt")?
    } else {
        download_input()?
    };

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
