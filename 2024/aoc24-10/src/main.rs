use std::{collections::HashSet, str::FromStr};

use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    Problem::go()
}

struct Problem {
    map: Grid2D<u32>,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        Ok(Self {
            map: Grid2D::from_char_str(contents)?,
        })
    }
}

impl Problem {
    fn trailheads(&self) -> Vec<Point2D> {
        self.map
            .iter_horizontal()
            .filter_map(|(pt, height)| match height {
                0 => Some(pt),
                _ => None,
            })
            .collect()
    }

    fn reachable_summits(&self, trailhead: Point2D) -> HashSet<Point2D> {
        let mut todo = vec![trailhead];
        let mut reachable = HashSet::new();

        while let Some(current) = todo.pop() {
            for pt in current.cardinal_neighbors(self.map.bounds) {
                let height = self.map[pt];
                if height == self.map[current] + 1 {
                    if height == 9 {
                        reachable.insert(pt);
                    } else {
                        todo.push(pt);
                    }
                }
            }
        }

        reachable
    }

    fn rating(&self, trailhead: Point2D) -> usize {
        let mut todo = vec![vec![trailhead]];
        let mut paths = HashSet::new();

        while let Some(current) = todo.pop() {
            if let Some(pos) = current.last() {
                for pt in pos.cardinal_neighbors(self.map.bounds) {
                    let height = self.map[pt];
                    let mut next_step = current.clone();
                    next_step.push(pt);
                    if height == self.map[pos] + 1 {
                        if height == 9 {
                            paths.insert(next_step);
                        } else {
                            todo.push(next_step);
                        }
                    }
                }
            }
        }

        paths.len()
    }
}

impl Solution for Problem {
    type Part1 = usize;
    type Part2 = usize;

    fn part1(&mut self) -> Result<Self::Part1> {
        Ok(self
            .trailheads()
            .into_iter()
            .map(|pt| self.reachable_summits(pt).len())
            .sum())
    }

    fn part2(&self) -> Result<Self::Part2> {
        Ok(self
            .trailheads()
            .into_iter()
            .map(|pt| self.rating(pt))
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

        assert_eq!(1, result);

        Ok(())
    }

    #[test]
    fn sample2_part1() -> Result<()> {
        let mut problem = Problem::from_str(SAMPLE_2)?;

        let result = problem.part1()?;

        assert_eq!(36, result);

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let problem = Problem::from_str(SAMPLE_3)?;

        let result = problem.part2()?;

        assert_eq!(81, result);

        Ok(())
    }

    const SAMPLE: &str = "\
0123
1234
8765
9876
";

    const SAMPLE_2: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

    const SAMPLE_3: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";
}
