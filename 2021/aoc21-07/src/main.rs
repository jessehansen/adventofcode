use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    run_vec(parse, part1, part2)
}

fn parse(contents: &str) -> Result<Vec<i32>> {
    contents
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse().context("invalid input"))
        .collect()
}

fn part1(contents: &[i32]) -> Result<i32> {
    let min = contents
        .iter()
        .min()
        .ok_or_else(|| anyhow!("contents missing values"))?;
    let max = contents
        .iter()
        .max()
        .ok_or_else(|| anyhow!("contents missing values"))?;

    let mut least_fuel = std::i32::MAX;

    for i in *min..*max {
        let fuel = contents.iter().map(|x| (x - i).abs()).sum();
        if fuel < least_fuel {
            least_fuel = fuel
        }
    }

    Ok(least_fuel)
}

// returns the value for the nth triangle number
// see https://en.wikipedia.org/wiki/Triangular_number
fn triangle_number(n: i32) -> i32 {
    (n * n + n) / 2
}

fn part2(contents: &[i32]) -> Result<i32> {
    let min = contents
        .iter()
        .min()
        .ok_or_else(|| anyhow!("contents missing values"))?;
    let max = contents
        .iter()
        .max()
        .ok_or_else(|| anyhow!("contents missing values"))?;

    let mut least_fuel = std::i32::MAX;

    for i in *min..*max {
        let fuel = contents
            .iter()
            .map(|x| triangle_number((x - i).abs()))
            .sum();
        if fuel < least_fuel {
            least_fuel = fuel
        }
    }

    Ok(least_fuel)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        let parsed = parse(SAMPLE)?;

        let result = part1(&parsed)?;

        assert_eq!(result, 37);

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let parsed = parse(SAMPLE)?;

        let result = part2(&parsed)?;

        assert_eq!(result, 168);

        Ok(())
    }

    const SAMPLE: &str = "\
16,1,2,0,4,2,7,1,2,14
";
}
