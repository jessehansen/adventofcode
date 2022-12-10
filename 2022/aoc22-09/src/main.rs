use std::collections::HashSet;
use std::str::FromStr;

use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    go(Problem::parse)
}

struct Problem {
    moves: Vec<Move>,
}

struct Move {
    direction: Direction,
    distance: u32,
}

use Direction::*;

struct Dir(Direction);

impl FromStr for Dir {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Dir(Right)),
            "U" => Ok(Dir(Up)),
            "L" => Ok(Dir(Left)),
            "D" => Ok(Dir(Down)),
            _ => bail!("invalid direction"),
        }
    }
}

impl FromStr for Move {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, distance): (Dir, u32) = parse_pair(s, ' ')?;

        Ok(Move {
            direction: dir.0,
            distance,
        })
    }
}

impl Problem {
    fn parse(contents: &str) -> Result<Problem> {
        Ok(Problem {
            moves: parse_lines(contents)?,
        })
    }
}

struct Rope {
    knots: Vec<IPoint2D>,
    tail_visited: HashSet<IPoint2D>,
    ix_last: usize,
}

impl Rope {
    fn new(len: usize) -> Rope {
        let mut knots = vec![];
        for _ in 0..len {
            knots.push(IPoint2D::ORIGIN);
        }
        Rope {
            knots,
            tail_visited: HashSet::new(),
            ix_last: len - 1,
        }
    }

    fn mv(&mut self, direction: Direction) {
        self.knots[0] = self.knots[0].mv(direction);
        for ix_knot in 1..self.knots.len() {
            let knot = self.knots[ix_knot];
            let (dx, dy) = self.knots[ix_knot - 1].cardinal_distance(&knot);
            if dx.abs() > 1 || dy.abs() > 1 {
                self.knots[ix_knot] = knot.move_by(dx.signum(), dy.signum());
            }
        }
        self.tail_visited.insert(self.knots[self.ix_last]);
    }
}

impl Solution<usize, usize> for Problem {
    fn part1(&mut self) -> Result<usize> {
        let mut rope = Rope::new(2);
        for mv in &self.moves {
            for _ in 0..mv.distance {
                rope.mv(mv.direction);
            }
        }

        Ok(rope.tail_visited.len())
    }

    fn part2(&self) -> Result<usize> {
        let mut rope = Rope::new(10);
        for mv in &self.moves {
            for _ in 0..mv.distance {
                rope.mv(mv.direction);
            }
        }

        Ok(rope.tail_visited.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        let mut problem = Problem::parse(SAMPLE)?;

        let result = problem.part1()?;

        assert_eq!(13, result);

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let problem = Problem::parse(SAMPLE)?;

        let result = problem.part2()?;

        assert_eq!(1, result);

        Ok(())
    }

    #[test]
    fn sample2_part2() -> Result<()> {
        let problem = Problem::parse(SAMPLE_2)?;

        let result = problem.part2()?;

        assert_eq!(36, result);

        Ok(())
    }

    const SAMPLE: &str = "\
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";

    const SAMPLE_2: &str = "\
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";
}
