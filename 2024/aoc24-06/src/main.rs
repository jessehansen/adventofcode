use std::{collections::HashSet, str::FromStr};

use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    Problem::go()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Space {
    Empty,
    Obstacle,
    Guard(Direction),
}

use Direction::*;
use Space::*;

impl FromStr for Space {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "." => Ok(Empty),
            "#" => Ok(Obstacle),
            "^" => Ok(Guard(Up)),
            "<" => Ok(Guard(Left)),
            ">" => Ok(Guard(Right)),
            "v" => Ok(Guard(Down)),
            _ => Err(anyhow!("invalid space")),
        }
    }
}

struct Problem {
    map: Grid2D<Space>,
    guard: Guard,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Guard {
    location: Point2D,
    direction: Direction,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        let mut map = Grid2D::from_char_str(contents)?;
        let guard = map
            .iter_horizontal()
            .filter_map(|(pt, space)| match space {
                Guard(dir) => Some(Guard {
                    location: pt,
                    direction: *dir,
                }),
                _ => None,
            })
            .next()
            .ok_or_invalid()?;
        map[guard.location] = Empty;

        Ok(Self { map, guard })
    }
}

impl Problem {
    fn step(&self, guard: Guard, extra_obstacle_location: Option<Point2D>) -> Option<Guard> {
        match guard
            .location
            .mv(guard.direction, self.map.bounds)
            .map(|pt| (pt, self.map[pt]))
        {
            Some((pt, space))
                if space == Empty && extra_obstacle_location == Some(pt) || space == Obstacle =>
            {
                self.step(
                    Guard {
                        location: guard.location,
                        direction: guard.direction.clockwise90(),
                    },
                    extra_obstacle_location,
                )
            }
            Some((pt, Empty)) => Some(Guard {
                location: pt,
                direction: guard.direction,
            }),
            _ => None,
        }
    }

    fn determine_guard_route(&self) -> Result<HashSet<Point2D>> {
        let mut visited = HashSet::new();
        let mut guard = Some(self.guard);

        while guard.is_some() {
            let g = guard.unwrap();
            visited.insert(g.location);
            guard = self.step(g, None);
        }

        Ok(visited)
    }

    fn is_loop_with_obstacle_at(&self, location: &Point2D) -> bool {
        let mut guard_steps = HashSet::new();
        let mut guard = Some(self.guard);

        while guard.is_some() {
            let g = guard.unwrap();
            if !guard_steps.insert(g) {
                // if we're in the same place, facing the same direction,
                // we have hit a loop
                return true;
            }
            guard = self.step(g, Some(*location));
        }

        false
    }
}

impl Solution for Problem {
    type Part1 = usize;
    type Part2 = usize;

    fn part1(&mut self) -> Result<Self::Part1> {
        let visited = self.determine_guard_route()?;
        Ok(visited.len())
    }

    fn part2(&self) -> Result<Self::Part2> {
        let possible_loop_obstacles = self.determine_guard_route()?;

        Ok(possible_loop_obstacles
            .iter()
            .filter(|location| **location != self.guard.location)
            .filter(|location| self.is_loop_with_obstacle_at(location))
            .count())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        let mut problem = Problem::from_str(SAMPLE)?;

        let result = problem.part1()?;

        assert_eq!(41, result);

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let problem = Problem::from_str(SAMPLE)?;

        let result = problem.part2()?;

        assert_eq!(6, result);

        Ok(())
    }

    const SAMPLE: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";
}
