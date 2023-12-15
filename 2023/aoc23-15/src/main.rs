use std::str::FromStr;

use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    Problem::go()
}

fn hash_algorithm(ascii: &str) -> usize {
    ascii.chars().map(|c| c as usize).fold(0, |acc, value| {
        let value = acc + value;
        let value = value * 17;
        value % 256
    })
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
enum Operation {
    Remove,
    Add { focal_length: usize },
}

use Operation::*;

struct Instruction {
    label: String,
    op: Operation,
    label_hash: usize,
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self> {
        if value.ends_with("-") {
            let label = value.trim_end_matches('-');
            let label_hash = hash_algorithm(label);
            Ok(Instruction {
                label: label.to_string(),
                op: Remove,
                label_hash,
            })
        } else {
            let (label, focal_length) = value.split_once("=").ok_or_invalid()?;
            let label_hash = hash_algorithm(label);
            Ok(Instruction {
                label: label.to_string(),
                op: Add {
                    focal_length: focal_length.parse_wrapped()?,
                },
                label_hash,
            })
        }
    }
}

struct Problem {
    init_sequence: Vec<String>,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        Ok(Self {
            init_sequence: contents
                .lines()
                .flat_map(|line| line.split(',').map(|inst| inst.to_string()))
                .collect(),
        })
    }
}

impl Solution for Problem {
    type Part1 = usize;
    type Part2 = usize;

    fn part1(&mut self) -> Result<Self::Part1> {
        Ok(self
            .init_sequence
            .iter()
            .map(|seq| hash_algorithm(seq))
            .sum())
    }

    fn part2(&self) -> Result<Self::Part2> {
        let instructions: Vec<Instruction> = self
            .init_sequence
            .iter()
            .map(|inst| -> Result<Instruction> { inst.parse_wrapped() })
            .collect::<Result<Vec<_>>>()?;
        let mut boxes: Vec<Vec<(String, usize)>> = vec![vec![]; 256];

        for instruction in instructions {
            match instruction.op {
                Remove => {
                    boxes[instruction.label_hash].retain(|lens| lens.0 != instruction.label);
                }
                Add { focal_length } => {
                    let mut replaced = false;
                    for lens in boxes[instruction.label_hash].iter_mut() {
                        if lens.0 == instruction.label {
                            *lens = (instruction.label.clone(), focal_length);
                            replaced = true;
                        }
                    }
                    if !replaced {
                        boxes[instruction.label_hash]
                            .push((instruction.label.clone(), focal_length));
                    }
                }
            }
        }

        Ok(boxes
            .iter()
            .enumerate()
            .flat_map(|(box_num, b)| {
                b.iter()
                    .enumerate()
                    .map(move |(slot_num, lens)| (box_num + 1) * (slot_num + 1) * lens.1)
            })
            .sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_algorithm_test() {
        assert_eq!(52, hash_algorithm("HASH"));
        assert_eq!(30, hash_algorithm("rn=1"));
        assert_eq!(253, hash_algorithm("cm-"));
    }

    #[test]
    fn sample_part1() -> Result<()> {
        let mut problem = Problem::from_str(SAMPLE)?;

        let result = problem.part1()?;

        assert_eq!(1320, result);

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let problem = Problem::from_str(SAMPLE)?;

        let result = problem.part2()?;

        assert_eq!(145, result);

        Ok(())
    }

    const SAMPLE: &str = "\
rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
}
