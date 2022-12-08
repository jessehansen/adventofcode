use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    go(Problem::parse)
}

struct Problem {}

impl Problem {
    fn parse(contents: &str) -> Result<Problem> {
        Ok(Problem {})
    }
}

impl Solution<usize, usize> for Problem {
    fn part1(contents: &[String]) -> Result<_> {
        bail!("not implemented")
    }

    fn part2(contents: &[String]) -> Result<_> {
        bail!("not implemented")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        let mut problem = Problem::parse(SAMPLE)?;

        let result = problem.part1()?;

        assert_eq!(0, result);

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let problem = Problem::parse(SAMPLE)?;

        let result = problem.part2()?;

        assert_eq!(0, result);

        Ok(())
    }

    const SAMPLE: &str = "\
";
}
