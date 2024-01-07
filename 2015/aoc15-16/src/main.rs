use std::str::FromStr;

use anyhow::*;
use aoc_common::*;
use fnv::FnvHashMap;

fn main() -> Result<()> {
    Problem::go()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Field {
    Children,
    Cats,
    Samoyeds,
    Pomeranians,
    Akitas,
    Vizslas,
    Goldfish,
    Trees,
    Cars,
    Perfumes,
}
use Field::*;

impl FromStr for Field {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        match contents {
            "children" => Ok(Children),
            "cats" => Ok(Cats),
            "samoyeds" => Ok(Samoyeds),
            "pomeranians" => Ok(Pomeranians),
            "akitas" => Ok(Akitas),
            "vizslas" => Ok(Vizslas),
            "goldfish" => Ok(Goldfish),
            "trees" => Ok(Trees),
            "cars" => Ok(Cars),
            "perfumes" => Ok(Perfumes),
            _ => bail!("invalid"),
        }
    }
}

impl Field {
    fn part_2_matches(&self, sue_value: usize, ticker_value: usize) -> bool {
        match self {
            Cats | Trees => sue_value > ticker_value,
            Pomeranians | Goldfish => sue_value < ticker_value,
            _ => sue_value == ticker_value,
        }
    }
}

struct Problem {
    aunts: Vec<FnvHashMap<Field, usize>>,
    ticker: FnvHashMap<Field, usize>,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        let mut ticker = FnvHashMap::default();
        ticker.insert(Children, 3usize);
        ticker.insert(Cats, 7usize);
        ticker.insert(Samoyeds, 2usize);
        ticker.insert(Pomeranians, 3usize);
        ticker.insert(Akitas, 0usize);
        ticker.insert(Vizslas, 0usize);
        ticker.insert(Goldfish, 5usize);
        ticker.insert(Trees, 3usize);
        ticker.insert(Cars, 2usize);
        ticker.insert(Perfumes, 1usize);
        Ok(Self {
            aunts: contents
                .lines()
                .map(|line| {
                    let (_, line) = line.split_once(": ").ok_or_invalid()?;
                    Ok(line
                        .split(", ")
                        .map(|field| -> Result<(Field, usize)> {
                            let (field, value) = field.split_once(": ").ok_or_invalid()?;
                            Ok((field.parse_wrapped()?, value.parse_wrapped()?))
                        })
                        .collect::<Result<FnvHashMap<Field, usize>>>()?)
                })
                .collect::<Result<Vec<FnvHashMap<Field, usize>>>>()?,
            ticker,
        })
    }
}

impl Solution for Problem {
    type Part1 = usize;
    type Part2 = usize;

    fn part1(&mut self) -> Result<Self::Part1> {
        for id in 1..=(self.aunts.len()) {
            if self.aunts[id - 1]
                .iter()
                .all(|(field, value)| self.ticker[field] == *value)
            {
                return Ok(id);
            }
        }
        bail!("couldn't find sue")
    }

    fn part2(&self) -> Result<Self::Part2> {
        for id in 1..=(self.aunts.len()) {
            if self.aunts[id - 1]
                .iter()
                .all(|(field, value)| field.part_2_matches(*value, self.ticker[field]))
            {
                return Ok(id);
            }
        }
        bail!("couldn't find sue")
    }
}
