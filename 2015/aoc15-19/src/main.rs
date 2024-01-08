use std::{cmp::Reverse, str::FromStr};

use anyhow::*;
use aoc_common::*;
use fnv::FnvHashSet;

fn main() -> Result<()> {
    Problem::go()
}

struct Problem {
    replacements: Vec<(String, String)>,
    molecule: String,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        let (replacements, molecule) = contents.split_once("\n\n").ok_or_invalid()?;
        let replacements = replacements
            .lines()
            .map(|line| {
                let (input, output) = line.split_once(" => ").ok_or_invalid()?;

                Ok((input.to_string(), output.to_string()))
            })
            .collect::<Result<_>>()?;
        Ok(Self {
            replacements,
            molecule: molecule.to_string(),
        })
    }
}

impl Problem {
    fn next(&self, state: &str) -> FnvHashSet<String> {
        let mut outputs = FnvHashSet::default();

        for (input, output) in &self.replacements {
            for (location, _) in state.match_indices(input.as_str()) {
                let mut next = String::new();
                next.push_str(&state[..location]);
                next.push_str(output.as_str());
                next.push_str(&state[(location + input.len())..]);
                outputs.insert(next);
            }
        }

        outputs
    }

    // general solution for part 2, never completes for problem
    /*
    fn part2_general(&self) -> Result<usize> {
        Ok(dijkstra(
            ProblemState {
                state: "e".to_string(),
                steps: 0,
            },
            |state| {
                let steps = state.steps;
                self.next(&state.state)
                    .into_iter()
                    .map(move |next| ProblemState {
                        state: next,
                        steps: steps + 1,
                    })
            },
            |state| state.state == self.molecule,
        )
        .map(|s| s.steps)
        .ok_or_invalid()?)
    }
    */
}

impl Solution for Problem {
    type Part1 = usize;
    type Part2 = usize;

    fn part1(&mut self) -> Result<Self::Part1> {
        Ok(self.next(&self.molecule).len())
    }

    fn part2(&self) -> Result<Self::Part2> {
        // by input analysis: https://www.reddit.com/r/adventofcode/comments/3xflz8/comment/cy4etju/?utm_source=share&utm_medium=web2x&context=3
        let elements = self
            .molecule
            .chars()
            .filter(|c| c.is_ascii_uppercase())
            .count();
        let rn = self.molecule.matches("Rn").count();
        let ar = self.molecule.matches("Ar").count();
        let y = self.molecule.matches('Y').count();

        Ok(elements - ar - rn - 2 * y - 1)
    }
}

struct ProblemState {
    state: String,
    steps: usize,
}

impl OptimizationState for ProblemState {
    type CacheKey = String;

    type Score = Reverse<usize>;

    fn cache_key(&self) -> Self::CacheKey {
        self.state.clone()
    }

    fn score(&self) -> Self::Score {
        Reverse(self.steps)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        let mut problem = Problem::from_str(SAMPLE)?;

        let result = problem.part1()?;

        assert_eq!(4, result);

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let problem = Problem::from_str(SAMPLE_2)?;

        let result = problem.part2_general()?;

        assert_eq!(3, result);

        Ok(())
    }

    #[test]
    fn sample_part2_longer() -> Result<()> {
        let mut problem = Problem::from_str(SAMPLE_2)?;
        problem.molecule = "HOHOHO".to_string();

        let result = problem.part2_general()?;

        assert_eq!(6, result);

        Ok(())
    }

    const SAMPLE: &str = "\
H => HO
H => OH
O => HH

HOH";

    const SAMPLE_2: &str = "\
e => H
e => O
H => HO
H => OH
O => HH

HOH";
}
