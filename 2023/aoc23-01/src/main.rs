use std::str::FromStr;

use anyhow::*;
use aoc_common::*;

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
    type Part1 = u32;
    type Part2 = usize;

    fn part1(&mut self) -> Result<Self::Part1> {
        Ok(self
            .lines
            .iter()
            .map(|line| get_calibration_digit_value(line))
            .sum())
    }

    fn part2(&self) -> Result<Self::Part2> {
        Ok(self
            .lines
            .iter()
            .map(|line| get_calibration_value(line))
            .sum())
    }
}

fn get_calibration_digit_value(line: &str) -> u32 {
    let first_ix = line
        .find(|c: char| c.is_ascii_digit())
        .expect("no first digit");
    let last_ix = line
        .rfind(|c: char| c.is_ascii_digit())
        .expect("no last digit");

    return line
        .chars()
        .nth(first_ix)
        .map_or(0, |c| c.to_digit(10).unwrap())
        * 10
        + line
            .chars()
            .nth(last_ix)
            .map_or(0, |c| c.to_digit(10).unwrap());
}

fn get_calibration_value(line: &str) -> usize {
    let len = line.len();

    let mut ix = 0;
    let mut first = 0;
    let mut last = 0;
    while ix < len {
        let digit = if &line[ix..=ix] == "1" || line[ix..len].starts_with("one") {
            1
        } else if &line[ix..=ix] == "2" || line[ix..len].starts_with("two") {
            2
        } else if &line[ix..=ix] == "3" || line[ix..len].starts_with("three") {
            3
        } else if &line[ix..=ix] == "4" || line[ix..len].starts_with("four") {
            4
        } else if &line[ix..=ix] == "5" || line[ix..len].starts_with("five") {
            5
        } else if &line[ix..=ix] == "6" || line[ix..len].starts_with("six") {
            6
        } else if &line[ix..=ix] == "7" || line[ix..len].starts_with("seven") {
            7
        } else if &line[ix..=ix] == "8" || line[ix..len].starts_with("eight") {
            8
        } else if &line[ix..=ix] == "9" || line[ix..len].starts_with("nine") {
            9
        } else {
            0
        };

        if digit != 0 {
            if first == 0 {
                first = digit;
            }
            last = digit;
        }

        ix += 1
    }

    first * 10 + last
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
