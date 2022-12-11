use std::str::FromStr;

use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    go(Problem::parse)
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Move {
    Rock,
    Paper,
    Scizzors,
}

impl Move {
    pub fn score(self: &Move) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scizzors => 3,
        }
    }

    pub fn beats(self: &Move) -> Move {
        match self {
            Move::Rock => Move::Scizzors,
            Move::Paper => Move::Rock,
            Move::Scizzors => Move::Paper,
        }
    }

    pub fn loses(self: &Move) -> Move {
        match self {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scizzors,
            Move::Scizzors => Move::Rock,
        }
    }
}

impl FromStr for Move {
    type Err = Error;

    fn from_str(input: &str) -> Result<Move> {
        match input {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scizzors),
            _ => Err(anyhow!("invalid move")),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

impl Outcome {
    pub fn should_move(self: &Outcome, opponent: &Move) -> Move {
        match self {
            Outcome::Lose => opponent.beats(),
            Outcome::Draw => *opponent,
            Outcome::Win => opponent.loses(),
        }
    }

    pub fn score(self: &Outcome) -> u32 {
        match self {
            Outcome::Lose => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}

impl FromStr for Outcome {
    type Err = Error;

    fn from_str(input: &str) -> Result<Outcome> {
        match input {
            "X" => Ok(Outcome::Lose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err(anyhow!("invalid move")),
        }
    }
}

struct Round {
    opponent: Move,
    maybe_me: Move,
    outcome: Outcome,
}

impl FromStr for Round {
    type Err = Error;

    fn from_str(input: &str) -> Result<Round> {
        let parts: Vec<&str> = input.split_whitespace().collect();

        if parts.len() != 2 {
            bail!("invalid line, expected 2 parts, got {}", parts.len());
        }

        Ok(Round {
            opponent: parts[0].parse()?,
            maybe_me: parts[1].parse()?,
            outcome: parts[1].parse()?,
        })
    }
}

impl Round {
    pub fn maybe_outcome(self: &Round) -> Outcome {
        if self.maybe_me == self.opponent {
            Outcome::Draw
        } else if self.maybe_me.beats() == self.opponent {
            Outcome::Win
        } else {
            Outcome::Lose
        }
    }

    pub fn score_part_1(self: &Round) -> u32 {
        self.maybe_outcome().score() + self.maybe_me.score()
    }

    pub fn score_part_2(self: &Round) -> u32 {
        self.outcome.score() + self.outcome.should_move(&self.opponent).score()
    }
}

struct Problem {
    rounds: Vec<Round>,
}

impl Problem {
    fn parse(contents: &str) -> Result<Problem> {
        Ok(Problem {
            rounds: parse_lines(contents)?,
        })
    }
}

impl Solution<u32, u32> for Problem {
    fn part1(&mut self) -> Result<u32> {
        Ok(self.rounds.iter().map(|x| x.score_part_1()).sum())
    }

    fn part2(&self) -> Result<u32> {
        Ok(self.rounds.iter().map(|x| x.score_part_2()).sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        let mut problem = Problem::parse(SAMPLE)?;

        let result = problem.part1()?;

        assert_eq!(15, result);

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let problem = Problem::parse(SAMPLE)?;

        let result = problem.part2()?;

        assert_eq!(12, result);

        Ok(())
    }

    const SAMPLE: &str = "\
A Y
B X
C Z
";
}
