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
        Ok(self
            .lines
            .iter()
            .map(|x| {
                let digits = find_digits(x);

                let first = digits.first().ok_or_else(|| anyhow!("no first digit"))?;
                let last = digits
                    .iter()
                    .last()
                    .ok_or_else(|| anyhow!("no first digit"))?;

                Ok(first * 10 + last)
            })
            .filter_map(|x| x.ok())
            .sum())
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

fn find_digits(line: &str) -> Vec<usize> {
    let mut res = vec![];
    let len = line.len();

    let mut ix = 0;
    while ix < len {
        if &line[ix..=ix] == "0" {
            res.push(0);
        } else if &line[ix..=ix] == "1" || line[ix..len].starts_with("one") {
            res.push(1);
        } else if &line[ix..=ix] == "2" || line[ix..len].starts_with("two") {
            res.push(2);
        } else if &line[ix..=ix] == "3" || line[ix..len].starts_with("three") {
            res.push(3);
        } else if &line[ix..=ix] == "4" || line[ix..len].starts_with("four") {
            res.push(4);
        } else if &line[ix..=ix] == "5" || line[ix..len].starts_with("five") {
            res.push(5);
        } else if &line[ix..=ix] == "6" || line[ix..len].starts_with("six") {
            res.push(6);
        } else if &line[ix..=ix] == "7" || line[ix..len].starts_with("seven") {
            res.push(7);
        } else if &line[ix..=ix] == "8" || line[ix..len].starts_with("eight") {
            res.push(8);
        } else if &line[ix..=ix] == "9" || line[ix..len].starts_with("nine") {
            res.push(9);
        }

        ix += 1
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
