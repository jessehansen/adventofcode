use std::{cmp::min, fmt, str::FromStr};

use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    Problem::go()
}

#[derive(Clone, Copy, Default, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
enum Space {
    #[default]
    Ash,
    Rocks,
}

impl FromStr for Space {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        match contents {
            "." => Ok(Ash),
            "#" => Ok(Rocks),
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
                Ash => ".",
                Rocks => "#",
            }
        )
    }
}

impl Space {
    fn opposite(&self) -> Space {
        match self {
            Ash => Rocks,
            Rocks => Ash,
        }
    }
}

use Space::*;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
enum AxisOfReflection {
    Vertical(usize),
    Horizontal(usize),
}

use AxisOfReflection::*;

impl AxisOfReflection {
    fn summarize(&self) -> usize {
        match self {
            Vertical(col) => *col,
            Horizontal(row) => row * 100,
        }
    }
}

fn find_axes_of_reflection(
    grid: &Grid2D<Space>,
    smudge_at: Option<Point2D>,
) -> Vec<AxisOfReflection> {
    let mut axes = vec![];
    let get = |pt: Point2D| -> Space {
        let val = grid[pt];
        if smudge_at == Some(pt) {
            val.opposite()
        } else {
            val
        }
    };

    for boundary_col in 1..grid.bounds.width {
        if (0..grid.bounds.height).all(|row| {
            let distance_to_check = min(boundary_col, grid.bounds.width - boundary_col);

            (0..distance_to_check).all(|distance_from_boundary| {
                get(pt(boundary_col - (distance_from_boundary + 1), row))
                    == get(pt(boundary_col + distance_from_boundary, row))
            })
        }) {
            axes.push(Vertical(boundary_col));
        }
    }
    for boundary_row in 1..grid.bounds.height {
        if (0..grid.bounds.width).all(|col| {
            let distance_to_check = min(boundary_row, grid.bounds.height - boundary_row);
            (0..distance_to_check)
                .all(|i| get(pt(col, boundary_row - (i + 1))) == get(pt(col, boundary_row + i)))
        }) {
            axes.push(Horizontal(boundary_row));
        }
    }
    axes
}

struct Problem {
    grids: Vec<Grid2D<Space>>,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        Ok(Self {
            grids: contents
                .split("\n\n")
                .map(Grid2D::<Space>::from_char_str)
                .collect::<Result<Vec<_>>>()?,
        })
    }
}

impl Solution for Problem {
    type Part1 = usize;
    type Part2 = usize;

    fn part1(&mut self) -> Result<Self::Part1> {
        Ok(self
            .grids
            .iter()
            .map(|g| {
                find_axes_of_reflection(g, None)
                    .first()
                    .unwrap()
                    .summarize()
            })
            .sum())
    }

    fn part2(&self) -> Result<Self::Part2> {
        Ok(self
            .grids
            .iter()
            .map(|g| {
                let original_reflections = find_axes_of_reflection(g, None);
                g.bounds
                    .iter_horizontal()
                    .find_map(|pt| {
                        let axes = find_axes_of_reflection(g, Some(pt));
                        for aor in &axes {
                            if !original_reflections.contains(aor) {
                                return Some(*aor);
                            }
                        }
                        None
                    })
                    .unwrap()
                    .summarize()
            })
            .sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        let mut problem = Problem::from_str(SAMPLE)?;

        let result = problem.part1()?;

        assert_eq!(405, result);

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let problem = Problem::from_str(SAMPLE)?;

        let result = problem.part2()?;

        assert_eq!(400, result);

        Ok(())
    }

    const SAMPLE: &str = "\
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
}
