use std::{collections::HashMap, str::FromStr};

use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    Problem::go()
}

#[derive(Clone, Copy, Default, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
enum Spring {
    #[default]
    Broken,
    Working,
    Unknown,
}

use Spring::*;

impl FromStr for Spring {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        match contents {
            "?" => Ok(Unknown),
            "." => Ok(Working),
            "#" => Ok(Broken),
            _ => bail!("invalid spring"),
        }
    }
}

struct Line {
    springs: Vec<Spring>,
    broken_counts: Vec<usize>,
}

impl Line {
    fn possible_arrangements(&self) -> u64 {
        let mut cache = HashMap::default();
        dfs(&mut cache, &self.springs, &self.broken_counts, 0, 0, 0)
    }

    fn expand(&self) -> Line {
        let mut springs = vec![];
        for _ in 0..4 {
            springs.append(&mut self.springs.clone());
            springs.push(Unknown);
        }
        springs.append(&mut self.springs.clone());

        Line {
            springs,
            broken_counts: self.broken_counts.repeat(5),
        }
    }
}

impl FromStr for Line {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        let (springs, counts) = contents.split_once(' ').ok_or_invalid()?;
        Ok(Self {
            springs: springs.parse_chars()?,
            broken_counts: counts.parse_split(',')?,
        })
    }
}

struct Problem {
    lines: Vec<Line>,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        Ok(Self {
            lines: contents.parse_lines()?,
        })
    }
}

impl Solution for Problem {
    type Part1 = u64;
    type Part2 = u64;

    fn part1(&mut self) -> Result<Self::Part1> {
        Ok(self
            .lines
            .iter()
            .map(|line| line.possible_arrangements())
            .sum())
    }

    fn part2(&self) -> Result<Self::Part2> {
        Ok(self
            .lines
            .iter()
            .map(|line| line.expand().possible_arrangements())
            .sum())
    }
}

fn dfs(
    cache: &mut HashMap<(usize, usize, usize), u64>,
    springs: &[Spring],
    broken_counts: &[usize],
    current_spring: usize,
    current_count: usize,
    size: usize,
) -> u64 {
    if current_spring >= springs.len() {
        // exhausted all groups
        if current_count >= broken_counts.len() {
            return 1;
        }

        // the line ends with a "damaged" symbol and we've matched that last group
        if current_count == broken_counts.len() - 1 && broken_counts[current_count] == size {
            return 1;
        }

        return 0;
    }

    match springs[current_spring] {
        Working => {
            // skip sequence of operational spots
            if size == 0 {
                return dfs(
                    cache,
                    springs,
                    broken_counts,
                    current_spring + 1,
                    current_count,
                    size,
                );
            }

            // the current combination failed to match a proper sequence from the input
            if current_count >= broken_counts.len() || size != broken_counts[current_count] {
                return 0;
            }

            // we have a match: process the next group
            dfs(
                cache,
                springs,
                broken_counts,
                current_spring + 1,
                current_count + 1,
                0,
            )
        }

        Broken => {
            // we do not expect more damaged spots, thus failed to match
            if current_count >= broken_counts.len() || size + 1 > broken_counts[current_count] {
                return 0;
            }

            dfs(
                cache,
                springs,
                broken_counts,
                current_spring + 1,
                current_count,
                size + 1,
            )
        }

        Unknown => {
            if let Some(answer) = cache.get(&(current_spring, current_count, size)).copied() {
                return answer;
            }

            let mut ways = 0;

            // if we did not encounter any damaged cells,
            // we can treat this one as undamaged
            if size == 0 {
                ways += dfs(
                    cache,
                    springs,
                    broken_counts,
                    current_spring + 1,
                    current_count,
                    size,
                );
            }

            // if we need more damaged cells to complete our match,
            // we can treat the current cell as damaged
            if current_count < broken_counts.len() && size < broken_counts[current_count] {
                ways += dfs(
                    cache,
                    springs,
                    broken_counts,
                    current_spring + 1,
                    current_count,
                    size + 1,
                );
            }

            // we have the correct number of damaged cells, so we can just
            // treat this one as undamaged in order to complete the match
            if current_count < broken_counts.len() && size == broken_counts[current_count] {
                ways += dfs(
                    cache,
                    springs,
                    broken_counts,
                    current_spring + 1,
                    current_count + 1,
                    0,
                );
            }

            cache.insert((current_spring, current_count, size), ways);
            ways
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        let mut problem = Problem::from_str(SAMPLE)?;

        let result = problem.part1()?;

        assert_eq!(21, result);

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let problem = Problem::from_str(SAMPLE)?;

        let result = problem.part2()?;

        assert_eq!(525152, result);

        Ok(())
    }

    const SAMPLE: &str = "\
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
}
