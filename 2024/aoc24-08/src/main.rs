use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    Problem::go()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Space {
    Empty,
    Antenna(char),
}
use Space::*;

struct Problem {
    map: Grid2D<Space>,
    by_frequency: HashMap<char, Vec<Point2D>>,
}

impl FromStr for Space {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.chars().next() {
            Some('.') => Ok(Empty),
            Some(c) if matches!(c, 'A'..='Z' | 'a'..='z' | '0'..='9') => Ok(Antenna(c)),
            _ => bail!("invalid space"),
        }
    }
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        let map = Grid2D::from_char_str(contents)?;
        let by_frequency = map
            .iter_horizontal()
            .filter_map(|(pt, space)| match space {
                Empty => None,
                Antenna(c) => Some((pt, *c)),
            })
            .fold(
                HashMap::new(),
                |mut acc: HashMap<char, Vec<Point2D>>, (pt, frequency)| {
                    acc.entry(frequency).or_default().push(pt);
                    acc
                },
            );
        Ok(Self { map, by_frequency })
    }
}

struct AntinodeWalker {
    current: IPoint2D,
    bounds: Bounds2D,
    dx: i32,
    dy: i32,
}

impl Iterator for AntinodeWalker {
    type Item = Point2D;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self
            .current
            .try_into()
            .ok()
            .filter(|&p| self.bounds.contains(&p));

        self.current = ipt(self.current.x + self.dx, self.current.y + self.dy);

        result
    }
}

impl Problem {
    fn determine_antinodes_flawed(&self, a: &Point2D, b: &Point2D) -> Vec<Point2D> {
        let ia: IPoint2D = a.try_into().unwrap();
        let ib: IPoint2D = b.try_into().unwrap();
        let dx = ib.x - ia.x;
        let dy = ib.y - ia.y;

        let ia1 = ipt(ia.x - dx, ia.y - dy);
        let ia2 = ipt(ib.x + dx, ib.y + dy);

        [ia1, ia2]
            .into_iter()
            .filter_map(|antinode| {
                antinode
                    .try_into()
                    .ok()
                    .filter(|&p| self.map.bounds.contains(&p))
            })
            .collect()
    }

    fn determine_antinodes(&self, a: &Point2D, b: &Point2D) -> Vec<Point2D> {
        let ia: IPoint2D = a.try_into().unwrap();
        let ib: IPoint2D = b.try_into().unwrap();
        let dx = ib.x - ia.x;
        let dy = ib.y - ia.y;

        AntinodeWalker {
            current: ia,
            bounds: self.map.bounds,
            dx: -dx,
            dy: -dy,
        }
        .chain(AntinodeWalker {
            current: ib,
            bounds: self.map.bounds,
            dx,
            dy,
        })
        .collect()
    }
}

impl Solution for Problem {
    type Part1 = usize;
    type Part2 = usize;

    fn part1(&mut self) -> Result<Self::Part1> {
        let antinodes: HashSet<Point2D> = self
            .by_frequency
            .iter()
            .flat_map(|(_frequency, antennae)| {
                antennae.iter().combinations(2).flat_map(|pair| {
                    self.determine_antinodes_flawed(pair[0], pair[1])
                        .into_iter()
                })
            })
            .collect();

        Ok(antinodes.len())
    }

    fn part2(&self) -> Result<Self::Part2> {
        let antinodes: HashSet<Point2D> = self
            .by_frequency
            .iter()
            .flat_map(|(_frequency, antennae)| {
                antennae
                    .iter()
                    .combinations(2)
                    .flat_map(|pair| self.determine_antinodes(pair[0], pair[1]).into_iter())
            })
            .collect();

        Ok(antinodes.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        let mut problem = Problem::from_str(SAMPLE)?;

        let result = problem.part1()?;

        assert_eq!(14, result);

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let problem = Problem::from_str(SAMPLE)?;

        let result = problem.part2()?;

        assert_eq!(34, result);

        Ok(())
    }

    #[test]
    fn sample2_part2() -> Result<()> {
        let problem = Problem::from_str(SAMPLE_2)?;

        let result = problem.part2()?;

        assert_eq!(9, result);

        Ok(())
    }

    const SAMPLE: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

    const SAMPLE_2: &str = "\
T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
..........
";
}
