use std::str::FromStr;

use anyhow::*;
use aoc_common::*;
use nom::{
    bytes::complete::{tag, take_until},
    IResult,
};

fn main() -> Result<()> {
    Problem::go()
}

struct RuleContent {
    color: String,
    count: usize,
}

struct Rule {
    color: String,
    contents: Vec<RuleContent>,
}

fn take_color(input: &str) -> IResult<&str, &str> {
    take_until(" bag")(input)
}

fn skip_bags_contain(input: &str) -> IResult<&str, &str> {
    tag(" bags contain ")(input)
}

fn take_number(input: &str) -> IResult<&str, &str> {
    delimited(char('('), take(2), char(')'))
}

fn take_contents(input: &str) -> IResult<&str, Vec<(usize, &str, &str)>> {
    separated_list0(tag(" , "), (take_number, take_color, skip_bags))
}

impl FromStr for Rule {
    type Err = Error;
    fn from_str(rule: &str) -> Result<Self> {
        let (color, _, contents, _) =
            (take_color, skip_bags_contain, take_contents, tag(".")).parse(rule)?;
        Ok(Self {
            color: color.to_owned(),
            contents,
        })
    }
}

struct Problem {
    rules: Vec<Rule>,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Problem> {
        Ok(Self {
            rules: parse_lines(contents)?,
        })
    }
}

impl Solution for Problem {
    type Part1 = usize;
    type Part2 = usize;

    fn part1(&mut self) -> Result<Self::Part1> {
        bail!("not implemented")
    }

    fn part2(&self) -> Result<Self::Part2> {
        bail!("not implemented")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        let mut problem = Problem::from_str(SAMPLE)?;

        let result = problem.part1()?;

        assert_eq!(0, result);

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let problem = Problem::from_str(SAMPLE)?;

        let result = problem.part2()?;

        assert_eq!(0, result);

        Ok(())
    }

    const SAMPLE: &str = "\
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
";
}
