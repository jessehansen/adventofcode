use anyhow::*;
use aoc_common::*;
use std::cmp::min;
use std::str::FromStr;

fn main() -> Result<()> {
    run_vec(parse_lines, part1, part2)
}

struct Present {
    length: u32,
    width: u32,
    height: u32,
}

impl Present {
    fn paper_required(&self) -> u32 {
        2 * self.length * self.width
            + 2 * self.width * self.height
            + 2 * self.height * self.length
            + self.smallest_side()
    }
    fn ribbon_required(&self) -> u32 {
        self.smallest_perimiter() + self.volume()
    }

    fn smallest_side(&self) -> u32 {
        min(
            self.length * self.width,
            min(self.width * self.height, self.height * self.length),
        )
    }

    fn smallest_perimiter(&self) -> u32 {
        min(
            2 * (self.length + self.width),
            min(
                2 * (self.width + self.height),
                2 * (self.height + self.length),
            ),
        )
    }

    fn volume(&self) -> u32 {
        self.length * self.width * self.height
    }
}

impl FromStr for Present {
    type Err = Error;

    fn from_str(present: &str) -> Result<Self> {
        let mut parts = present.split('x');

        Ok(Present {
            length: parts.next().ok_or(anyhow!("missing length"))?.parse()?,
            width: parts.next().ok_or(anyhow!("missing width"))?.parse()?,
            height: parts.next().ok_or(anyhow!("missing height"))?.parse()?,
        })
    }
}

fn part1(contents: &[Present]) -> Result<u32> {
    Ok(contents.iter().map(|x| x.paper_required()).sum())
}

fn part2(contents: &[Present]) -> Result<u32> {
    Ok(contents.iter().map(|x| x.ribbon_required()).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        let parsed = parse_lines(SAMPLE)?;

        let result = part1(&parsed)?;

        assert_eq!(result, 101);

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let parsed = parse_lines(SAMPLE)?;

        let result = part2(&parsed)?;

        assert_eq!(result, 48);

        Ok(())
    }

    const SAMPLE: &str = "\
2x3x4
1x1x10
";
}
