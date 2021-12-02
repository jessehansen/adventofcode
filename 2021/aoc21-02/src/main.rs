use std::fs;
use std::str::FromStr;

fn main() {
    let sample = fs::read_to_string("./sample.txt")
        .expect("Something went wrong reading the file");
    let input = fs::read_to_string("./input.txt")
        .expect("Something went wrong reading the file");

    part1(&sample, "sample");
    part1(&input, "input");

    part2(&sample, "sample");
    part2(&input, "input");
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
            "forward"  => Ok(Cmd::Forward),
            "down"  => Ok(Cmd::Down),
            "up"  => Ok(Cmd::Up),
            _      => Err(()),
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

        return match Cmd::from_str(parts[0]) {
            Ok(cmd) => match parts[1].parse() {
                Ok(mag) => Ok(CmdVec{
                    command: cmd,
                    magnitude: mag,
                }),
                Err(_) => Err(())
            },
            Err(_) => Err(()),
        }
    }
}

fn parse(contents:&str) -> Vec<CmdVec> {
    return contents.lines().into_iter().map(|x| CmdVec::from_str(x).expect("invalid input")).collect();
}

fn part1(contents:&str, description: &str){
    let mut x = 0;
    let mut depth = 0;

    let contents = parse(contents);
    for cmd_vec in contents {
        match cmd_vec.command {
            Cmd::Forward => x += cmd_vec.magnitude,
            Cmd::Down => depth += cmd_vec.magnitude,
            Cmd::Up => depth -= cmd_vec.magnitude,
        }
    }

    println!("Answer Part 1 ({}) = x={}, depth={}, mult={}", description, x, depth, x * depth);
}

fn part2(contents:&str, description: &str){
    let mut x = 0;
    let mut aim = 0;
    let mut depth = 0;

    let contents = parse(contents);
    for cmd_vec in contents {
        match cmd_vec.command {
            Cmd::Forward => {
                x += cmd_vec.magnitude;
                depth += aim * cmd_vec.magnitude;
            },
            Cmd::Down => aim += cmd_vec.magnitude,
            Cmd::Up => aim -= cmd_vec.magnitude,
        }
    }

    println!("Answer Part 2 ({}) = x={}, depth={}, mult={}", description, x, depth, x * depth);
}
