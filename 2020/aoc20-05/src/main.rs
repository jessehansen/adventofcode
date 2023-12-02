use std::str::FromStr;

use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    Problem::go()
}

#[derive(Clone)]
struct Seat {
    id: usize,
}

impl FromStr for Seat {
    type Err = Error;

    fn from_str(seat: &str) -> Result<Self> {
        if seat.len() != 10 {
            bail!("invalid seat length");
        }

        let mut min_row = 0;
        let mut max_row = 128;

        for n in 0..=6 {
            match seat.chars().nth(n) {
                Some('F') => {
                    max_row -= (max_row - min_row) / 2;
                }
                Some('B') => {
                    min_row += (max_row - min_row) / 2;
                }
                _ => bail!("invalid seat row specifier"),
            }
        }
        let mut min_col = 0;
        let mut max_col = 8;
        for n in 7..=9 {
            match seat.chars().nth(n) {
                Some('L') => {
                    max_col -= (max_col - min_col) / 2;
                }
                Some('R') => {
                    min_col += (max_col - min_col) / 2;
                }
                Some(c) => bail!("invalid seat row specifier {c}"),
                _ => bail!("missing seat row specifier"),
            }
        }

        Ok(Seat {
            id: min_row * 8 + min_col,
        })
    }
}

struct Problem {
    filled_seats: Vec<Seat>,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Problem> {
        Ok(Problem {
            filled_seats: parse_lines(contents)?,
        })
    }
}

impl Solution for Problem {
    type Part1 = usize;
    type Part2 = usize;

    fn part1(&mut self) -> Result<Self::Part1> {
        self.filled_seats
            .iter()
            .map(|s| s.id)
            .max()
            .ok_or_else(|| anyhow!("no seats"))
    }

    fn part2(&self) -> Result<Self::Part2> {
        let mut seats = self.filled_seats.clone();
        seats.sort_by_key(|s| s.id);

        Ok(seats
            .windows(2)
            .find(|neighbors| neighbors[0].id != neighbors[1].id - 1)
            .ok_or_else(|| anyhow!("no missing neighbor"))?[0]
            .id
            + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seat_parse() -> Result<()> {
        assert_eq!(357, Seat::from_str("FBFBBFFRLR")?.id);
        assert_eq!(567, Seat::from_str("BFFFBBFRRR")?.id);
        assert_eq!(119, Seat::from_str("FFFBBBFRRR")?.id);
        assert_eq!(820, Seat::from_str("BBFFBBFRLL")?.id);

        Ok(())
    }

    #[test]
    fn sample_part1() -> Result<()> {
        let mut problem = Problem::from_str(SAMPLE)?;

        let result = problem.part1()?;

        assert_eq!(820, result);

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let problem = Problem::from_str(SAMPLE)?;

        let result = problem.part2()?;

        assert_eq!(120, result);

        Ok(())
    }

    const SAMPLE: &str = "\
FBFBBFFRLR
BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL
";
}
