use std::str::FromStr;

use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    Problem::go()
}

#[derive(Debug)]
struct Machine {
    a: IPoint2D,
    b: IPoint2D,
    prize: IPoint2D,
}

struct Problem {
    machines: Vec<Machine>,
}

impl FromStr for Machine {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        let lines: Vec<&str> = contents.lines().collect();
        if lines.len() != 3 {
            bail!("invalid machine string");
        }

        let (_, a) = lines[0].split_once(": ").ok_or_invalid()?;
        let (a_x, a_y) = a.split_once(", ").ok_or_invalid()?;
        let a_x = a_x[2..].parse_wrapped()?;
        let a_y = a_y[2..].parse_wrapped()?;

        let (_, b) = lines[1].split_once(": ").ok_or_invalid()?;
        let (b_x, b_y) = b.split_once(", ").ok_or_invalid()?;
        let b_x = b_x[2..].parse_wrapped()?;
        let b_y = b_y[2..].parse_wrapped()?;

        let (_, prize) = lines[2].split_once(": ").ok_or_invalid()?;
        let (prize_x, prize_y) = prize.split_once(", ").ok_or_invalid()?;
        let prize_x = prize_x[2..].parse_wrapped()?;
        let prize_y = prize_y[2..].parse_wrapped()?;

        Ok(Machine {
            a: ipt(a_x, a_y),
            b: ipt(b_x, b_y),
            prize: ipt(prize_x, prize_y),
        })
    }
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        Ok(Self {
            machines: contents.parse_line_groups()?,
        })
    }
}

impl Machine {
    fn cheapest_win(&self) -> Option<i32> {
        let mut cheapest = None;

        for a_presses in 0..=100 {
            for b_presses in (if a_presses == 0 { 1 } else { 0 })..=100 {
                let cost = a_presses * 3 + b_presses;
                if cost == 0 || cheapest.is_some_and(|c| c <= cost) {
                    continue;
                }
                if self.a * a_presses + self.b * b_presses == self.prize {
                    cheapest = Some(cost);
                }
            }
        }

        cheapest
    }

    fn cheapest_win_part_2(&self) -> Option<i64> {
        let a_x = self.a.x as i64;
        let a_y = self.a.y as i64;

        let b_x = self.b.x as i64;
        let b_y = self.b.y as i64;

        let p_x = self.prize.x as i64 + 10000000000000;
        let p_y = self.prize.y as i64 + 10000000000000;

        // we have a system of equations with 2 vars = the number of a presses (a) and b presses
        // (b)
        //
        // prize_x = a(a_x) + b(b_x)
        // prize_y = a(a_y) + b(b_y)

        // solving the equations yields

        let a = (-b_x * p_y + b_y * p_x) / (a_x * b_y - a_y * b_x);
        let b = (a_x * p_y - a_y * p_x) / (a_x * b_y - a_y * b_x);

        // check if the solutions were integers - if so, this machine has a win condition
        if a_x * a + b_x * b == p_x && a_y * a + b_y * b == p_y {
            Some(a * 3 + b)
        } else {
            None
        }
    }
}

impl Solution for Problem {
    type Part1 = i32;
    type Part2 = i64;

    fn part1(&mut self) -> Result<Self::Part1> {
        Ok(self
            .machines
            .iter()
            .map(|m| m.cheapest_win().unwrap_or_default())
            .sum())
    }

    fn part2(&self) -> Result<Self::Part2> {
        Ok(self
            .machines
            .iter()
            .map(|m| m.cheapest_win_part_2().unwrap_or_default())
            .sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        let mut problem = Problem::from_str(SAMPLE)?;

        let result = problem.part1()?;

        assert_eq!(480, result);

        Ok(())
    }

    const SAMPLE: &str = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";
}
