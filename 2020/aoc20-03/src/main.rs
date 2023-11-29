use std::str::FromStr;

use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    Problem::go()
}

#[derive(Copy, Clone, Default, Debug, PartialEq)]
enum Square {
    #[default]
    Open,
    Tree,
}

use Square::*;

impl FromStr for Square {
    type Err = Error;

    fn from_str(sq: &str) -> Result<Self> {
        match sq {
            "." => Ok(Open),
            "#" => Ok(Tree),
            _ => bail!("invalid square"),
        }
    }
}

struct Problem {
    map: Grid2D<Square>,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Problem> {
        Ok(Problem {
            map: Grid2D::from_char_str(contents)?,
        })
    }
}

impl Solution for Problem {
    type Part1 = usize;
    type Part2 = usize;

    fn part1(&mut self) -> Result<Self::Part1> {
        Ok(self.count_trees(3, 1))
    }

    fn part2(&self) -> Result<Self::Part2> {
        Ok(self.count_trees(1, 1)
            * self.count_trees(3, 1)
            * self.count_trees(5, 1)
            * self.count_trees(7, 1)
            * self.count_trees(1, 2))
    }
}

impl Problem {
    fn count_trees(&self, slope_x: usize, slope_y: usize) -> usize {
        let mut location = Some(Point2D::ORIGIN);
        let mut tree_count = 0;
        while let Some(loc) = location {
            if self.map[loc] == Tree {
                tree_count += 1
            }
            location = move_slope(&loc, slope_x, slope_y, &self.map.bounds);
        }
        tree_count
    }
}

fn move_slope(
    start: &Point2D,
    slope_x: usize,
    slope_y: usize,
    bounds: &Bounds2D,
) -> Option<Point2D> {
    let next = pt(start.x + slope_x, start.y + slope_y);
    if next.y > bounds.height - 1 {
        None
    } else if next.x > bounds.width - 1 {
        Some(pt(next.x - bounds.width, next.y))
    } else {
        Some(next)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        let mut problem = Problem::from_str(SAMPLE)?;

        let result = problem.part1()?;

        assert_eq!(7, result);

        Ok(())
    }

    #[test]
    fn count_trees() -> Result<()> {
        let problem = Problem::from_str(SAMPLE)?;

        assert_eq!(2, problem.count_trees(1, 1));
        assert_eq!(7, problem.count_trees(3, 1));
        assert_eq!(3, problem.count_trees(5, 1));
        assert_eq!(4, problem.count_trees(7, 1));
        assert_eq!(2, problem.count_trees(1, 2));

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let problem = Problem::from_str(SAMPLE)?;

        let result = problem.part2()?;

        assert_eq!(336, result);

        Ok(())
    }

    const SAMPLE: &str = "\
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
";
}
