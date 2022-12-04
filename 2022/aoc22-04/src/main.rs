use std::str::FromStr;

use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    run_vec(|contents| parse_line_pairs(contents, ","), part1, part2)
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

fn part1(contents: &[(Range, Range)]) -> Result<usize> {
    Ok(contents
        .into_iter()
        .filter(|(first, second)| first.contains(&second) || second.contains(&first))
        .count())
}

fn part2(contents: &[(Range, Range)]) -> Result<usize> {
    Ok(contents
        .into_iter()
        .filter(|(first, second)| first.overlaps(&second))
        .count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        let parsed = parse_line_pairs(SAMPLE, ",")?;

        let result = part1(&parsed)?;

        assert_eq!(2, result);

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let parsed = parse_line_pairs(SAMPLE, ",")?;

        let result = part2(&parsed)?;

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
