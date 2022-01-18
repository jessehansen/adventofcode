use aoc_common::*;
use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

fn main() {
    run_vec(parse, part1, part2);
}

enum Op {
    TurnOn,
    TurnOff,
    Toggle,
}

use Op::*;

impl FromStr for Op {
    type Err = ();

    fn from_str(op: &str) -> Result<Self, Self::Err> {
        match op {
            "turn on" => Ok(TurnOn),
            "turn off" => Ok(TurnOff),
            "toggle" => Ok(Toggle),
            _ => Err(()),
        }
    }
}

struct Instruction {
    op: Op,
    start: Point2D,
    end: Point2D,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(instruction: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^(?P<op>[^\d]+) (?P<start>\d+,\d+) through (?P<end>\d+,\d+)$")
                    .unwrap();
        }

        if let Some(caps) = RE.captures(instruction) {
            Ok(Instruction {
                op: caps["op"].parse().unwrap(),
                start: caps["start"].parse().unwrap(),
                end: caps["end"].parse().unwrap(),
            })
        } else {
            Err(())
        }
    }
}

fn parse(contents: &str) -> Vec<Instruction> {
    contents.lines().map(|x| x.parse().unwrap()).collect()
}

const BOUNDS: Bounds2D = Bounds2D {
    width: 1000,
    height: 1000,
};

fn part1(instructions: &[Instruction]) -> usize {
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
    on.iter_horizontal().filter(|(_, x)| **x).count()
}

fn part2(instructions: &[Instruction]) -> usize {
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
    lights.iter_horizontal().map(|(_, x)| x).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() {
        let parsed = parse(SAMPLE);

        let result = part1(&parsed);

        assert_eq!(result, 998_996);
    }

    #[test]
    #[ignore]
    fn sample_part2() {
        let parsed = parse(SAMPLE);

        let result = part2(&parsed);

        assert_eq!(result, 0);
    }

    const SAMPLE: &str = "\
turn on 0,0 through 999,999
toggle 0,0 through 999,0
turn off 499,499 through 500,500
";
}
