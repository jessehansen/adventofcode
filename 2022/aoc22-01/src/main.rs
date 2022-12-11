use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    go(Problem::parse)
}
struct Problem {
    calories: Vec<u32>,
}

impl Problem {
    fn parse(contents: &str) -> Result<Problem> {
        Ok(Problem {
            calories: parse_line_groups(contents, |elf_items: &str| -> Result<u32> {
                elf_items
                    .lines()
                    .map(|x: &str| -> Result<u32> { wrap_parse_error(x.parse()) })
                    .sum()
            })?,
        })
    }
}

impl Solution<u32, u32> for Problem {
    fn part1(&mut self) -> Result<u32> {
        self.calories
            .iter()
            .max()
            .copied()
            .ok_or_else(|| anyhow!("No calories for elf"))
    }

    fn part2(&self) -> Result<u32> {
        let mut sums: Vec<u32> = self.calories.clone();
        sums.sort_unstable_by(|a, b| b.cmp(a));
        sums.truncate(3);
        Ok(sums.into_iter().sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        let mut problem = Problem::parse(SAMPLE)?;

        let result = problem.part1()?;

        assert_eq!(24000, result);
        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let problem = Problem::parse(SAMPLE)?;

        let result = problem.part2()?;

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
