use std::{collections::HashMap, ops::RangeInclusive, str::FromStr};

use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    Problem::go()
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
enum Category {
    XCool,
    Musical,
    Aero,
    Shiny,
}

use Category::*;

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
enum Target {
    Accept,
    Reject,
    Workflow(String),
}

use Target::*;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
enum Operation {
    All,
    LessThan(Category, usize),
    GreaterThan(Category, usize),
}

use Operation::*;

#[derive(Debug)]
struct Rule {
    op: Operation,
    target: Target,
}

struct Problem {
    workflows: HashMap<String, Vec<Rule>>,
    parts: Vec<HashMap<Category, usize>>,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        let (workflows, parts) = contents.split_once("\n\n").ok_or_invalid()?;
        Ok(Self {
            workflows: workflows
                .lines()
                .map(|line| -> Result<(String, Vec<Rule>)> {
                    let (name, rules) = line.split_once('{').ok_or_invalid()?;

                    Ok((
                        name.to_string(),
                        rules.trim_end_matches('}').parse_split(',')?,
                    ))
                })
                .collect::<Result<HashMap<_, _>>>()?,
            parts: parts
                .lines()
                .map(|part| -> Result<HashMap<Category, usize>> {
                    Ok(part
                        .trim_start_matches('{')
                        .trim_end_matches('}')
                        .split(',')
                        .map(|cat_val| -> Result<(Category, usize)> {
                            let (category, value) = cat_val.split_once('=').ok_or_invalid()?;
                            Ok((category.parse_wrapped()?, value.parse_wrapped()?))
                        })
                        .collect::<Result<HashMap<_, _>>>()?)
                })
                .collect::<Result<Vec<HashMap<_, _>>>>()?,
        })
    }
}

impl FromStr for Rule {
    type Err = Error;

    fn from_str(rule: &str) -> Result<Self> {
        if let Some((op, target)) = rule.split_once(':') {
            Ok(Self {
                op: op.parse_wrapped()?,
                target: target.parse_wrapped()?,
            })
        } else {
            Ok(Self {
                op: All,
                target: rule.parse_wrapped()?,
            })
        }
    }
}

impl FromStr for Operation {
    type Err = Error;

    fn from_str(op: &str) -> Result<Self> {
        if let Some((category, rhs)) = op.split_once('<') {
            Ok(LessThan(category.parse_wrapped()?, rhs.parse_wrapped()?))
        } else if let Some((category, rhs)) = op.split_once('>') {
            Ok(GreaterThan(category.parse_wrapped()?, rhs.parse_wrapped()?))
        } else {
            bail!("invalid operation");
        }
    }
}

impl FromStr for Target {
    type Err = Error;

    fn from_str(target: &str) -> Result<Self> {
        match target {
            "A" => Ok(Accept),
            "R" => Ok(Reject),
            _ => Ok(Workflow(target.to_string())),
        }
    }
}

impl FromStr for Category {
    type Err = Error;

    fn from_str(category: &str) -> Result<Self> {
        match category {
            "x" => Ok(XCool),
            "m" => Ok(Musical),
            "a" => Ok(Aero),
            "s" => Ok(Shiny),
            _ => bail!("invalid category {category}"),
        }
    }
}

impl Solution for Problem {
    type Part1 = usize;
    type Part2 = usize;

    fn part1(&mut self) -> Result<Self::Part1> {
        Ok(self
            .parts
            .iter()
            .filter(|part| self.is_accepted(part))
            .map(|part| part.values().sum::<usize>())
            .sum())
    }

    fn part2(&self) -> Result<Self::Part2> {
        // count solutions recursively, splitting each range when a rule is encountered
        Ok(self.count_solutions(
            "in",
            HashMap::from([
                (XCool, 1..=4000),
                (Musical, 1..=4000),
                (Aero, 1..=4000),
                (Shiny, 1..=4000),
            ]),
        ))
    }
}

type Range = RangeInclusive<i32>;
type Ranges = HashMap<Category, Range>;

impl Problem {
    fn is_accepted(&self, part: &&HashMap<Category, usize>) -> bool {
        let mut workflow = &self.workflows["in"];
        loop {
            for rule in workflow {
                match rule.exec(part) {
                    Some(Accept) => {
                        return true;
                    }
                    Some(Reject) => {
                        return false;
                    }
                    Some(Workflow(next)) => {
                        workflow = &self.workflows[next];
                        break;
                    }
                    _ => (),
                }
            }
        }
    }

    fn count_solutions(&self, current_workflow: &str, ranges: Ranges) -> usize {
        let mut ranges = ranges;
        let mut count = 0;
        for rule in &self.workflows[current_workflow] {
            let (ranges_with_target, next_rule_ranges) = rule.split_ranges(&ranges);
            // when a rule range matches, increment count by the number of solutions (recursively if a workflow)
            if let Some((target, new_ranges)) = ranges_with_target {
                count += match target {
                    Accept => range_combinations(new_ranges),
                    Reject => 0,
                    Workflow(workflow) => self.count_solutions(workflow, new_ranges),
                }
            }
            if let Some(new_ranges) = next_rule_ranges {
                // use new ranges and continue to the next rule
                ranges = new_ranges;
            }
        }
        count
    }
}

// ranges will always be split into either a range with a target, a range without a target (meaning
// this rule didn't match, so fall through to next rule), or both. I don't know of a rust type that
// is either/or/both and in practice we don't really care if it's either or both so I'll just use a
// pair of options with the associated types
type SplitRanges<'a> = (Option<(&'a Target, Ranges)>, Option<Ranges>);

impl Rule {
    fn exec(&self, part: &&HashMap<Category, usize>) -> Option<&Target> {
        match self.op {
            All => Some(&self.target),
            LessThan(category, rhs) if part[&category] < rhs => Some(&self.target),
            GreaterThan(category, rhs) if part[&category] > rhs => Some(&self.target),
            _ => None,
        }
    }

    fn split_ranges(&self, ranges: &Ranges) -> SplitRanges {
        match self.op {
            All => (Some((&self.target, ranges.clone())), None),

            LessThan(category, rhs) => {
                // figure out if the rule does or does not match for all of the range, or part of it
                let range = &ranges[&category];
                let rhs: i32 = rhs.try_into().unwrap();
                if range.contains(&rhs) {
                    // split the range into one that matches and one that doesn't
                    // < does not include rhs
                    let r1 = override_range(ranges, category, (*range.start())..=(rhs - 1));
                    let r2 = override_range(ranges, category, rhs..=(*range.end()));
                    (Some((&self.target, r1)), Some(r2))
                } else if rhs <= *range.start() {
                    // never matches
                    (None, Some(ranges.clone()))
                } else {
                    // always matches
                    (Some((&self.target, ranges.clone())), None)
                }
            }
            GreaterThan(category, rhs) => {
                let range = &ranges[&category];
                let rhs: i32 = rhs.try_into().unwrap();
                if range.contains(&rhs) {
                    // in this case, r1 does include rhs, but r2 does not
                    let r1 = override_range(ranges, category, (*range.start())..=rhs);
                    let r2 = override_range(ranges, category, (rhs + 1)..=(*range.end()));
                    (Some((&self.target, r2)), Some(r1))
                } else if rhs >= *range.end() {
                    (None, Some(ranges.clone()))
                } else {
                    (Some((&self.target, ranges.clone())), None)
                }
            }
        }
    }
}

fn override_range(ranges: &Ranges, category: Category, range: Range) -> Ranges {
    let mut ranges = ranges.clone();
    ranges.insert(category, range);
    ranges
}

fn range_combinations(ranges: Ranges) -> usize {
    ranges.values().map(|range| range.clone().count()).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        let mut problem = Problem::from_str(SAMPLE)?;

        let result = problem.part1()?;

        assert_eq!(19114, result);

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let problem = Problem::from_str(SAMPLE)?;

        let result = problem.part2()?;

        assert_eq!(167409079868000, result);

        Ok(())
    }

    const SAMPLE: &str = "\
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
}
