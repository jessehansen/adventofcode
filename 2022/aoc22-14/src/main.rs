use std::collections::HashSet;
use std::fmt::Display;
use std::str::FromStr;

use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    Problem::go()
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum CaveContent {
    Air,
    Rock,
    Sand,
}

impl Display for CaveContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Air => ".",
                Rock => "#",
                Sand => "o",
            }
        )
    }
}

use CaveContent::*;

struct Problem {
    cave: SparseGrid2D<CaveContent>,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Problem> {
        let rocks: HashSet<Point2D> = contents
            .lines()
            .flat_map(|line| {
                let points: Vec<Point2D> = parse_split(line, " -> ").expect("invalid line");

                points
                    .windows(2)
                    .flat_map(|endpoints| endpoints[0].to(endpoints[1]))
                    .collect::<Vec<Point2D>>()
            })
            .collect();

        Ok(Problem {
            cave: rocks.iter().map(|pt| (*pt, Rock)).collect(),
        })
    }
}

enum Step {
    Falling(Point2D),
    Rest(Point2D),
    FallingForever,
}

fn sand_step(cave: &SparseGrid2D<CaveContent>, sand: Point2D) -> Step {
    let bounds = cave.bounds().unwrap();
    let below = sand.down(bounds.height);
    match below {
        Some(pt_below) => match cave.get(&pt_below) {
            None | Some(Air) => Step::Falling(pt_below),
            Some(Rock) | Some(Sand) => match pt_below.left() {
                None => Step::FallingForever,
                Some(pt) => match cave.get(&pt) {
                    None | Some(Air) => Step::Falling(pt),
                    Some(Rock) | Some(Sand) => match pt_below.right(bounds.width) {
                        None => Step::FallingForever,
                        Some(pt) => match cave.get(&pt) {
                            None | Some(Air) => Step::Falling(pt),
                            Some(Rock) | Some(Sand) => Step::Rest(sand),
                        },
                    },
                },
            },
        },
        None => Step::FallingForever,
    }
}

fn drop_sand(cave: &mut SparseGrid2D<CaveContent>) -> bool {
    let mut sand = pt(500, 0);
    loop {
        match sand_step(cave, sand) {
            Step::Falling(pt) => sand = pt,
            Step::Rest(pt) => {
                cave.set(pt, Sand);
                return false;
            }
            Step::FallingForever => {
                return true;
            }
        }
    }
}

fn get_with_infinite_floor<'a>(
    cave: &'a SparseGrid2D<CaveContent>,
    pt: &'_ Point2D,
    floor_at_y: usize,
) -> &'a CaveContent {
    match cave.get(pt) {
        Some(content) => content,
        None => {
            if pt.y >= floor_at_y {
                &Rock
            } else {
                &Air
            }
        }
    }
}

fn sand_step_2(cave: &SparseGrid2D<CaveContent>, sand: Point2D, floor_at_y: usize) -> Step {
    let below = sand
        .down(floor_at_y + 1)
        .expect("invalid sand point below floor");
    match get_with_infinite_floor(cave, &below, floor_at_y) {
        Air => Step::Falling(below),
        Rock | Sand => match below.left() {
            None => panic!("left of origin"),
            Some(pt) => match get_with_infinite_floor(cave, &pt, floor_at_y) {
                Air => Step::Falling(pt),
                Rock | Sand => {
                    let pt = below.right_unbounded();
                    match get_with_infinite_floor(cave, &pt, floor_at_y) {
                        Air => Step::Falling(pt),
                        Rock | Sand => Step::Rest(sand),
                    }
                }
            },
        },
    }
}

fn drop_sand_2(cave: &mut SparseGrid2D<CaveContent>, floor_at_y: usize) -> bool {
    let mut sand = pt(500, 0);
    let mut counter = 0;
    loop {
        match sand_step_2(cave, sand, floor_at_y) {
            Step::Falling(pt) => sand = pt,
            Step::Rest(pt) => {
                cave.set(pt, Sand);
                if pt == aoc_common::pt(500, 0) {
                    // cave can not accept more sand
                    return true;
                }
                return false;
            }
            Step::FallingForever => {
                panic!("Fell out the bottom somehow");
            }
        }
        counter += 1;
        if counter > 500 {
            panic!("too many drops for sand")
        }
    }
}

impl Solution for Problem {
    type Part1 = usize;
    type Part2 = usize;

    fn part1(&mut self) -> Result<Self::Part1> {
        let mut cave = self.cave.clone();
        while !drop_sand(&mut cave) {}
        Ok(cave.into_iter().filter(|(_, x)| matches!(x, Sand)).count())
    }

    fn part2(&self) -> Result<Self::Part2> {
        let mut cave = self.cave.clone();
        let floor_at_y = cave.bounds().ok_or(anyhow!("invalid empty cave"))?.height + 1;

        while !drop_sand_2(&mut cave, floor_at_y) {}

        Ok(cave.into_iter().filter(|(_, x)| matches!(x, Sand)).count())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        let mut problem = Problem::from_str(SAMPLE)?;

        let result = problem.part1()?;

        assert_eq!(24, result);

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let problem = Problem::from_str(SAMPLE)?;

        let result = problem.part2()?;

        assert_eq!(93, result);

        Ok(())
    }

    #[test]
    fn get_with_infinite_floor_test() -> Result<()> {
        let problem = Problem::from_str(SAMPLE)?;

        assert_eq!(
            &Air,
            get_with_infinite_floor(
                &problem.cave,
                &pt(500, 6),
                problem.cave.bounds().unwrap().height + 1
            )
        );

        Ok(())
    }

    const SAMPLE: &str = "\
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
";
}
