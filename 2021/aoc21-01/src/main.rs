use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    run_vec(parse, part1, part2)
}

fn parse(contents: &str) -> Result<Vec<u32>> {
    contents
        .lines()
        .map(|x| Ok(x.parse().context("invalid input")?))
        .collect()
}

fn part1(contents: &[u32]) -> Result<usize> {
    Ok(contents.windows(2).filter(|x| x[1] > x[0]).count())
}

fn part2(contents: &[u32]) -> Result<usize> {
    let windows_of_3: Vec<u32> = contents.windows(3).map(|x| x.iter().sum()).collect();

    Ok(windows_of_3.windows(2).filter(|x| x[1] > x[0]).count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        let parsed = parse(SAMPLE)?;

        let result = part1(&parsed)?;

        assert_eq!(result, 7);

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let parsed = parse(SAMPLE)?;

        let result = part2(&parsed)?;

        assert_eq!(result, 5);

        Ok(())
    }

    const SAMPLE: &str = "\
199
200
208
210
200
207
240
269
260
263
";
}
