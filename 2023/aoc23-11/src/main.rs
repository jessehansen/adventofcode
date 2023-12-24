use std::{
    cmp::{max, min},
    collections::HashSet,
    fmt,
    str::FromStr,
};

use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    Problem::go()
}

#[derive(Clone, Copy, Default, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
enum Space {
    #[default]
    Empty,
    Galaxy,
}

use Space::*;

impl FromStr for Space {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        match contents {
            "." => Ok(Empty),
            "#" => Ok(Galaxy),
            _ => bail!("invalid space"),
        }
    }
}

impl fmt::Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Empty => ".",
                Galaxy => "#",
            }
        )
    }
}

struct Problem {
    galaxies: Vec<Point2D>,
    empty_rows: HashSet<usize>,
    empty_cols: HashSet<usize>,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        let map = Grid2D::from_char_str(contents)?;
        let galaxies = map
            .iter_horizontal()
            .filter_map(|(pt, space)| match space {
                Galaxy => Some(pt),
                _ => None,
            })
            .collect();
        let empty_rows = (0..map.bounds.height)
            .filter(|row| map.row(*row).all(|(_, &x)| x == Empty))
            .collect();
        let empty_cols = (0..map.bounds.width)
            .filter(|col| map.col(*col).all(|(_, &x)| x == Empty))
            .collect();
        Ok(Self {
            galaxies,
            empty_rows,
            empty_cols,
        })
    }
}

impl Solution for Problem {
    type Part1 = usize;
    type Part2 = usize;

    fn part1(&mut self) -> Result<Self::Part1> {
        Ok(self.min_distance_sum(2))
    }

    fn part2(&self) -> Result<Self::Part2> {
        Ok(self.min_distance_sum(1_000_000))
    }
}

impl Problem {
    fn min_distance_sum(&self, empty_multiplier: usize) -> usize {
        (0..self.galaxies.len() - 1)
            .map(|i1| {
                ((i1 + 1)..self.galaxies.len())
                    .map(|i2| {
                        self.modified_distance(
                            self.galaxies[i1],
                            self.galaxies[i2],
                            empty_multiplier,
                        )
                    })
                    .sum::<usize>()
            })
            .sum()
    }

    fn modified_distance(&self, pt1: Point2D, pt2: Point2D, empty_multiplier: usize) -> usize {
        let row_min = min(pt1.y, pt2.y);
        let row_max = max(pt1.y, pt2.y);
        let col_min = min(pt1.x, pt2.x);
        let col_max = max(pt1.x, pt2.x);

        (row_min..row_max)
            .map(|row| {
                if self.empty_rows.contains(&row) {
                    empty_multiplier
                } else {
                    1
                }
            })
            .sum::<usize>()
            + (col_min..col_max)
                .map(|col| {
                    if self.empty_cols.contains(&col) {
                        empty_multiplier
                    } else {
                        1
                    }
                })
                .sum::<usize>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        let mut problem = Problem::from_str(SAMPLE)?;

        let result = problem.part1()?;

        assert_eq!(374, result);

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let problem = Problem::from_str(SAMPLE)?;

        let result = problem.part2()?;

        assert_eq!(82000210, result);

        Ok(())
    }

    const SAMPLE: &str = "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
}
