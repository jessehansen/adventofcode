use std::{collections::HashMap, str::FromStr};

use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    Problem::go()
}

struct Update {
    pages: Vec<usize>,
}

struct Problem {
    rules: HashMap<usize, Vec<usize>>,
    updates: Vec<Update>,
}

impl FromStr for Update {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        Ok(Self {
            pages: contents.parse_split(',')?,
        })
    }
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        let (rules, updates) = contents.split_once("\n\n").ok_or_invalid()?;
        let rules = rules.lines().fold(
            HashMap::new(),
            |mut map: HashMap<usize, Vec<usize>>, rule| {
                if let Result::Ok((l, r)) = rule.parse_pair("|") {
                    map.entry(l).or_default().push(r);
                }
                map
            },
        );
        Ok(Self {
            rules,
            updates: updates.parse_lines()?,
        })
    }
}

impl Update {
    fn meets_rules(&self, rules: &HashMap<usize, Vec<usize>>) -> bool {
        for (early, late) in rules
            .iter()
            .flat_map(|(early, late)| late.iter().map(move |l| (early, l)))
        {
            let mut later_encountered = false;
            for p in &self.pages {
                if p == early && later_encountered {
                    return false;
                }
                if p == late {
                    later_encountered = true;
                }
            }
        }
        true
    }

    fn middle_page(&self) -> usize {
        self.pages[self.pages.len() / 2]
    }

    fn reorder_with(&self, rules: &HashMap<usize, Vec<usize>>) -> Update {
        let mut reordered = Update {
            pages: self.pages.clone(),
        };

        while !reordered.meets_rules(rules) {
            // until it's in the correct order, swap the first "early" page with the first "later"
            // page in the list
            //
            // This works for my input, not sure if it generalizes - it's probably possible to
            // craft an input where this never completes
            'outer: for (ix_early, &page_number) in reordered.pages.iter().enumerate() {
                if let Some(later_pages) = rules.get(&page_number) {
                    for &later_page in later_pages {
                        if let Some(ix_late) =
                            reordered.pages.iter().position(|&pn| pn == later_page)
                        {
                            if ix_early > ix_late {
                                reordered.pages.swap(ix_early, ix_late);
                                break 'outer;
                            }
                        }
                    }
                }
            }
        }

        reordered
    }
}

impl Solution for Problem {
    type Part1 = usize;
    type Part2 = usize;

    fn part1(&mut self) -> Result<Self::Part1> {
        Ok(self
            .updates
            .iter()
            .filter(|u| u.meets_rules(&self.rules))
            .map(|u| u.middle_page())
            .sum())
    }

    fn part2(&self) -> Result<Self::Part2> {
        Ok(self
            .updates
            .iter()
            .filter(|u| !u.meets_rules(&self.rules))
            .map(|u| u.reorder_with(&self.rules).middle_page())
            .sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        let mut problem = Problem::from_str(SAMPLE)?;

        let result = problem.part1()?;

        assert_eq!(143, result);

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let problem = Problem::from_str(SAMPLE)?;

        let result = problem.part2()?;

        assert_eq!(123, result);

        Ok(())
    }

    const SAMPLE: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";
}
