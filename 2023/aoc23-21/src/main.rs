use std::{
    collections::{HashMap, HashSet},
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
    Garden,
    Rock,
    Start,
}

use Space::*;

impl FromStr for Space {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        match contents {
            "." => Ok(Garden),
            "#" => Ok(Rock),
            "S" => Ok(Start),
            _ => bail!("invalid space"),
        }
    }
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

impl Problem {
    fn count_reachable_in_steps(&self, steps: &[usize]) -> Vec<usize> {
        let mut current_locations: HashSet<IPoint2D> = self
            .map
            .iter_horizontal()
            .filter_map(|(pt, &space)| match space {
                Start => IPoint2D::try_from(pt).ok(),
                _ => None,
            })
            .collect();

        let mut cache: HashMap<IPoint2D, Vec<IPoint2D>> = HashMap::new();

        let rocks: HashSet<IPoint2D> = self
            .map
            .iter_horizontal()
            .filter_map(|(pt, space)| {
                if space == &Rock {
                    pt.try_into().ok()
                } else {
                    None
                }
            })
            .collect();

        let mut results = vec![];

        // prime cache - for every location in the grid, figure out where next steps would lead

        for (pt, &space) in self.map.iter_horizontal() {
            if space == Rock {
                continue;
            }
            if let std::result::Result::Ok(pt) = IPoint2D::try_from(pt) {
                cache.insert(
                    pt,
                    pt.cardinal_neighbors()
                        .into_iter()
                        .filter(|pt| !rocks.contains(pt))
                        .collect(),
                );
            }
        }

        for step in 1..=(usize::MAX) {
            let next_locations = current_locations
                .iter()
                .flat_map(|pt| {
                    let (template_pt, copy_distance) =
                        pt.map_infinite_to_template_bounds(&self.map.bounds);
                    cache.entry(template_pt).or_insert_with(|| {
                        //should never happen
                        println!("CACHE MISS");
                        template_pt
                            .cardinal_neighbors()
                            .into_iter()
                            .filter(|&pt| !rocks.contains(&pt))
                            .collect()
                    });
                    cache[&template_pt]
                        .iter()
                        .map(|pt| pt + copy_distance)
                        .collect::<Vec<_>>()
                })
                .collect::<HashSet<_>>();

            current_locations = next_locations;

            if step == steps[results.len()] {
                results.push(current_locations.len());
                if results.len() == steps.len() {
                    return results;
                }
            }
        }
        vec![]
    }

    fn determine_steps_after_periods(
        &self,
        period_start: usize,
        period_length: usize,
        num_periods: usize,
    ) -> usize {
        // I had to look up how to solve this one
        //
        // Hat tip to https://www.reddit.com/r/adventofcode/comments/18nevo3/comment/kecpva6/
        //
        // the number of locations after x periods is some polynomial equation
        //   P(x) = ax^2 + bx + c
        // For the input data, each period is 131 steps. The period start (65) is determined by the
        // remainder of our target step count (26501365) and the period.
        //
        // P(0) = locations after 65 steps
        // P(1) = locations after (65+131) steps
        // P(2) = locations after (65+131+131) steps
        //
        // Then it's just solving for a, b, c
        //
        // P(0) = a(0)^2 + b(0) + c
        //  therefore c = P(0)
        //
        // P(2) = a(2)^2 + b(2) + c = 4a + 2b + c
        // P(1) = a(1)^2 + b(1) + c = a + b + c
        //   this gives us enough to solve for a:
        //   P(2) - 2(P(1)) = (4a - 2a) + (2b - 2b) + (c - 2c)
        //   P(2) - 2(P(1)) = 2a - c
        //   therefore a = (P(2) - 2(P(1)) + c) / 2
        //
        // P(1) = a(1)^2 + b(1) + c,
        //  P(1) = a + b + c,
        //  therefore b = P(1) - a - c

        let counts = self.count_reachable_in_steps(&[
            period_start,
            period_start + period_length,
            period_start + period_length + period_length,
        ]);
        let p_0 = counts[0];
        let p_1 = counts[1];
        let p_2 = counts[2];

        let c = p_0;
        let a = (p_2 - 2 * p_1 + c) / 2;
        let b = p_1 - a - c;

        let x = num_periods;
        a * x * x + b * x + c
    }
}

impl Solution for Problem {
    type Part1 = usize;
    type Part2 = usize;

    fn part1(&mut self) -> Result<Self::Part1> {
        Ok(self.count_reachable_in_steps(&[64])[0])
    }

    fn part2(&self) -> Result<Self::Part2> {
        // The input MUST be quadratic with a period of bounds.width.
        // There may be other restrictions on the input or target step count, but this works
        // with my input
        let target_steps = 26501365;
        Ok(self.determine_steps_after_periods(
            target_steps % self.map.bounds.width,
            self.map.bounds.width,
            target_steps / self.map.bounds.width,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_six_steps() -> Result<()> {
        let problem = Problem::from_str(SAMPLE)?;

        assert_eq!(16, problem.count_reachable_in_steps(&[6])[0]);

        Ok(())
    }

    #[test]
    fn sample_more_steps() -> Result<()> {
        let problem = Problem::from_str(SAMPLE)?;

        // steps > 10 requires infinite grid step counting to work
        let counts = problem.count_reachable_in_steps(&[6, 10, 50, 100 /*, 500, 1000, 5000*/]);

        assert_eq!(16, counts[0]);
        assert_eq!(50, counts[1]);
        assert_eq!(1594, counts[2]);
        assert_eq!(6536, counts[3]);
        // assert_eq!(167004, counts[4]);
        // assert_eq!(668697, counts[5]);
        // assert_eq!(16733044, counts[6]);

        Ok(())
    }

    const SAMPLE: &str = "\
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
}
