use std::{fmt::Display, str::FromStr};

use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    Problem::go()
    // go(Problem::parse)
}

#[derive(Clone, Copy)]
enum Instruction {
    Noop,
    AddX(i32),
}

impl Instruction {
    fn cycle_count(&self) -> usize {
        match self {
            Instruction::Noop => 1,
            Instruction::AddX(_) => 2,
        }
    }
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');
        Ok(
            match parts.next().ok_or_else(|| anyhow!("missing instruction"))? {
                "noop" => Instruction::Noop,
                "addx" => Instruction::AddX(
                    parts
                        .next()
                        .ok_or_else(|| anyhow!("missing operand"))?
                        .parse()?,
                ),
                _ => bail!("unsupported instruction"),
            },
        )
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Noop => write!(f, "noop"),
            Instruction::AddX(dx) => write!(f, "addx {dx}"),
        }
    }
}

struct SamplingVM {
    cycle: i32,
    x: i32,
    samples: Vec<i32>,
}

impl SamplingVM {
    fn new() -> SamplingVM {
        SamplingVM {
            cycle: 0,
            x: 1,
            samples: vec![],
        }
    }
    fn advance_clock(&mut self) {
        self.cycle += 1;
        if self.cycle % 40 == 20 {
            self.samples.push(self.x * self.cycle);
        }
    }

    fn execute(&mut self, instruction: &Instruction) {
        for _ in 0..instruction.cycle_count() {
            self.advance_clock();
        }
        if let Instruction::AddX(dx) = instruction {
            self.x += dx;
        };
    }
}

struct SpriteVM {
    instructions: Vec<Instruction>,
    execution_pointer: usize,
    next_instruction_cycle: usize,
    x: i32,
    cycle: usize,
    display: String,
}

impl SpriteVM {
    fn new(instructions: &[Instruction]) -> SpriteVM {
        SpriteVM {
            instructions: instructions.to_vec(),
            execution_pointer: 0,
            next_instruction_cycle: instructions[0].cycle_count(),
            x: 1,
            cycle: 0,
            display: String::new(),
        }
    }

    fn tick(&mut self) {
        if self.cycle == self.next_instruction_cycle {
            if let Instruction::AddX(dx) = self.instructions[self.execution_pointer] {
                self.x += dx;
            };
            self.execution_pointer += 1;
            if self.execution_pointer < self.instructions.len() {
                self.next_instruction_cycle =
                    self.cycle + self.instructions[self.execution_pointer].cycle_count();
            } else {
                return;
            }
        }
        self.cycle += 1;
        let horiz_pos = ((self.cycle - 1) % 40) as i32;
        if horiz_pos - 1 <= self.x && horiz_pos + 1 >= self.x {
            self.display.push('#');
        } else {
            self.display.push('.');
        }
        if self.cycle % 40 == 0 {
            self.display.push('\n');
        }
    }

    fn run(&mut self) {
        let len = self.instructions.len();
        while self.execution_pointer < len {
            self.tick();
        }
    }
}

struct Problem {
    instructions: Vec<Instruction>,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Problem> {
        Ok(Problem {
            instructions: parse_lines(contents)?,
        })
    }
}

impl Solution for Problem {
    type Part1 = i32;
    type Part2 = String;

    fn part1(&mut self) -> Result<Self::Part1> {
        let mut vm = SamplingVM::new();
        for i in &self.instructions {
            vm.execute(i);
            if vm.cycle > 220 {
                break;
            }
        }

        Ok(vm.samples.into_iter().sum())
    }

    fn part2(&self) -> Result<Self::Part2> {
        let mut vm = SpriteVM::new(&self.instructions);

        vm.run();

        Ok(vm.display)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        let mut problem = Problem::from_str(SAMPLE)?;

        let result = problem.part1()?;

        assert_eq!(13140, result);

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let problem = Problem::from_str(SAMPLE)?;

        let result = problem.part2()?;

        println!("{}", result);
        println!("{}", EXPECTED_OUTPUT);

        assert_eq!(EXPECTED_OUTPUT, result);

        Ok(())
    }

    const SAMPLE: &str = "\
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";
    const EXPECTED_OUTPUT: &str = "\
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
";
}

/*
Cycle   1 -> ######################################## <- Cycle  40
Cycle  41 -> ######################################## <- Cycle  80
Cycle  81 -> ######################################## <- Cycle 120
Cycle 121 -> ######################################## <- Cycle 160
Cycle 161 -> ######################################## <- Cycle 200
Cycle 201 -> ######################################## <- Cycle 240
*/
