use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    run_vec(parse, part1, part2)
}

fn parse(contents: &str) -> Result<Vec<String>> {
    Ok(contents.lines().map(|x| x.to_string()).collect())
}

fn part1(contents: &[String]) -> Result<usize> {
    bail!("not implemented")
}

fn part2(contents: &[String]) -> Result<usize> {
    bail!("not implemented")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        let parsed = parse(SAMPLE)?;

        let result = part1(&parsed)?;

        bail!("not tested")
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let parsed = parse(SAMPLE)?;

        let result = part2(&parsed)?;

        bail!("not tested")
    }

    const SAMPLE: &str = "\
";
}
