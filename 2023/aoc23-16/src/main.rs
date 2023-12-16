use std::{collections::HashSet, fmt, str::FromStr};

use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    Problem::go()
}

#[derive(Clone, Copy, Default, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
enum Space {
    #[default]
    Empty,
    MirrorRight,
    MirrorLeft,
    SplitterVertical,
    SplitterHorizontal,
}

use Space::*;

impl FromStr for Space {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        match contents {
            "." => Ok(Empty),
            "/" => Ok(MirrorRight),
            "\\" => Ok(MirrorLeft),
            "|" => Ok(SplitterVertical),
            "-" => Ok(SplitterHorizontal),
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
                Empty => ".",
                MirrorRight => "/",
                MirrorLeft => "\\",
                SplitterVertical => "|",
                SplitterHorizontal => "-",
            }
        )
    }
}

impl Space {
    fn next_directions(&self, direction: Direction) -> Vec<Direction> {
        use Direction::*;
        match (self, direction) {
            (Empty, _) => vec![direction],
            (MirrorLeft, Up) => vec![Left],
            (MirrorLeft, Left) => vec![Up],
            (MirrorLeft, Right) => vec![Down],
            (MirrorLeft, Down) => vec![Right],
            (MirrorRight, Up) => vec![Right],
            (MirrorRight, Left) => vec![Down],
            (MirrorRight, Right) => vec![Up],
            (MirrorRight, Down) => vec![Left],
            (SplitterVertical, Up | Down) => vec![direction],
            (SplitterVertical, Left | Right) => vec![Up, Down],
            (SplitterHorizontal, Left | Right) => vec![direction],
            (SplitterHorizontal, Up | Down) => vec![Left, Right],
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct BeamInstant {
    location: Point2D,
    direction: Direction,
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
    fn track_beam(&self, beam: &BeamInstant) -> Vec<BeamInstant> {
        let space = self.map[beam.location];
        let bounds = self.map.bounds;
        space
            .next_directions(beam.direction)
            .iter()
            .filter_map(|&direction| {
                beam.location
                    .mv(direction, bounds)
                    .map(move |location| BeamInstant {
                        location,
                        direction,
                    })
            })
            .collect()
    }

    fn determine_energization(&self, starting_beam: BeamInstant) -> usize {
        let mut beams = vec![starting_beam];
        let mut encountered_states = HashSet::new();
        encountered_states.insert(beams[0].clone());
        while !beams.is_empty() {
            beams = beams
                .iter()
                .flat_map(|beam| self.track_beam(beam))
                .filter(|beam| encountered_states.insert(beam.clone()))
                .collect();
        }

        encountered_states
            .iter()
            .map(|bi| bi.location)
            .collect::<HashSet<Point2D>>()
            .len()
    }
}

impl Solution for Problem {
    type Part1 = usize;
    type Part2 = usize;

    fn part1(&mut self) -> Result<Self::Part1> {
        Ok(self.determine_energization(BeamInstant {
            location: Point2D::ORIGIN,
            direction: Direction::Right,
        }))
    }

    fn part2(&self) -> Result<Self::Part2> {
        let top_edge = (0..self.map.bounds.width).map(|x| BeamInstant {
            location: pt(x, 0),
            direction: Direction::Down,
        });
        let bottom_edge = (0..self.map.bounds.width).map(|x| BeamInstant {
            location: pt(x, self.map.bounds.height - 1),
            direction: Direction::Up,
        });
        let left_edge = (0..self.map.bounds.height).map(|y| BeamInstant {
            location: pt(0, y),
            direction: Direction::Right,
        });
        let right_edge = (0..self.map.bounds.height).map(|y| BeamInstant {
            location: pt(self.map.bounds.width - 1, y),
            direction: Direction::Left,
        });

        Ok(top_edge
            .chain(bottom_edge)
            .chain(left_edge)
            .chain(right_edge)
            .map(|beam| self.determine_energization(beam))
            .max()
            .ok_or_invalid()?)
    }
}

impl Problem {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        let mut problem = Problem::from_str(SAMPLE)?;

        let result = problem.part1()?;

        assert_eq!(46, result);

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let problem = Problem::from_str(SAMPLE)?;

        let result = problem.part2()?;

        assert_eq!(51, result);

        Ok(())
    }

    const SAMPLE: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
}
