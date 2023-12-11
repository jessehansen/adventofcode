use std::str::FromStr;

use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    Problem::go()
}

#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    fn ways_to_win(&self) -> usize {
        let mut wins = 0;
        for hold_time in 1..self.time {
            let final_distance = (self.time - hold_time) * hold_time;
            if final_distance > self.distance {
                wins += 1;
            }
        }
        wins
    }
}

#[derive(Debug)]
struct Problem {
    races: Vec<Race>,
    actual_race: Race,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        let (times, distances) = contents.split_once('\n').ok_or_invalid()?;

        let (_, times) = times.split_at(10);
        let (_, distances) = distances.split_at(10);

        let actual_times: String = times.chars().filter(|c| c.is_ascii_digit()).collect();
        let actual_distances: String = distances.chars().filter(|c| c.is_ascii_digit()).collect();

        let times: Vec<usize> = times.trim().parse_split_whitespace()?;
        let distances: Vec<usize> = distances.trim().parse_split_whitespace()?;

        Ok(Self {
            races: times
                .into_iter()
                .zip(distances)
                .map(|(time, distance)| Race { time, distance })
                .collect(),
            actual_race: Race {
                time: actual_times.parse_wrapped()?,
                distance: actual_distances.parse_wrapped()?,
            },
        })
    }
}

impl Solution for Problem {
    type Part1 = usize;
    type Part2 = usize;

    fn part1(&mut self) -> Result<Self::Part1> {
        Ok(self.races.iter().map(|r| r.ways_to_win()).product())
    }

    fn part2(&self) -> Result<Self::Part2> {
        Ok(self.actual_race.ways_to_win())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        let mut problem = Problem::from_str(SAMPLE)?;

        let result = problem.part1()?;

        assert_eq!(288, result);

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let problem = Problem::from_str(SAMPLE)?;

        let result = problem.part2()?;

        assert_eq!(71503, result);

        Ok(())
    }

    const SAMPLE: &str = "\
Time:      7  15   30
Distance:  9  40  200
";
}
