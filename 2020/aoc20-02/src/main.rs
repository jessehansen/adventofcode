use std::str::FromStr;

use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    Problem::go()
}

struct Problem {
    passwords: Vec<PasswordAndPolicy>,
}

struct PasswordAndPolicy {
    policy: Policy,
    password: String,
}

impl FromStr for PasswordAndPolicy {
    type Err = Error;

    fn from_str(line: &str) -> Result<PasswordAndPolicy> {
        let mut parts = line.split(": ");
        Ok(PasswordAndPolicy {
            policy: parts
                .next()
                .ok_or_else(|| anyhow!("invalid line, missing policy"))?
                .parse()?,
            password: parts
                .next()
                .ok_or_else(|| anyhow!("invalid line, missing password"))?
                .to_owned(),
        })
    }
}

struct Policy {
    character: char,
    min: usize,
    max: usize,
}

impl FromStr for Policy {
    type Err = Error;

    fn from_str(policy: &str) -> Result<Self> {
        let mut parts = policy.split(' ');
        let mut min_max = parts
            .next()
            .ok_or_else(|| anyhow!("invalid policy, missing min & max"))?
            .split('-');

        Ok(Policy {
            min: min_max
                .next()
                .ok_or_else(|| anyhow!("invalid policy, no min"))?
                .parse()?,
            max: min_max
                .next()
                .ok_or_else(|| anyhow!("invalid policy, no max"))?
                .parse()?,
            character: parts
                .next()
                .ok_or_else(|| anyhow!("invalid policy, missing character"))?
                .chars()
                .next()
                .ok_or_else(|| anyhow!("invalid policy, missing character"))?,
        })
    }
}

impl Policy {
    pub fn validate_part1(&self, password: &str) -> bool {
        let count = password.chars().filter(|c| c == &self.character).count();

        count >= self.min && count <= self.max
    }

    pub fn validate_part2(&self, password: &str) -> bool {
        let min_char = password.chars().nth(self.min - 1);
        let max_char = password.chars().nth(self.max - 1);
        if min_char == Some(self.character) {
            max_char != Some(self.character)
        } else {
            max_char == Some(self.character)
        }
    }
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Problem> {
        Ok(Problem {
            passwords: parse_lines(contents)?,
        })
    }
}

impl Solution for Problem {
    type Part1 = usize;
    type Part2 = usize;

    fn part1(&mut self) -> Result<Self::Part1> {
        Ok(self
            .passwords
            .iter()
            .filter(|pp| pp.policy.validate_part1(&pp.password))
            .count())
    }

    fn part2(&self) -> Result<Self::Part2> {
        Ok(self
            .passwords
            .iter()
            .filter(|pp| pp.policy.validate_part2(&pp.password))
            .count())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        let mut problem = Problem::from_str(SAMPLE)?;

        let result = problem.part1()?;

        assert_eq!(2, result);

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let problem = Problem::from_str(SAMPLE)?;

        let result = problem.part2()?;

        assert_eq!(1, result);

        Ok(())
    }

    const SAMPLE: &str = "\
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
";
}
