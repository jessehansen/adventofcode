use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    run_vec(parse, part1, part2)
}

fn parse(contents: &str) -> Result<Vec<u32>> {
    parse_line_groups(contents, |elf_items: &str| -> Result<u32> {
        elf_items
            .lines()
            .map(|x: &str| -> Result<u32> { wrap_parse_error(x.parse()) })
            .sum()
    })
}

fn part1(contents: &[u32]) -> Result<u32> {
    contents
        .into_iter()
        .max()
        .copied()
        .ok_or(anyhow!("No calories for elf"))
}

fn part2(contents: &[u32]) -> Result<u32> {
    let mut sums: Vec<u32> = contents.to_owned();
    sums.sort_unstable_by(|a, b| b.cmp(a));
    sums.truncate(3);
    Ok(sums.into_iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        let parsed = parse(SAMPLE)?;

        let result = part1(&parsed)?;

        assert_eq!(24000, result);
        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let parsed = parse(SAMPLE)?;

        let result = part2(&parsed)?;

        assert_eq!(45000, result);
        Ok(())
    }

    const SAMPLE: &str = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";
}
