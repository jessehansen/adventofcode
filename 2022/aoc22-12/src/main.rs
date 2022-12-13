use std::collections::{HashMap, VecDeque};
use std::str::FromStr;

use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    Problem::go()
}

struct Problem {
    height_map: Grid2D<Elevation>,
    start: Point2D,
    end: Point2D,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Problem> {
        let height_map: Grid2D<Elevation> = Grid2D::from_char_str(contents)?;
        let start = height_map
            .iter_horizontal()
            .find(|&(_, elev)| *elev == Elevation::Start)
            .ok_or(anyhow!("no start"))?
            .0;
        let end = height_map
            .iter_horizontal()
            .find(|&(_, elev)| *elev == Elevation::End)
            .ok_or(anyhow!("no end"))?
            .0;
        Ok(Problem {
            height_map,
            start,
            end,
        })
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Elevation {
    Start,
    End,
    Height(u32),
}

impl FromStr for Elevation {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(match s {
            "S" => Elevation::Start,
            "E" => Elevation::End,
            c if c.len() == 1 => Elevation::Height(c.chars().next().unwrap() as u32 - 'a' as u32),
            _ => bail!("invalid elevation"),
        })
    }
}

impl Default for Elevation {
    fn default() -> Self {
        Elevation::Start
    }
}

impl Elevation {
    fn can_step_to(&self, other: &Elevation) -> bool {
        match (self, other) {
            (Elevation::Start, _) => true,
            (Elevation::End, _) => false,
            (_, Elevation::Start) => false,
            (Elevation::Height(s), Elevation::End) => ('z' as u32 - 'a' as u32) <= s + 1, // End has same elevation as 'z'
            (Elevation::Height(s), Elevation::Height(o)) => *o <= s + 1,
        }
    }
}

impl Problem {
    fn shortest_path_from(&self, start: Point2D) -> usize {
        let mut q = VecDeque::from([start]);
        let mut distances = Grid2D::new_constant(self.height_map.bounds, usize::MAX);
        distances[start] = 0;
        while !q.is_empty() {
            let current = q.pop_front().unwrap(); // safe to unwrap since q cannot be empty
            let next_step_distance = distances[current] + 1;
            for (pt, elev) in self.height_map.cardinal_neighbors(current) {
                if self.height_map[current].can_step_to(elev) && next_step_distance < distances[pt]
                {
                    q.push_back(pt);
                    distances[pt] = next_step_distance;
                }
            }
        }

        distances[self.end]
    }
}

impl Solution for Problem {
    type Part1 = usize;
    type Part2 = usize;

    fn part1(&mut self) -> Result<Self::Part1> {
        Ok(self.shortest_path_from(self.start))
    }

    fn part2(&self) -> Result<Self::Part2> {
        self.height_map
            .iter_horizontal()
            .filter(|&(_pt, elev)| *elev == Elevation::Height(0) || *elev == Elevation::Start)
            .map(|(start, _)| self.shortest_path_from(start))
            .min()
            .ok_or(anyhow!("no min"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        let mut problem = Problem::from_str(SAMPLE)?;

        let result = problem.part1()?;

        assert_eq!(31, result);

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let problem = Problem::from_str(SAMPLE)?;

        let result = problem.part2()?;

        assert_eq!(29, result);

        Ok(())
    }

    const SAMPLE: &str = "\
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";
}
