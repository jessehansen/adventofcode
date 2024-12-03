use std::str::FromStr;

use anyhow::*;
use aoc_common::*;
use lazy_static::lazy_static;
use regex::Regex;

fn main() -> Result<()> {
    Problem::go()
}

struct Problem {
    input: String,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        Ok(Self {
            input: contents.to_string(),
        })
    }
}

impl Solution for Problem {
    type Part1 = usize;
    type Part2 = usize;

    fn part1(&mut self) -> Result<Self::Part1> {
        lazy_static! {
            static ref MUL_CALL: Regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        }
        Ok(MUL_CALL
            .captures_iter(&self.input)
            .map(|c| {
                let (_, [a, b]) = c.extract();
                // don't need safety here since regex is strict
                a.parse::<usize>().unwrap() * b.parse::<usize>().unwrap()
            })
            .sum::<usize>())
    }

    fn part2(&self) -> Result<Self::Part2> {
        lazy_static! {
            static ref VALID_CALL: Regex = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)").unwrap();
        }
        let mut sum = 0;
        let mut enabled = true;
        for m in VALID_CALL.find_iter(&self.input) {
            match m.as_str() {
                "do()" => {
                    enabled = true;
                }
                "don't()" => {
                    enabled = false;
                }
                mul_call if enabled => {
                    // don't need safety here since regex is strict
                    let args: Vec<usize> = mul_call
                        .substring(4, mul_call.len() - 5)
                        .parse_split(",")
                        .unwrap();
                    sum += args[0] * args[1];
                }
                _ => (),
            };
        }
        Ok(sum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        let mut problem = Problem::from_str(SAMPLE_1)?;

        let result = problem.part1()?;

        assert_eq!(161, result);

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let problem = Problem::from_str(SAMPLE_2)?;

        let result = problem.part2()?;

        assert_eq!(48, result);

        Ok(())
    }

    const SAMPLE_1: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    const SAMPLE_2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))<D-b>";
}
