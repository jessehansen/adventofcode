use std::str::FromStr;

use anyhow::*;
use aoc_common::*;

use Direction::*;

fn main() -> Result<()> {
    Problem::go()
}

struct Problem {
    forest: Grid2D<u32>,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        Ok(Problem {
            forest: Grid2D::from_char_str(contents)?,
        })
    }
}

impl Problem {
    fn is_visible_from(&self, pt: &Point2D, direction: Direction) -> bool {
        let height = self.forest[*pt];
        let mut location = pt.cardinal_neighbor(direction, self.forest.bounds);

        while let Some(loc) = location {
            if self.forest[loc] >= height {
                return false;
            }
            location = loc.cardinal_neighbor(direction, self.forest.bounds);
        }
        true
    }

    fn is_visible(&self, pt: &Point2D) -> bool {
        self.is_visible_from(pt, Left)
            || self.is_visible_from(pt, Right)
            || self.is_visible_from(pt, Up)
            || self.is_visible_from(pt, Down)
    }

    fn viewing_distance(&self, pt: &Point2D, direction: Direction) -> u32 {
        let height = self.forest[*pt];
        let mut location = pt.cardinal_neighbor(direction, self.forest.bounds);
        let mut distance = 0;

        while let Some(loc) = location {
            distance += 1;
            if self.forest[loc] >= height {
                break;
            }
            location = loc.cardinal_neighbor(direction, self.forest.bounds);
        }
        distance
    }

    fn viewing_score(&self, pt: &Point2D) -> u32 {
        self.viewing_distance(pt, Left)
            * self.viewing_distance(pt, Right)
            * self.viewing_distance(pt, Up)
            * self.viewing_distance(pt, Down)
    }
}

impl Solution for Problem {
    type Part1 = usize;
    type Part2 = u32;

    fn part1(&mut self) -> Result<usize> {
        Ok(self
            .forest
            .bounds
            .iter_horizontal()
            .filter(|pt| self.is_visible(pt))
            .count())
    }

    fn part2(&self) -> Result<u32> {
        self.forest
            .bounds
            .iter_horizontal()
            .map(|pt| self.viewing_score(&pt))
            .max()
            .ok_or_else(|| anyhow!("invalid forest"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        let mut problem = Problem::from_str(SAMPLE)?;

        let result = problem.part1()?;

        assert_eq!(21, result);

        Ok(())
    }

    #[test]
    fn test_viewing_distance() -> Result<()> {
        let problem = Problem::from_str(SAMPLE)?;

        assert_eq!(1, problem.viewing_distance(&pt(2, 1), Left));
        assert_eq!(2, problem.viewing_distance(&pt(2, 1), Right));
        assert_eq!(1, problem.viewing_distance(&pt(2, 1), Up));
        assert_eq!(2, problem.viewing_distance(&pt(2, 1), Down));

        Ok(())
    }

    #[test]
    fn test_viewing_score() -> Result<()> {
        let problem = Problem::from_str(SAMPLE)?;

        assert_eq!(4, problem.viewing_score(&pt(2, 1)));
        assert_eq!(8, problem.viewing_score(&pt(2, 3)));

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let problem = Problem::from_str(SAMPLE)?;

        let result = problem.part2()?;

        assert_eq!(8, result);

        Ok(())
    }

    const SAMPLE: &str = "\
30373
25512
65332
33549
35390
";
}
