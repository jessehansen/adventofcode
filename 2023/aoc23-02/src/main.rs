use std::{collections::HashMap, str::FromStr};

use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    Problem::go()
}

struct Game {
    id: usize,
    handfuls: Vec<Handful>,
}

struct Handful {
    cube_counts: HashMap<Color, usize>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Color {
    Red,
    Green,
    Blue,
}

use Color::*;

impl FromStr for Color {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        match contents {
            "red" | "Red" => Ok(Red),
            "green" | "Green" => Ok(Green),
            "blue" | "Blue" => Ok(Blue),
            c => Err(anyhow!("invalid color {c}")),
        }
    }
}

impl FromStr for Handful {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        Ok(Self {
            cube_counts: contents
                .split(", ")
                .map(|cube_count| -> Result<(Color, usize)> {
                    let (count, color) = parse_pair(cube_count, ' ')?;

                    Ok((color, count))
                })
                .collect::<Result<HashMap<_, _>>>()?,
        })
    }
}

impl FromStr for Game {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        if !contents.starts_with("Game ") {
            bail!("invalid game line");
        }
        let (_, rest) = contents.split_once(' ').ok_or_invalid()?;
        let (id_str, rest) = rest.split_once(": ").ok_or_invalid()?;
        Ok(Self {
            id: id_str.parse_wrapped()?,
            handfuls: parse_split(rest, "; ")?,
        })
    }
}

struct Problem {
    games: Vec<Game>,
}

impl Game {
    fn possible_with_counts(&self, counts: &HashMap<Color, usize>) -> bool {
        for handful in &self.handfuls {
            for (color, count) in &handful.cube_counts {
                if counts.get(color).unwrap_or(&0) < count {
                    return false;
                }
            }
        }
        true
    }

    fn min_power(&self) -> usize {
        let mut mins = HashMap::new();
        for handful in &self.handfuls {
            for (color, count) in &handful.cube_counts {
                match mins.get(color) {
                    None => {
                        mins.insert(color, *count);
                    }
                    Some(&min_so_far) if min_so_far < *count => {
                        mins.insert(color, *count);
                    }
                    _ => (),
                }
            }
        }
        mins.values().product()
    }
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        Ok(Self {
            games: parse_lines(contents)?,
        })
    }
}

impl Solution for Problem {
    type Part1 = usize;
    type Part2 = usize;

    fn part1(&mut self) -> Result<Self::Part1> {
        let counts = HashMap::from([(Red, 12), (Green, 13), (Blue, 14)]);
        Ok(self
            .games
            .iter()
            .filter(|g| g.possible_with_counts(&counts))
            .map(|g| g.id)
            .sum())
    }

    fn part2(&self) -> Result<Self::Part2> {
        Ok(self.games.iter().map(|g| g.min_power()).sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        let mut problem = Problem::from_str(SAMPLE)?;

        let result = problem.part1()?;

        assert_eq!(8, result);

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let problem = Problem::from_str(SAMPLE)?;

        let result = problem.part2()?;

        assert_eq!(2286, result);

        Ok(())
    }

    const SAMPLE: &str = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";
}
