use std::str::FromStr;

use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    Problem::go()
}

struct Report {
    data: Vec<usize>,
}

impl FromStr for Report {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Self {
            data: s.parse_split_whitespace()?,
        })
    }
}

fn is_safe_pair(a: &[usize]) -> bool {
    matches!(a[0].abs_diff(a[1]), 1..=3)
}

impl Report {
    fn is_safe(&self) -> bool {
        self.data.windows(2).all(is_safe_pair)
            && (self.data.windows(2).all(|pair| pair[0] > pair[1])
                || self.data.windows(2).all(|pair| pair[0] < pair[1]))
    }

    fn combinations_omitting_datum(&self) -> impl Iterator<Item = Report> + use<'_> {
        (0..self.data.len()).map(|ix| {
            let mut data = self.data.clone();
            data.remove(ix);
            Report { data }
        })
    }
}

struct Problem {
    reports: Vec<Report>,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        Ok(Self {
            reports: contents.parse_lines()?,
        })
    }
}

impl Solution for Problem {
    type Part1 = usize;
    type Part2 = usize;

    fn part1(&mut self) -> Result<Self::Part1> {
        Ok(self.reports.iter().filter(|r| r.is_safe()).count())
    }

    fn part2(&self) -> Result<Self::Part2> {
        Ok(self
            .reports
            .iter()
            // even though we technically don't need to check is_safe again here,
            // it's actually (very slightly) faster to check & not copy the data
            .filter(|r| r.is_safe() || r.combinations_omitting_datum().any(|c| c.is_safe()))
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
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";
}
