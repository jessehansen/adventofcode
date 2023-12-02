use std::{collections::HashSet, str::FromStr};

use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    Problem::go()
}

struct Group {
    people: Vec<Person>,
}
impl FromStr for Group {
    type Err = Error;

    fn from_str(group: &str) -> Result<Self> {
        Ok(Group {
            people: parse_lines(group)?,
        })
    }
}

impl Group {
    fn any_yes_qs(&self) -> HashSet<char> {
        let mut any_yes = self.people[0].yes_qs.clone();
        for person in self.people.iter().skip(1) {
            any_yes = any_yes.union(&person.yes_qs).copied().collect();
        }
        any_yes
    }

    fn all_yes_qs(&self) -> HashSet<char> {
        let mut all_yes = self.people[0].yes_qs.clone();
        for person in self.people.iter().skip(1) {
            all_yes = all_yes.intersection(&person.yes_qs).copied().collect();
        }
        all_yes
    }
}

struct Person {
    yes_qs: HashSet<char>,
}

impl FromStr for Person {
    type Err = Error;

    fn from_str(person: &str) -> Result<Self> {
        Ok(Person {
            yes_qs: person.chars().collect(),
        })
    }
}

struct Problem {
    groups: Vec<Group>,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Problem> {
        Ok(Problem {
            groups: parse_line_groups(contents)?,
        })
    }
}

impl Solution for Problem {
    type Part1 = usize;
    type Part2 = usize;

    fn part1(&mut self) -> Result<Self::Part1> {
        Ok(self.groups.iter().map(|g| g.any_yes_qs().len()).sum())
    }

    fn part2(&self) -> Result<Self::Part2> {
        Ok(self.groups.iter().map(|g| g.all_yes_qs().len()).sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        let mut problem = Problem::from_str(SAMPLE)?;

        let result = problem.part1()?;

        assert_eq!(11, result);

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let problem = Problem::from_str(SAMPLE)?;

        let result = problem.part2()?;

        assert_eq!(6, result);

        Ok(())
    }

    const SAMPLE: &str = "\
abc

a
b
c

ab
ac

a
a
a
a

b
";
}
