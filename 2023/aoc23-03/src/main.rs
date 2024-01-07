use std::str::FromStr;

use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    Problem::go()
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
enum GridSpace {
    #[default]
    Empty,
    Digit(u32),
    Symbol(char),
}

impl FromStr for GridSpace {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        let c = contents.chars().next().ok_or_invalid()?;
        match c {
            '0'..='9' => Ok(Digit(c.to_digit(10).ok_or_invalid()?)),
            '.' => Ok(Empty),
            c => Ok(Symbol(c)),
        }
    }
}

use GridSpace::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Number {
    start: Point2D,
    end: Point2D,
    value: u32,
    adjacent: Rect,
}

impl Number {
    fn new(start: Point2D, end: Point2D, value: u32, bounds: &Bounds2D) -> Number {
        let ul = start.up().unwrap_or(start);
        let ul = ul.left().unwrap_or(ul);
        let dr = end.down(bounds.height).unwrap_or(end);
        let dr = dr.right(bounds.width).unwrap_or(dr);
        Number {
            start,
            end,
            value,
            adjacent: Rect::new(ul, dr),
        }
    }
}

struct Problem {
    grid: Grid2D<GridSpace>,
    numbers: Vec<Number>,
    gear_locations: Vec<Point2D>,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        let grid = Grid2D::from_char_str(contents)?;
        let mut numbers = vec![];
        let mut gear_locations = vec![];

        let mut cur_number_start: Option<Point2D> = None;
        let mut last_pt = Point2D::ORIGIN;
        let mut cur_number = 0;

        for (pt, space) in grid.iter_horizontal() {
            if let Some(start) = cur_number_start {
                if !matches!(space, Digit(_)) || pt.y != start.y {
                    numbers.push(Number::new(start, last_pt, cur_number, &grid.bounds));
                    cur_number = 0;
                    cur_number_start = None;
                }
            }
            if let Digit(value) = space {
                cur_number = cur_number * 10 + value;
                if cur_number_start.is_none() {
                    cur_number_start = Some(pt);
                }
            }
            if space == &Symbol('*') {
                gear_locations.push(pt);
            }
            last_pt = pt;
        }
        if let Some(start) = cur_number_start {
            numbers.push(Number::new(start, last_pt, cur_number, &grid.bounds));
        }

        Ok(Self {
            grid,
            numbers,
            gear_locations,
        })
    }
}

impl Solution for Problem {
    type Part1 = u32;
    type Part2 = u32;

    fn part1(&mut self) -> Result<Self::Part1> {
        Ok(self
            .numbers
            .iter()
            .filter(|num| {
                let start = num.start;
                let end = num.end;
                start.to(&end).any(|pt| {
                    pt.neighbors(self.grid.bounds)
                        .any(|pt| matches!(self.grid[pt], Symbol(_)))
                })
            })
            .map(|num| num.value)
            .sum())
    }

    fn part2(&self) -> Result<Self::Part2> {
        Ok(self
            .gear_locations
            .iter()
            .map(|pt| self.gear_ratio(pt))
            .sum())
    }
}

impl Problem {
    fn gear_ratio(&self, pt: &Point2D) -> u32 {
        if self.grid[pt] != Symbol('*') {
            panic!("invalid gear location");
        }

        let (len, product) = self
            .numbers
            .iter()
            .filter_map(|num| {
                if num.adjacent.contains(pt) {
                    Some(num.value)
                } else {
                    None
                }
            })
            .fold((0, 1), |acc, value| (acc.0 + 1, acc.1 * value));

        if len > 1 {
            product
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        let mut problem = Problem::from_str(SAMPLE)?;

        let result = problem.part1()?;

        assert_eq!(4361, result);

        Ok(())
    }

    #[test]
    fn test_gear_ratio() -> Result<()> {
        let problem = Problem::from_str(SAMPLE)?;

        assert_eq!(16345, problem.gear_ratio(&pt(3, 1)));
        assert_eq!(451490, problem.gear_ratio(&pt(5, 8)));
        assert_eq!(0, problem.gear_ratio(&pt(3, 4)));

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let problem = Problem::from_str(SAMPLE)?;

        let result = problem.part2()?;

        assert_eq!(467835, result);

        Ok(())
    }

    const SAMPLE: &str = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";
}
