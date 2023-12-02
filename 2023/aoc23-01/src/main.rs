use std::str::FromStr;

use anyhow::*;
use aoc_common::*;
use regex::Regex;

fn main() -> Result<()> {
    Problem::go()
}

struct Problem {
    lines: Vec<String>,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        Ok(Self {
            lines: contents.lines().map(|x| x.to_string()).collect(),
        })
    }
}

impl Solution for Problem {
    type Part1 = usize;
    type Part2 = usize;

    fn part1(&mut self) -> Result<Self::Part1> {
        Ok(self
            .lines
            .iter()
            .map(|x| first_and_last_digits(x))
            .filter_map(|x| x.ok())
            .sum())
    }

    fn part2(&self) -> Result<Self::Part2> {
        let re = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|[0-9])").unwrap();

        Ok(self
            .lines
            .iter()
            .map(|x| {
                let nums: Vec<_> = overlapping_matches(x, &re);
                let first = nums.first().ok_or_else(|| anyhow!("no first digit"))?;
                let last = nums
                    .iter()
                    .last()
                    .ok_or_else(|| anyhow!("no first digit"))?;
                let first_last = [*first, *last]
                    .into_iter()
                    .map(to_digit)
                    .collect::<String>();
                wrap_parse_error(first_last.parse::<usize>())
            })
            .filter_map(|x| x.ok())
            .sum())
    }
}

fn to_digit(s: &str) -> &str {
    match s {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => s,
    }
}

fn first_and_last_digits(line: &str) -> Result<usize> {
    let digits = line
        .chars()
        .filter(|c| c.is_ascii_digit())
        .to_owned()
        .collect::<Vec<char>>();
    let first = digits.first().ok_or_else(|| anyhow!("no first digit"))?;
    let last = digits
        .iter()
        .last()
        .ok_or_else(|| anyhow!("no first digit"))?;
    let first_last = [*first, *last].into_iter().collect::<String>();
    wrap_parse_error(first_last.parse::<usize>())
}

fn overlapping_matches<'a>(haystack: &'a str, re: &'a Regex) -> Vec<&'a str> {
    let mut res = vec![];
    let mut ix = 0;
    let len = haystack.len();

    while ix < len {
        match re.find_at(haystack, ix) {
            Some(ma) => {
                res.push(ma.as_str());
                ix += 1;
            }
            None => break,
        };
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        let mut problem = Problem::from_str(SAMPLE)?;

        let result = problem.part1()?;

        assert_eq!(142, result);

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let problem = Problem::from_str(SAMPLE_2)?;

        let result = problem.part2()?;

        assert_eq!(281, result);

        Ok(())
    }

    const SAMPLE: &str = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";
    const SAMPLE_2: &str = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";
}
