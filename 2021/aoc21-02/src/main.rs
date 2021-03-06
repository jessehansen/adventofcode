use aoc_common::*;
use std::str::FromStr;

fn main() {
    run_vec(parse, part1, part2);
}

enum Cmd {
    Forward,
    Down,
    Up,
}

impl FromStr for Cmd {
    type Err = ();

    fn from_str(input: &str) -> Result<Cmd, Self::Err> {
        match input {
            "forward" => Ok(Cmd::Forward),
            "down" => Ok(Cmd::Down),
            "up" => Ok(Cmd::Up),
            _ => Err(()),
        }
    }
}

struct CmdVec {
    command: Cmd,
    magnitude: u32,
}

impl FromStr for CmdVec {
    type Err = ();

    fn from_str(input: &str) -> Result<CmdVec, Self::Err> {
        let parts: Vec<&str> = input.split_whitespace().collect();

        if parts.len() != 2 {
            return Err(());
        }

        match Cmd::from_str(parts[0]) {
            Ok(cmd) => match parts[1].parse() {
                Ok(mag) => Ok(CmdVec {
                    command: cmd,
                    magnitude: mag,
                }),
                Err(_) => Err(()),
            },
            Err(_) => Err(()),
        }
    }
}

fn parse(contents: &str) -> Vec<CmdVec> {
    contents
        .lines()
        .into_iter()
        .map(|x| x.parse().expect("invalid input"))
        .collect()
}

fn part1(contents: &[CmdVec]) -> u32 {
    let mut x = 0;
    let mut depth = 0;

    for cmd_vec in contents {
        match cmd_vec.command {
            Cmd::Forward => x += cmd_vec.magnitude,
            Cmd::Down => depth += cmd_vec.magnitude,
            Cmd::Up => depth -= cmd_vec.magnitude,
        }
    }

    x * depth
}

fn part2(contents: &[CmdVec]) -> u32 {
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

    x * depth
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() {
        let parsed = parse(SAMPLE);

        let result = part1(&parsed);

        assert_eq!(result, 150);
    }

    #[test]
    fn sample_part2() {
        let parsed = parse(SAMPLE);

        let result = part2(&parsed);

        assert_eq!(result, 900);
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
