use std::collections::HashSet;

use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    run_raw(part1, part2)
}

fn item_score(c: &char) -> Result<u32> {
    match *c {
        'a'..='z' => Ok(*c as u32 - 'a' as u32 + 1),
        'A'..='Z' => Ok(*c as u32 - 'A' as u32 + 27),
        _ => bail!("undefined"),
    }
}

fn part1(contents: &str) -> Result<u32> {
    let mut score: u32 = 0;
    for line in contents.lines() {
        let compartments = line.split_at(line.len() / 2);
        let compartment1: HashSet<char> = compartments.0.chars().collect();
        let compartment2: HashSet<char> = compartments.1.chars().collect();
        let common_items: Vec<&char> = compartment1.intersection(&compartment2).collect();
        if common_items.len() != 1 {
            bail!("compartments didn't have one common item");
        }
        score += item_score(common_items[0])?;
    }
    Ok(score)
}

fn part2(contents: &str) -> Result<u32> {
    let mut score: u32 = 0;
    let lines: Vec<&str> = contents.lines().collect();
    for group in lines.chunks(3) {
        let sack1: HashSet<char> = group[0].chars().collect();
        let sack2: HashSet<char> = group[1].chars().collect();
        let sack3: HashSet<char> = group[2].chars().collect();

        let common_items: HashSet<char> = sack1.intersection(&sack2).copied().collect();
        let all_three: Vec<&char> = common_items.intersection(&sack3).collect();

        if all_three.len() != 1 {
            bail!("group didn't have one item in all three sacks");
        }
        score += item_score(all_three[0])?;
    }
    Ok(score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        let result = part1(&SAMPLE)?;

        assert_eq!(157, result);

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let result = part2(&SAMPLE)?;

        assert_eq!(70, result);

        Ok(())
    }

    const SAMPLE: &str = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";
}
