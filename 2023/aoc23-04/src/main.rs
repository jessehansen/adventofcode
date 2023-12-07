use std::{collections::HashSet, str::FromStr};

use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    Problem::go()
}

struct Card {
    id: usize,
    numbers: Vec<usize>,
    winning_numbers: Vec<usize>,
}

impl FromStr for Card {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        if !contents.starts_with("Card ") {
            bail!("invalid card, no id");
        }

        let (_, rest) = contents.split_once(' ').ok_or_invalid()?;
        let (id_str, rest) = rest.split_once(": ").ok_or_invalid()?;
        let (winning_numbers, numbers) = rest.split_once(" | ").ok_or_invalid()?;

        Ok(Self {
            id: id_str.trim_start().parse_wrapped()?,
            numbers: parse_split_ignore_empty(numbers, ' ')?,
            winning_numbers: parse_split_ignore_empty(winning_numbers, ' ')?,
        })
    }
}

impl Card {
    fn win_count(&self) -> usize {
        self.numbers
            .iter()
            .copied()
            .collect::<HashSet<usize>>()
            .intersection(&self.winning_numbers.iter().copied().collect())
            .count()
    }

    fn score(&self) -> usize {
        match self.win_count() {
            0 => 0,
            c => 1 << (c - 1),
        }
    }
}

struct Problem {
    cards: Vec<Card>,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        Ok(Self {
            cards: parse_lines(contents)?,
        })
    }
}

impl Solution for Problem {
    type Part1 = usize;
    type Part2 = usize;

    fn part1(&mut self) -> Result<Self::Part1> {
        Ok(self.cards.iter().map(|c| c.score()).sum())
    }

    fn part2(&self) -> Result<Self::Part2> {
        let mut copy_counts = vec![1; self.cards.len() + 1];
        copy_counts[0] = 0;

        for card in &self.cards {
            let win_count = card.win_count();
            for id in (card.id + 1)..(card.id + 1 + win_count) {
                copy_counts[id] += copy_counts[card.id];
            }
        }

        Ok(copy_counts.iter().sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        let mut problem = Problem::from_str(SAMPLE)?;

        let result = problem.part1()?;

        assert_eq!(13, result);

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let problem = Problem::from_str(SAMPLE)?;

        let result = problem.part2()?;

        assert_eq!(30, result);

        Ok(())
    }

    const SAMPLE: &str = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";
}
