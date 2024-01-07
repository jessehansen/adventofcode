use std::{collections::VecDeque, str::FromStr};

use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    Problem::go()
}

struct Problem {
    containers: Vec<usize>,
    combination_lengths: Vec<usize>,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        Ok(Self {
            containers: contents.parse_lines()?,
            combination_lengths: vec![],
        })
    }
}

impl Solution for Problem {
    type Part1 = usize;
    type Part2 = usize;

    fn part1(&mut self) -> Result<Self::Part1> {
        self.determine_combinations(150);
        Ok(self.combination_lengths.len())
    }

    fn part2(&self) -> Result<Self::Part2> {
        let min_len = self.combination_lengths.iter().min().ok_or_invalid()?;
        Ok(self
            .combination_lengths
            .iter()
            .filter(|&combo_len| combo_len == min_len)
            .count())
    }
}

impl Problem {
    fn determine_combinations(&mut self, target_volume: usize) {
        let mut todo = VecDeque::new();

        for (ix, c) in self.containers.iter().enumerate() {
            let mut used = vec![false; self.containers.len()];
            used[ix] = true;
            todo.push_back((ix + 1, *c, used));
        }

        while let Some((first_ix, volume, used)) = todo.pop_front() {
            if volume > target_volume {
                continue;
            }
            if volume == target_volume {
                self.combination_lengths
                    .push(used.into_iter().filter(|&used| used).count());
                continue;
            }
            for (ix, c) in self.containers.iter().enumerate().skip(first_ix) {
                if !used[ix] {
                    let mut used = used.clone();
                    used[ix] = true;
                    todo.push_back((ix + 1, volume + c, used));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_combinations() -> Result<()> {
        let mut problem = Problem::from_str(SAMPLE)?;

        problem.determine_combinations(25);

        assert_eq!(4, problem.combination_lengths.len());

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let mut problem = Problem::from_str(SAMPLE)?;
        problem.determine_combinations(25);

        let result = problem.part2()?;

        assert_eq!(3, result);

        Ok(())
    }

    const SAMPLE: &str = "\
20
15
10
5
5
";
}
