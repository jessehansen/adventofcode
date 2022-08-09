use anyhow::*;
use aoc_common::*;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

fn main() -> Result<()> {
    run_vec(parse, part1, part2)
}

#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Point {
    type Err = Error;

    fn from_str(input: &str) -> Result<Point> {
        let parts = input
            .split(',')
            .map(|x| Ok(x.parse()?))
            .collect::<Result<Vec<i32>>>()?;

        if parts.len() != 2 {
            bail!("expected comma-separated ints");
        }

        Ok(Point {
            x: parts[0],
            y: parts[1],
        })
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Copy, Clone)]
struct Line {
    a: Point,
    b: Point,
}

impl FromStr for Line {
    type Err = Error;

    fn from_str(input: &str) -> Result<Line> {
        let parts: Vec<&str> = input.split_whitespace().collect();

        if parts.len() != 3 {
            bail!("expected 2 points separated by arrow")
        }

        Ok(Line {
            a: Point::from_str(parts[0])?,
            b: Point::from_str(parts[2])?,
        })
    }
}

struct LineIter {
    current: Point,
    end: Point,
    delta_x: i32,
    delta_y: i32,
    done: bool,
}

impl Line {
    fn iter(&self) -> LineIter {
        let delta_x = if self.a.x != self.b.x {
            if self.a.x > self.b.x {
                -1
            } else {
                1
            }
        } else {
            0
        };
        let delta_y = if self.a.y != self.b.y {
            if self.a.y > self.b.y {
                -1
            } else {
                1
            }
        } else {
            0
        };
        LineIter {
            current: self.a,
            end: self.b,
            delta_x,
            delta_y,
            done: false,
        }
    }
}

impl Iterator for LineIter {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        let result = self.current;
        if result.x == self.end.x && result.y == self.end.y {
            self.done = true;
        } else {
            self.current = Point {
                x: self.current.x + self.delta_x,
                y: self.current.y + self.delta_y,
            };
        }
        Some(result)
    }
}

fn parse(contents: &str) -> Result<Vec<Line>> {
    contents
        .lines()
        .into_iter()
        .map(|x| x.parse().context("invalid input"))
        .collect()
}

fn part1(lines: &[Line]) -> Result<usize> {
    let mut grid = HashMap::new();
    for line in lines {
        if line.a.x != line.b.x && line.a.y != line.b.y {
            continue;
        }
        for point in line.iter() {
            let magnitude = grid.entry(format!("{}", point)).or_insert(0);
            *magnitude += 1;
        }
    }

    Ok(grid.into_values().filter(|x| *x > 1).count())
}

fn part2(lines: &[Line]) -> Result<usize> {
    let mut grid = HashMap::new();
    for line in lines {
        for point in line.iter() {
            let magnitude = grid.entry(format!("{}", point)).or_insert(0);
            *magnitude += 1;
        }
    }

    Ok(grid.into_values().filter(|x| *x > 1).count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        let parsed = parse(SAMPLE)?;

        let result = part1(&parsed)?;

        assert_eq!(result, 5);

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let parsed = parse(SAMPLE)?;

        let result = part2(&parsed)?;

        assert_eq!(result, 12);

        Ok(())
    }

    const SAMPLE: &str = "\
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
";
}
