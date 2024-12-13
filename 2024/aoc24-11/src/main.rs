use std::{collections::HashMap, str::FromStr};

use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    Problem::go()
}

struct Problem {
    stones: Vec<u64>,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        Ok(Self {
            stones: contents.parse_split_whitespace()?,
        })
    }
}

fn blink_single_stone(stone: u64) -> [Option<u64>; 2] {
    if stone == 0 {
        [Some(1), None]
    } else {
        let s = format!("{stone}");
        if s.len() % 2 == 0 {
            let (left, right) = s.split_at(s.len() / 2);
            [left.parse().ok(), right.parse().ok()]
        } else {
            [Some(stone * 2024), None]
        }
    }
}

fn count_stones_after_blinks(
    stone: u64,
    blinks: usize,
    cache: &mut HashMap<(u64, usize), u64>,
) -> u64 {
    if blinks == 0 {
        return 1;
    }

    if let Some(entry) = cache.get(&(stone, blinks)) {
        *entry
    } else {
        let total = blink_single_stone(stone)
            .into_iter()
            .flatten()
            .map(|s| count_stones_after_blinks(s, blinks - 1, cache))
            .sum();

        cache.insert((stone, blinks), total);
        total
    }
}
impl Problem {
    fn blink(&self, blinks: usize) -> u64 {
        let mut cache = HashMap::new();

        self.stones
            .iter()
            .map(|s| count_stones_after_blinks(*s, blinks, &mut cache))
            .sum()
    }
}

impl Solution for Problem {
    type Part1 = u64;
    type Part2 = u64;

    fn part1(&mut self) -> Result<Self::Part1> {
        Ok(self.blink(25))
    }

    fn part2(&self) -> Result<Self::Part2> {
        Ok(self.blink(75))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_blink_multi() -> Result<()> {
        let problem = Problem::from_str(SAMPLE)?;

        assert_eq!(22, problem.blink(6));
        assert_eq!(55312, problem.blink(25));

        Ok(())
    }

    const SAMPLE: &str = "125 17";
}
