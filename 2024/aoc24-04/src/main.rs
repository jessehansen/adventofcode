use std::str::FromStr;

use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    Problem::go()
}

struct Problem {
    puzzle: Grid2D<char>,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        Ok(Self {
            puzzle: Grid2D::from_char_str(contents)?,
        })
    }
}

const MOVEMENT_DELTAS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

impl Solution for Problem {
    type Part1 = usize;
    type Part2 = usize;

    fn part1(&mut self) -> Result<Self::Part1> {
        Ok(self
            .puzzle
            .iter_horizontal()
            .filter(|(_, c)| **c == 'X')
            .map(|(pt, _)| {
                MOVEMENT_DELTAS
                    .iter()
                    .filter(|d| self.is_xmas(pt, **d))
                    .count()
            })
            .sum::<usize>())
    }

    fn part2(&self) -> Result<Self::Part2> {
        Ok(self
            .puzzle
            .iter_horizontal()
            .filter(|(pt, _)| self.is_x_mas(*pt))
            .count())
    }
}

const LETTERS: [char; 4] = ['X', 'M', 'A', 'S'];

impl Problem {
    fn is_xmas(&self, pt: Point2D, (dx, dy): (i32, i32)) -> bool {
        for i in 0..=3 {
            match pt.move_by_delta(dx * i, dy * i, self.puzzle.bounds) {
                Some(pt) => {
                    if self.puzzle[pt] != LETTERS[i as usize] {
                        return false;
                    }
                }
                _ => {
                    return false;
                }
            }
        }

        true
    }

    // MAS in X shape (4 ways)
    // M S    M M    S M    S S
    //  A      A      A      A
    // M S    S S    S M    M M
    fn is_x_mas(&self, pt: Point2D) -> bool {
        if let Some(a_point) = pt.move_by_delta(1, 1, self.puzzle.bounds) {
            if self.puzzle[a_point] != 'A' {
                return false;
            }

            let x: Vec<Point2D> = [
                Some(pt),
                pt.move_by_delta(2, 0, self.puzzle.bounds),
                pt.move_by_delta(0, 2, self.puzzle.bounds),
                pt.move_by_delta(2, 2, self.puzzle.bounds),
            ]
            .into_iter()
            .flatten()
            .collect();

            if x.len() != 4 {
                return false;
            }
            matches!(
                (
                    self.puzzle[x[0]],
                    self.puzzle[x[1]],
                    self.puzzle[x[2]],
                    self.puzzle[x[3]],
                ),
                ('M', 'S', 'M', 'S')
                    | ('M', 'M', 'S', 'S')
                    | ('S', 'M', 'S', 'M')
                    | ('S', 'S', 'M', 'M')
            )
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_horizontal_fwd_xmas() -> Result<()> {
        let problem = Problem::from_str(SAMPLE)?;

        assert!(problem.is_xmas(pt(5, 0), (1, 0)));

        Ok(())
    }

    #[test]
    fn sample_not_xmas() -> Result<()> {
        let problem = Problem::from_str(SAMPLE)?;

        assert!(!problem.is_xmas(pt(2, 2), (-1, 0)));

        Ok(())
    }

    #[test]
    fn sample_part1() -> Result<()> {
        let mut problem = Problem::from_str(SAMPLE)?;

        let result = problem.part1()?;

        assert_eq!(18, result);

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let problem = Problem::from_str(SAMPLE)?;

        let result = problem.part2()?;

        assert_eq!(9, result);

        Ok(())
    }

    const SAMPLE: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";
}
