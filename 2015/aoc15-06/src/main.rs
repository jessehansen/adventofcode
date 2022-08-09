use anyhow::*;
use aoc_common::*;
use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

fn main() -> Result<()> {
    run_vec(parse, part1, part2)
}

enum Op {
    TurnOn,
    TurnOff,
    Toggle,
}

use Op::*;

impl FromStr for Op {
    type Err = Error;

    fn from_str(op: &str) -> Result<Self> {
        match op {
            "turn on" => Ok(TurnOn),
            "turn off" => Ok(TurnOff),
            "toggle" => Ok(Toggle),
            unknown => bail!("unsupported op '{}'", unknown),
        }
    }
}

struct Instruction {
    op: Op,
    start: Point2D,
    end: Point2D,
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(instruction: &str) -> Result<Self> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^(?P<op>[^\d]+) (?P<start>\d+,\d+) through (?P<end>\d+,\d+)$")
                    .unwrap();
        }

        if let Some(caps) = RE.captures(instruction) {
            Ok(Instruction {
                op: caps["op"].parse()?,
                start: caps["start"].parse()?,
                end: caps["end"].parse()?,
            })
        } else {
            bail!("line didn't match pattern")
        }
    }
}

fn parse(contents: &str) -> Result<Vec<Instruction>> {
    contents.lines().map(|x| Ok(x.parse()?)).collect()
}

const BOUNDS: Bounds2D = Bounds2D {
    width: 1000,
    height: 1000,
};

fn part1(instructions: &[Instruction]) -> Result<usize> {
    let mut on = Grid2D::new_constant(BOUNDS, false);
    for instruction in instructions {
        instruction
            .start
            .to(instruction.end)
            .for_each(|pt| match instruction.op {
                TurnOn => on[pt] = true,
                TurnOff => on[pt] = false,
                Toggle => on[pt] = !on[pt],
            });
    }
    Ok(on.iter_horizontal().filter(|(_, x)| **x).count())
}

fn part2(instructions: &[Instruction]) -> Result<usize> {
    let mut lights = Grid2D::new_constant(BOUNDS, 0);
    for instruction in instructions {
        instruction
            .start
            .to(instruction.end)
            .for_each(|pt| match instruction.op {
                TurnOn => {
                    lights[pt] += 1;
                }
                TurnOff => {
                    if lights[pt] > 0 {
                        lights[pt] -= 1;
                    }
                }
                Toggle => {
                    lights[pt] += 2;
                }
            });
    }
    Ok(lights.iter_horizontal().map(|(_, x)| x).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        let parsed = parse(SAMPLE)?;

        let result = part1(&parsed)?;

        assert_eq!(result, 998_996);

        Ok(())
    }

    const SAMPLE: &str = "\
turn on 0,0 through 999,999
toggle 0,0 through 999,0
turn off 499,499 through 500,500
";
}
