use std::str::FromStr;

use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    Problem::go()
}

struct Problem {
    entries: Vec<u32>,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Problem> {
        Ok(Problem {
            entries: parse_lines(contents)?,
        })
    }
}

impl Solution for Problem {
    type Part1 = u32;
    type Part2 = u32;

    fn part1(&mut self) -> Result<Self::Part1> {
        for entry1 in &self.entries {
            for entry2 in &self.entries {
                if entry1 + entry2 == 2020 {
                    return Ok(entry1 * entry2);
                }
            }
        }
        bail!("no valid combinations sum to 2020")
    }

    fn part2(&self) -> Result<Self::Part2> {
        for entry1 in &self.entries {
            for entry2 in &self.entries {
                for entry3 in &self.entries {
                    if entry1 + entry2 + entry3 == 2020 {
                        return Ok(entry1 * entry2 * entry3);
                    }
                }
            }
        }
        bail!("no valid combinations sum to 2020")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        let mut problem = Problem::from_str(SAMPLE)?;

        let result = problem.part1()?;

        assert_eq!(514579, result);

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let problem = Problem::from_str(SAMPLE)?;

        let result = problem.part2()?;

        assert_eq!(241861950, result);

        Ok(())
    }

    const SAMPLE: &str = "\
1721
979
366
299
675
1456
";
}
