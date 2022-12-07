use std::{fmt::Display, str::FromStr};

use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    run(parse, part1, part2)
}

fn parse(contents: &str) -> Result<(Port, Vec<Move>)> {
    parse_pair_by(contents, "\n\n", parse_untrimmed, parse_lines)
}

type Crate = char;

#[derive(Clone)]
struct Port {
    stacks: Vec<Vec<Crate>>,
}

impl Port {
    pub fn do_move(&mut self, mv: &Move) -> Result<()> {
        for _ in 0..mv.count {
            let c = self.stacks[mv.from - 1]
                .pop()
                .ok_or_else(|| anyhow!("tried to move from empty stack"))?;
            self.stacks[mv.to - 1].push(c);
        }

        Ok(())
    }

    pub fn do_move_9001(&mut self, mv: &Move) -> Result<()> {
        let last_grabbed_crate = self.stacks[mv.from - 1].len() - mv.count;
        let mut c: Vec<Crate> = self.stacks[mv.from - 1]
            .drain(last_grabbed_crate..)
            .collect();
        self.stacks[mv.to - 1].append(&mut c);

        Ok(())
    }

    pub fn top_crates(&self) -> Result<String> {
        let results = self
            .stacks
            .iter()
            .filter_map(|x| match x.len() {
                0 => None,
                len => Some(x[len - 1] as u8),
            })
            .collect();

        Ok(String::from_utf8(results)?)
    }
}

impl FromStr for Port {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self, Self::Err> {
        let mut bottom_up = contents.lines().rev();
        let stack_count = bottom_up
            .next()
            .ok_or_else(|| anyhow!("malformed stack grid"))?
            .len()
            / 4
            + 1;
        let mut stacks = Vec::with_capacity(stack_count);
        for _ in 0..stack_count {
            stacks.push(vec![]);
        }

        for line in bottom_up {
            for (pos, chunk) in line.as_bytes().chunks(4).enumerate() {
                if chunk.len() > 1 && chunk[1] != b' ' {
                    stacks[pos].push(
                        char::from_u32(chunk[1].into())
                            .ok_or_else(|| anyhow!("invalid crate character"))?
                            as Crate,
                    )
                }
            }
        }

        Ok(Port { stacks })
    }
}

impl Display for Port {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let max_len = self
            .stacks
            .iter()
            .map(|x| x.len())
            .max()
            .expect("Invalid port");

        for ix in (0..max_len).into_iter().rev() {
            for stack in &self.stacks {
                if ix < stack.len() {
                    write!(f, "[{}] ", stack[ix])?;
                } else {
                    write!(f, "    ")?;
                }
            }
            writeln!(f)?
        }

        for ix in 1..(self.stacks.len() + 1) {
            write!(f, " {ix}  ")?;
        }

        writeln!(f)?;

        std::fmt::Result::Ok(())
    }
}

struct Move {
    count: usize,
    from: usize,
    to: usize,
}

impl FromStr for Move {
    type Err = Error;

    fn from_str(mv: &str) -> Result<Self, Self::Err> {
        // "move {} from {} to {}"
        let (count, from, to) = grab_3(mv, " ", 1, 3, 5)?;
        Ok(Move { count, from, to })
    }
}

fn part1((starting_stacks, moves): &(Port, Vec<Move>)) -> Result<String> {
    let mut port = starting_stacks.clone();

    for mv in moves.iter() {
        port.do_move(mv)?;
    }

    port.top_crates()
}

fn part2((starting_stacks, moves): &(Port, Vec<Move>)) -> Result<String> {
    let mut port = starting_stacks.clone();

    for mv in moves.iter() {
        port.do_move_9001(mv)?;
    }

    port.top_crates()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        let parsed = parse(SAMPLE)?;

        let result = part1(&parsed)?;

        assert_eq!("CMZ", result);

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let parsed = parse(SAMPLE)?;

        let result = part2(&parsed)?;

        assert_eq!("MCD", result);

        Ok(())
    }

    const SAMPLE: &str = r#"    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"#;
}
