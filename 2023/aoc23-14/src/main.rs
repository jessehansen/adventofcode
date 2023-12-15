use std::{collections::HashMap, fmt, str::FromStr};

use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    Problem::go()
}

#[derive(Clone, Copy, Default, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
enum Space {
    #[default]
    Empty,
    RoundRock,
    CubeRock,
}

use Space::*;

impl FromStr for Space {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        match contents {
            "." => Ok(Empty),
            "O" => Ok(RoundRock),
            "#" => Ok(CubeRock),
            _ => bail!("invalid space"),
        }
    }
}

impl fmt::Display for Space {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Empty => ".",
                RoundRock => "O",
                CubeRock => "#",
            }
        )
    }
}

fn tilt_up(before: Grid2D<Space>) -> Grid2D<Space> {
    let mut grid = before.clone();

    let rolling_rocks: Vec<Point2D> = grid
        .iter_horizontal()
        .filter_map(|(pt, space)| match space {
            RoundRock => Some(pt),
            _ => None,
        })
        .collect();

    for rock in rolling_rocks {
        let mut destination = rock;
        loop {
            let next = destination.up();
            if next.is_some() && matches!(grid[next.unwrap()], Empty) {
                destination = next.unwrap();
            } else {
                break;
            }
        }

        if rock != destination {
            grid[destination] = RoundRock;
            grid[rock] = Empty;
        }
    }

    grid
}

fn cycle(grid: Grid2D<Space>) -> Grid2D<Space> {
    // north is up
    let grid = tilt_up(grid);
    // rotate first so west is up
    let grid = tilt_up(grid.rotate90());
    // rotate again so south is up
    let grid = tilt_up(grid.rotate90());
    // rotate again so east is up
    let grid = tilt_up(grid.rotate90());

    // rotate back so north is up
    grid.rotate90()
}

fn calc_load(grid: &Grid2D<Space>) -> usize {
    grid.iter_horizontal()
        .filter_map(|(pt, space)| match space {
            RoundRock => Some(grid.bounds.height - pt.y),
            _ => None,
        })
        .sum()
}

struct Problem {
    map: Grid2D<Space>,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        Ok(Self {
            map: Grid2D::from_char_str(contents)?,
        })
    }
}

impl Solution for Problem {
    type Part1 = usize;
    type Part2 = usize;

    fn part1(&mut self) -> Result<Self::Part1> {
        let map = tilt_up(self.map.clone());
        Ok(calc_load(&map))
    }

    fn part2(&self) -> Result<Self::Part2> {
        let mut load_cache = vec![];
        let mut visited = HashMap::new();
        let mut map = self.map.clone();

        let repeated_at = loop {
            map = cycle(map);
            let cache_key = format!("{map}");

            if let Some(&index) = visited.get(&cache_key) {
                break index;
            } else {
                visited.insert(cache_key, load_cache.len());
                load_cache.push(calc_load(&map));
            }
        };

        let index_for_billionth =
            (1_000_000_000 - load_cache.len() - 1) % (load_cache.len() - repeated_at) + repeated_at;
        Ok(load_cache[index_for_billionth])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        let mut problem = Problem::from_str(SAMPLE)?;

        let result = problem.part1()?;

        assert_eq!(136, result);

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let problem = Problem::from_str(SAMPLE)?;

        let result = problem.part2()?;

        assert_eq!(64, result);

        Ok(())
    }

    const SAMPLE: &str = "\
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
}
