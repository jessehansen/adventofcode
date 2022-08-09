use anyhow::*;
use aoc_common::*;
use std::str::FromStr;

fn main() -> Result<()> {
    run_vec(parse, part1, part2)
}

enum Cmd {
    Forward,
    Down,
    Up,
}

impl FromStr for Cmd {
    type Err = Error;

    fn from_str(input: &str) -> Result<Cmd> {
        match input {
            "forward" => Ok(Cmd::Forward),
            "down" => Ok(Cmd::Down),
            "up" => Ok(Cmd::Up),
            _ => Err(anyhow!("invalid direction")),
        }
    }
}

struct CmdVec {
    command: Cmd,
    magnitude: u32,
}

impl FromStr for CmdVec {
    type Err = Error;

    fn from_str(input: &str) -> Result<CmdVec> {
        let parts: Vec<&str> = input.split_whitespace().collect();

        if parts.len() != 2 {
            bail!("invalid line, expected 2 parts, got {}", parts.len());
        }

        Ok(CmdVec {
            command: parts[0].parse()?,
            magnitude: parts[1].parse()?,
        })
    }
}

fn parse(contents: &str) -> Result<Vec<CmdVec>> {
    contents
        .lines()
        .into_iter()
        .map(|x| Ok(x.parse().context("invalid input")?))
        .collect()
}

fn part1(contents: &[CmdVec]) -> Result<u32> {
    let mut x = 0;
    let mut depth = 0;

    for cmd_vec in contents {
        match cmd_vec.command {
            Cmd::Forward => x += cmd_vec.magnitude,
            Cmd::Down => depth += cmd_vec.magnitude,
            Cmd::Up => depth -= cmd_vec.magnitude,
        }
    }

    Ok(x * depth)
}

fn part2(contents: &[CmdVec]) -> Result<u32> {
    let mut x = 0;
    let mut aim = 0;
    let mut depth = 0;

    for cmd_vec in contents {
        match cmd_vec.command {
            Cmd::Forward => {
                x += cmd_vec.magnitude;
                depth += aim * cmd_vec.magnitude;
            }
            Cmd::Down => aim += cmd_vec.magnitude,
            Cmd::Up => aim -= cmd_vec.magnitude,
        }
    }

    Ok(x * depth)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        let parsed = parse(SAMPLE)?;

        let result = part1(&parsed)?;

        assert_eq!(result, 150);

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let parsed = parse(SAMPLE)?;

        let result = part2(&parsed)?;

        assert_eq!(result, 900);

        Ok(())
    }

    const SAMPLE: &str = "\
forward 5
down 5
forward 8
up 3
down 8
forward 2
";
}
