use std::str::FromStr;

use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    Problem::go()
}

struct Range {
    start: u32,
    end: u32,
}

impl FromStr for Range {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = parse_pair(s, "-")?;

        Ok(Range { start, end })
    }
}

impl Range {
    pub fn contains(&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.end
    }
    pub fn overlaps(&self, other: &Range) -> bool {
        self.end >= other.start && other.end >= self.start
    }
}

struct Problem {
    ranges: Vec<(Range, Range)>,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Problem> {
        Ok(Problem {
            ranges: parse_line_pairs(contents, ",")?,
        })
    }
}
impl Solution for Problem {
    type Part1 = usize;
    type Part2 = usize;

    fn part1(&mut self) -> Result<usize> {
        Ok(self
            .ranges
            .iter()
            .filter(|(first, second)| first.contains(second) || second.contains(first))
            .count())
    }

    fn part2(&self) -> Result<usize> {
        Ok(self
            .ranges
            .iter()
            .filter(|(first, second)| first.overlaps(second))
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

        assert_eq!(4, result);

        Ok(())
    }

    const SAMPLE: &str = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";
}
