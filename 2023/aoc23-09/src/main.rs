use std::str::FromStr;

use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    Problem::go()
}

struct Sensor {
    change_stack: Vec<Vec<i32>>,
}

struct Problem {
    sensors: Vec<Sensor>,
}

impl FromStr for Sensor {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        let mut change_stack: Vec<Vec<i32>> = vec![contents.parse_split_whitespace()?];

        loop {
            let next_line: Vec<i32> = change_stack
                .last()
                .unwrap()
                .windows(2)
                .map(|w| w[1] - w[0])
                .collect();

            if next_line.iter().any(|v| *v != 0) {
                change_stack.push(next_line);
            } else {
                break;
            }
        }

        Ok(Self { change_stack })
    }
}

impl Sensor {
    fn predict_next(&self) -> i32 {
        let mut last_change = 0;

        for i in (0..self.change_stack.len()).rev() {
            let line = &self.change_stack[i];
            last_change += line.last().unwrap();
        }

        last_change
    }

    fn predict_prev(&self) -> i32 {
        let mut prev_change = 0;

        for i in (0..self.change_stack.len()).rev() {
            let line = &self.change_stack[i];
            prev_change = line[0] - prev_change;
        }

        prev_change
    }
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        Ok(Self {
            sensors: contents.parse_lines()?,
        })
    }
}

impl Solution for Problem {
    type Part1 = i32;
    type Part2 = i32;

    fn part1(&mut self) -> Result<Self::Part1> {
        Ok(self.sensors.iter().map(|s| s.predict_next()).sum())
    }

    fn part2(&self) -> Result<Self::Part2> {
        Ok(self.sensors.iter().map(|s| s.predict_prev()).sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        let mut problem = Problem::from_str(SAMPLE)?;

        let result = problem.part1()?;

        assert_eq!(114, result);

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let problem = Problem::from_str(SAMPLE)?;

        let result = problem.part2()?;

        assert_eq!(2, result);

        Ok(())
    }

    const SAMPLE: &str = "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
}
