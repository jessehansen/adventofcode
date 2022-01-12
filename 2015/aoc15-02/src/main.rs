use aoc_common::*;
use std::cmp::min;
use std::str::FromStr;

fn main() {
    run_vec(parse, part1, part2);
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
    type Err = ();

    fn from_str(present: &str) -> Result<Self, Self::Err> {
        let mut parts = present.split('x');

        Ok(Present {
            length: parts.next().unwrap().parse().unwrap(),
            width: parts.next().unwrap().parse().unwrap(),
            height: parts.next().unwrap().parse().unwrap(),
        })
    }
}

fn parse(contents: &str) -> Vec<Present> {
    contents.lines().map(|x| x.parse().unwrap()).collect()
}

fn part1(contents: &[Present]) -> u32 {
    contents.iter().map(|x| x.paper_required()).sum()
}

fn part2(contents: &[Present]) -> u32 {
    contents.iter().map(|x| x.ribbon_required()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() {
        let parsed = parse(SAMPLE);

        let result = part1(&parsed);

        assert_eq!(result, 101);
    }

    #[test]
    fn sample_part2() {
        let parsed = parse(SAMPLE);

        let result = part2(&parsed);

        assert_eq!(result, 48);
    }

    const SAMPLE: &str = "\
2x3x4
1x1x10
";
}
