use itertools::{Itertools, repeat_n};
use std::str::FromStr;

use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    Problem::go()
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Multiply,
    Concatenate,
}
use Operator::*;

#[derive(Debug)]
struct Equation {
    result: u64,
    params: Vec<u64>,
}

struct Problem {
    equations: Vec<Equation>,
}

impl FromStr for Equation {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        let (result, params) = contents.split_once(": ").ok_or_invalid()?;
        Ok(Self {
            result: result.parse_wrapped()?,
            params: params.parse_split_whitespace()?,
        })
    }
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        Ok(Self {
            equations: contents.parse_lines()?,
        })
    }
}

impl Operator {
    fn execute(&self, left: u64, right: &u64) -> u64 {
        match self {
            Add => left + right,
            Multiply => left * right,
            Concatenate => format!("{left}{right}").parse().unwrap(),
        }
    }
}

impl Equation {
    fn can_be_true_with_operations(&self, candidate_ops: &[Operator]) -> bool {
        repeat_n(candidate_ops, self.params.len() - 1)
            .multi_cartesian_product()
            .any(|ops| self.is_true(&ops))
    }

    fn is_true(&self, ops: &[&Operator]) -> bool {
        ops.iter()
            .zip(&self.params[1..])
            .fold(self.params[0], |acc, (op, param)| op.execute(acc, param))
            == self.result
    }
}

impl Solution for Problem {
    type Part1 = u64;
    type Part2 = u64;

    fn part1(&mut self) -> Result<Self::Part1> {
        Ok(self
            .equations
            .iter()
            .filter(|eq| eq.can_be_true_with_operations(&[Add, Multiply]))
            .map(|eq| eq.result)
            .sum())
    }

    fn part2(&self) -> Result<Self::Part2> {
        Ok(self
            .equations
            .iter()
            .filter(|eq| eq.can_be_true_with_operations(&[Add, Multiply, Concatenate]))
            .map(|eq| eq.result)
            .sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        let mut problem = Problem::from_str(SAMPLE)?;

        let result = problem.part1()?;

        assert_eq!(3749, result);

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let problem = Problem::from_str(SAMPLE)?;

        let result = problem.part2()?;

        assert_eq!(11387, result);

        Ok(())
    }

    const SAMPLE: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";
}
