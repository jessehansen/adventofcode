use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    run_raw(part1, part2)
}

fn part1(contents: &str) -> Result<i32> {
    contents.chars().fold(Ok(0), |prev, c| match prev {
        std::result::Result::Ok(floor) => match c {
            '(' => Ok(floor + 1),
            ')' => Ok(floor - 1),
            _ => bail!("invalid char"),
        },
        err => err,
    })
}

fn part2(contents: &str) -> Result<usize> {
    let mut floor = 0;
    for (pos, c) in contents.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => bail!("invalid char"),
        }
        if floor < 0 {
            return Ok(pos + 1);
        }
    }

    bail!("no floor")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        assert_eq!(part1("(())")?, 0);
        assert_eq!(part1("()()")?, 0);
        assert_eq!(part1("(((")?, 3);
        assert_eq!(part1("(()(()(")?, 3);
        assert_eq!(part1("))(((((")?, 3);
        assert_eq!(part1("())")?, -1);
        assert_eq!(part1("))(")?, -1);
        assert_eq!(part1(")))")?, -3);
        assert_eq!(part1(")())())")?, -3);

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        assert_eq!(part2(")")?, 1);
        assert_eq!(part2("()())")?, 5);

        Ok(())
    }
}
