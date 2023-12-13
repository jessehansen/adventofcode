use std::{collections::HashSet, str::FromStr};

use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    Problem::go()
}

#[derive(Clone, Copy, Default, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
enum Space {
    #[default]
    Ground,
    Start,
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
}

use Space::*;

impl FromStr for Space {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        match contents {
            "." => Ok(Ground),
            "S" => Ok(Start),
            "|" => Ok(Vertical),
            "-" => Ok(Horizontal),
            "L" => Ok(NorthEast),
            "J" => Ok(NorthWest),
            "7" => Ok(SouthWest),
            "F" => Ok(SouthEast),
            _ => bail!("invalid space"),
        }
    }
}

impl Space {
    fn is_corner(&self) -> bool {
        matches!(self, Start | NorthEast | NorthWest | SouthEast | SouthWest)
    }
}

struct Problem {
    map: Grid2D<Space>,
    start: Point2D,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        Ok(Self {
            map: Grid2D::from_char_str(contents)?,
            start: Point2D::ORIGIN,
        })
    }
}

impl Solution for Problem {
    type Part1 = usize;
    type Part2 = usize;

    fn part1(&mut self) -> Result<Self::Part1> {
        let (start, _) = self
            .map
            .iter_horizontal()
            .find(|(_, &space)| space == Start)
            .ok_or_invalid()?;
        self.start = start;

        let mut locations: Vec<(Point2D, Point2D)> = self
            .map
            .cardinal_neighbors(start)
            .filter_map(|(loc, _)| self.next(&start, &loc).map(|next| (loc, next)))
            .collect();

        let mut visited = HashSet::from([start]);
        let mut step = 1;

        while !locations.iter().any(|(_, cur)| visited.contains(cur)) {
            for (prev, cur) in locations.iter_mut() {
                visited.insert(*cur);
                if let Some(next) = self.next(prev, cur) {
                    *prev = *cur;
                    *cur = next;
                }
            }
            step += 1;
        }

        Ok(step)
    }

    fn part2(&self) -> Result<Self::Part2> {
        let start = self.start;
        let mut corners = vec![start];
        let mut prev = start;
        let mut cur = self
            .map
            .cardinal_neighbors(start)
            .find_map(|(loc, _)| self.next(&start, &loc).map(|_| loc))
            .ok_or_invalid()?;

        // we have to go through the loop in one direction to get the corners in order, might as
        // well keep track of the length again
        let mut boundary_length = 1;
        while cur != start {
            if self.map[cur].is_corner() {
                corners.push(cur);
            }
            if let Some(next) = self.next(&prev, &cur) {
                prev = cur;
                cur = next;
                boundary_length += 1;
            } else {
                panic!("no next step");
            }
        }

        let loop_area = self.loop_area(corners);
        // derived from pick's formula
        Ok(loop_area - boundary_length / 2 + 1)
    }
}

impl Problem {
    fn next(&self, prev: &Point2D, cur: &Point2D) -> Option<Point2D> {
        let bounds = &self.map.bounds;
        let north = cur.up();
        let south = cur.down(bounds.height);
        let west = cur.left();
        let east = cur.right(bounds.width);
        match self.map[*cur] {
            Vertical if Some(*prev) == north => south,
            Vertical if Some(*prev) == south => north,
            Horizontal if Some(*prev) == west => east,
            Horizontal if Some(*prev) == east => west,
            NorthEast if Some(*prev) == north => east,
            NorthEast if Some(*prev) == east => north,
            NorthWest if Some(*prev) == north => west,
            NorthWest if Some(*prev) == west => north,
            SouthWest if Some(*prev) == south => west,
            SouthWest if Some(*prev) == west => south,
            SouthEast if Some(*prev) == south => east,
            SouthEast if Some(*prev) == east => south,
            _ => None,
        }
    }

    // shoelace formula
    fn loop_area(&self, corners: Vec<Point2D>) -> usize {
        let mut area: i32 = 0;
        let len = corners.len();
        // there's probably a smarter windowing function I could do here, but I'm tired
        for i in 0..len {
            let j = (i + 1) % len;
            area += i32_from(corners[i].x) * i32_from(corners[j].y)
                - i32_from(corners[j].x) * i32_from(corners[i].y);
        }

        usize_from(area.abs()) / 2
    }
}

fn i32_from(other: usize) -> i32 {
    i32::try_from(other).unwrap()
}

fn usize_from(other: i32) -> usize {
    usize::try_from(other).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        let mut problem = Problem::from_str(SAMPLE)?;

        let result = problem.part1()?;

        assert_eq!(4, result);

        Ok(())
    }

    #[test]
    fn sample2_part1() -> Result<()> {
        let mut problem = Problem::from_str(SAMPLE2)?;

        let result = problem.part1()?;

        assert_eq!(8, result);

        Ok(())
    }

    #[test]
    fn sample3_part2() -> Result<()> {
        let mut problem = Problem::from_str(SAMPLE3)?;

        let _ = problem.part1();
        let result = problem.part2()?;

        assert_eq!(4, result);

        Ok(())
    }

    #[test]
    fn sample4_part2() -> Result<()> {
        let mut problem = Problem::from_str(SAMPLE4)?;

        let _ = problem.part1();
        let result = problem.part2()?;

        assert_eq!(4, result);

        Ok(())
    }

    #[test]
    fn sample5_part2() -> Result<()> {
        let mut problem = Problem::from_str(SAMPLE5)?;

        let _ = problem.part1();
        let result = problem.part2()?;

        assert_eq!(8, result);

        Ok(())
    }

    #[test]
    fn sample6_part2() -> Result<()> {
        let mut problem = Problem::from_str(SAMPLE6)?;

        let _ = problem.part1();
        let result = problem.part2()?;

        assert_eq!(10, result);

        Ok(())
    }

    const SAMPLE: &str = "\
.....
.S-7.
.|.|.
.L-J.
.....";

    const SAMPLE2: &str = "\
..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

    const SAMPLE3: &str = "\
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

    const SAMPLE4: &str = "\
..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";

    const SAMPLE5: &str = "\
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

    const SAMPLE6: &str = "\
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
}
