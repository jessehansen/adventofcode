use std::{collections::HashMap, str::FromStr};

use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    Problem::go()
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
enum Step {
    Left,
    Right,
}
use Step::*;

impl FromStr for Step {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        match contents {
            "L" => Ok(Left),
            "R" => Ok(Right),
            c => bail!("invalid step {c}"),
        }
    }
}

struct Problem {
    steps: Vec<Step>,
    locs: HashMap<String, (String, String)>,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        let (steps, locs) = contents.split_once("\n\n").ok_or_invalid()?;
        Ok(Self {
            steps: steps.parse_chars()?,
            locs: locs
                .lines()
                .map(|line| {
                    let (loc, rest) = line.split_once(" = ").unwrap();
                    let (left, right) = rest.trim_matches(&['(', ')']).split_once(", ").unwrap();
                    (loc.to_string(), (left.to_string(), right.to_string()))
                })
                .collect(),
        })
    }
}

impl Solution for Problem {
    type Part1 = usize;
    type Part2 = usize;

    fn part1(&mut self) -> Result<Self::Part1> {
        Ok(self.steps_to_end("AAA", "ZZZ"))
    }

    fn part2(&self) -> Result<Self::Part2> {
        let step_counts: Vec<usize> = self
            .locs
            .keys()
            .filter(|x| x.ends_with('A'))
            .map(|start| self.steps_to_end(start, "Z"))
            .collect();
        Ok(least_common_multiple(&step_counts))
    }
}

impl Problem {
    fn steps_to_end(&self, start: &str, end: &str) -> usize {
        let mut loc = &start.to_string();
        let mut step = self.steps.iter().cycle();
        let mut step_count = 0;

        while !loc.ends_with(end) {
            let next_step = step.next().unwrap();
            match next_step {
                Left => {
                    loc = &self.locs[loc].0;
                }
                Right => {
                    loc = &self.locs[loc].1;
                }
            }
            step_count += 1;
        }

        step_count
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
    fn sample2_part1() -> Result<()> {
        let mut problem = Problem::from_str(SAMPLE2)?;

        let result = problem.part1()?;

        assert_eq!(6, result);

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let problem = Problem::from_str(SAMPLE3)?;

        let result = problem.part2()?;

        assert_eq!(6, result);

        Ok(())
    }

    const SAMPLE: &str = "\
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const SAMPLE2: &str = "\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    const SAMPLE3: &str = "\
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
}
