use std::str::FromStr;

use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    Problem::go()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Light {
    Off,
    On,
}

use Light::*;

impl FromStr for Light {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        match contents {
            "." => Ok(Off),
            "#" => Ok(On),
            _ => bail!("invalid space"),
        }
    }
}
struct Problem {
    lights: Grid2D<Light>,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        Ok(Self {
            lights: Grid2D::from_char_str(contents)?,
        })
    }
}

impl Problem {
    fn run(&self, steps: usize, stuck_on: &[Point2D]) -> Grid2D<Light> {
        let mut lights = self.lights.clone();
        for stuck in stuck_on {
            lights[*stuck] = On;
        }
        for _ in 0..steps {
            lights = lights.map(|(pt, l)| {
                if stuck_on.contains(&pt) {
                    On
                } else {
                    let neighbors_on = lights.neighbors(pt).filter(|(_, l)| l == &&On).count();
                    match (l, neighbors_on) {
                        (Off, 3) => On,
                        (On, 2..=3) => On,
                        _ => Off,
                    }
                }
            });
        }
        lights
    }
}

impl Solution for Problem {
    type Part1 = usize;
    type Part2 = usize;

    fn part1(&mut self) -> Result<Self::Part1> {
        Ok(self
            .run(100, &[])
            .iter_horizontal()
            .filter(|(_, l)| l == &&On)
            .count())
    }

    fn part2(&self) -> Result<Self::Part2> {
        let four_corners = self.lights.bounds.corners();
        Ok(self
            .run(100, &four_corners)
            .iter_horizontal()
            .filter(|(_, l)| l == &&On)
            .count())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        let problem = Problem::from_str(SAMPLE)?;

        let result = problem.run(4, &[]);

        assert_eq!(
            4,
            result.iter_horizontal().filter(|(_, l)| l == &&On).count()
        );

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let problem = Problem::from_str(SAMPLE)?;

        let result = problem.run(5, &problem.lights.bounds.corners());

        assert_eq!(
            17,
            result.iter_horizontal().filter(|(_, l)| l == &&On).count()
        );

        Ok(())
    }

    const SAMPLE: &str = "\
.#.#.#
...##.
#....#
..#...
#.#..#
####..";
}
