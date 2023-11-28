use std::str::FromStr;

use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    Problem::go()
}

struct Problem {
    calories: Vec<u32>,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Problem> {
        Ok(Problem {
            calories: contents
                .split("\n\n")
                .map(|group: &str| -> Result<u32> {
                    group
                        .lines()
                        .map(|line: &str| -> Result<u32> { wrap_parse_error(line.parse()) })
                        .sum()
                })
                .collect::<Result<Vec<u32>>>()?,
        })
    }
}

impl Solution for Problem {
    type Part1 = u32;
    type Part2 = u32;

    fn part1(&mut self) -> Result<u32> {
        self.calories
            .iter()
            .max()
            .copied()
            .ok_or_else(|| anyhow!("No calories for elf"))
    }

    fn part2(&self) -> Result<u32> {
        let mut sums: Vec<u32> = self.calories.clone();
        sums.sort_unstable_by(|a, b| b.cmp(a));
        sums.truncate(3);
        Ok(sums.into_iter().sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        let mut problem = Problem::from_str(SAMPLE)?;

        let result = problem.part1()?;

        assert_eq!(24000, result);
        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let problem = Problem::from_str(SAMPLE)?;

        let result = problem.part2()?;

        assert_eq!(45000, result);
        Ok(())
    }

    const SAMPLE: &str = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";
}
