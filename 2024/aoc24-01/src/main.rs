use std::str::FromStr;

use anyhow::*;
use aoc_common::*;

use std::collections::HashMap;

fn main() -> Result<()> {
    Problem::go()
}

struct Problem {
    list1: Vec<u32>,
    list2: Vec<u32>,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        let mut list1 = Vec::new();
        let mut list2 = Vec::new();
        for line in contents.lines() {
            let (a, b) = line.split_once("   ").ok_or_invalid()?;
            list1.push(a.parse_wrapped()?);
            list2.push(b.parse_wrapped()?);
        }
        list1.sort_unstable();
        list2.sort_unstable();
        Ok(Problem { list1, list2 })
    }
}

impl Solution for Problem {
    type Part1 = u32;
    type Part2 = u32;

    fn part1(&mut self) -> Result<Self::Part1> {
        Ok(self
            .list1
            .iter()
            .zip(self.list2.iter())
            .map(|(&a, &b)| a.abs_diff(b))
            .sum())
    }

    fn part2(&self) -> Result<Self::Part2> {
        let mut counts: HashMap<u32, u32> = HashMap::new();
        for &b in &self.list2 {
            *counts.entry(b).or_insert(0) += 1;
        }

        Ok(self
            .list1
            .iter()
            .map(|a| counts.get(a).unwrap_or(&0) * a)
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

        assert_eq!(11, result);

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let problem = Problem::from_str(SAMPLE)?;

        let result = problem.part2()?;

        assert_eq!(31, result);

        Ok(())
    }

    const SAMPLE: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";
}
