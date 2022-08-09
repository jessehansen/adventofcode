use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    run_raw(part1, part2)
}

fn part1(key: &str) -> Result<usize> {
    let mut num = 1;
    loop {
        num += 1;
        let hash = md5::compute(format!("{}{}", key, num));
        if hash[0] == 0 && hash[1] == 0 && hash[2] < 16 {
            return Ok(num);
        }
    }
}

fn part2(key: &str) -> Result<usize> {
    let mut num = 1;
    loop {
        num += 1;
        let hash = md5::compute(format!("{}{}", key, num));
        if hash[0] == 0 && hash[1] == 0 && hash[2] == 0 {
            return Ok(num);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        assert_eq!(part1("abcdef")?, 609043);
        assert_eq!(part1("pqrstuv")?, 1048970);

        Ok(())
    }
}
