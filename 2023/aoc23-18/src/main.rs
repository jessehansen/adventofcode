use std::str::FromStr;

use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    Problem::go()
}

struct PlanStep {
    direction: Direction,
    distance: i64,
    color: String,
}

impl PlanStep {
    fn extract_step_from_color(&self) -> Result<(Direction, i64)> {
        use Direction::*;
        if self.color.len() != 7 || !self.color.starts_with('#') {
            bail!("invalid color");
        }
        let distance = i64::from_str_radix(&self.color[1..6], 16)?;
        let direction = match &self.color[6..7] {
            "0" => Ok(Right),
            "1" => Ok(Down),
            "2" => Ok(Left),
            "3" => Ok(Up),
            _ => bail!("invalid direction"),
        }?;

        Ok((direction, distance))
    }
}

impl FromStr for PlanStep {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        use Direction::*;
        let (dir, contents) = contents.split_once(' ').ok_or_invalid()?;
        let direction = match dir {
            "R" => Ok(Right),
            "L" => Ok(Left),
            "U" => Ok(Up),
            "D" => Ok(Down),
            _ => bail!("invalid dir"),
        }?;
        let (distance, color) = contents.split_once(' ').ok_or_invalid()?;

        Ok(PlanStep {
            direction,
            distance: distance.parse_wrapped()?,
            color: color
                .trim_start_matches('(')
                .trim_end_matches(')')
                .to_string(),
        })
    }
}

struct Problem {
    plan: Vec<PlanStep>,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        Ok(Self {
            plan: contents.parse_lines()?,
        })
    }
}

fn calculate_area<I>(steps: I) -> i64
where
    I: Iterator<Item = (Direction, i64)>,
{
    let mut vertices = vec![IPoint2D::ORIGIN];
    let mut boundary_length: i64 = 0;
    for (direction, distance) in steps {
        vertices.push(
            vertices
                .last()
                .unwrap()
                .move_to(direction, distance.try_into().unwrap()),
        );

        boundary_length += distance;
    }
    // we calculated the area, but because it was calculated from the center of the borders half of
    // the boundary length is left off of the final result. Then there are also 4 corners that are
    // not accounted for, so add those in
    shoelace_loop_area_64(vertices) + boundary_length / 2 + 1
}

impl Solution for Problem {
    type Part1 = i64;
    type Part2 = i64;

    fn part1(&mut self) -> Result<Self::Part1> {
        Ok(calculate_area(
            self.plan.iter().map(|s| (s.direction, s.distance)),
        ))
    }

    fn part2(&self) -> Result<Self::Part2> {
        Ok(calculate_area(
            self.plan
                .iter()
                .filter_map(|s| s.extract_step_from_color().ok()),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        let mut problem = Problem::from_str(SAMPLE)?;

        let result = problem.part1()?;

        assert_eq!(62, result);

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let problem = Problem::from_str(SAMPLE)?;

        let result = problem.part2()?;

        assert_eq!(952408144115, result);

        Ok(())
    }

    const SAMPLE: &str = "\
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
}
