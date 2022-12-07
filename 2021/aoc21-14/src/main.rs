use anyhow::*;
use aoc_common::run;
use std::collections::HashMap;
use std::str::FromStr;

fn main() -> Result<()> {
    run(parse, part1, part2)
}

struct FormulaInstructions {
    template: String,
    rules: HashMap<String, String>,
}

impl FromStr for FormulaInstructions {
    type Err = Error;

    fn from_str(input: &str) -> Result<FormulaInstructions> {
        let mut parts = input.split("\n\n");

        let template = parts
            .next()
            .ok_or_else(|| anyhow!("missing template"))?
            .trim()
            .to_string();

        let rules: Result<Vec<(String, String)>> = parts
            .next()
            .ok_or_else(|| anyhow!("missing rules"))?
            .lines()
            .map(|x| {
                let mut parts = x.split(" -> ");
                Ok((
                    parts
                        .next()
                        .ok_or_else(|| anyhow!("missing rule input"))?
                        .to_string(),
                    parts
                        .next()
                        .ok_or_else(|| anyhow!("missing rule output"))?
                        .to_string(),
                ))
            })
            .collect();

        Ok(FormulaInstructions {
            template,
            rules: rules?.into_iter().collect(),
        })
    }
}

fn parse(contents: &str) -> Result<FormulaInstructions> {
    contents.parse()
}

fn part1(instructions: &FormulaInstructions) -> Result<usize> {
    let mut template = instructions.template.to_string();
    let rules = &instructions.rules;

    for _ in 0..10 {
        let mut i = 0;
        while i < template.len() - 1 {
            let pair = &template[i..i + 2];
            if let Some(insertion) = rules.get(pair) {
                template.insert_str(i + 1, insertion);
                i += insertion.len()
            }
            i += 1;
        }
    }

    let mut chars = HashMap::new();

    template.chars().for_each(|x| {
        let ct = chars.entry(x).or_insert(0);
        *ct += 1;
    });

    Ok(chars.values().max().unwrap() - chars.values().min().unwrap())
}

fn step2(
    pairs: &HashMap<String, u64>,
    chars: &mut HashMap<char, u64>,
    rules: &HashMap<String, String>,
) -> HashMap<String, u64> {
    let mut new_pairs = HashMap::<String, u64>::new();
    for (pair, count) in pairs {
        if let Some(insertion) = rules.get(pair) {
            // add insertion character to counts
            let ct = chars
                .entry(insertion.chars().next().unwrap())
                .or_insert(0_u64);
            *ct += count;

            // add new pair entries
            let mut pair_chars = pair.chars();
            let left_pair = format!("{}{}", pair_chars.next().unwrap(), insertion);
            let ct = new_pairs.entry(left_pair).or_insert(0);
            *ct += count;

            let right_pair = format!("{}{}", insertion, pair_chars.next().unwrap());
            let ct = new_pairs.entry(right_pair).or_insert(0);
            *ct += count;
        } else {
            let ct = new_pairs.entry(pair.to_string()).or_insert(0);
            *ct += count;
        }
    }
    new_pairs
}

fn part2(instructions: &FormulaInstructions) -> Result<u64> {
    let rules = &instructions.rules;

    let mut chars = HashMap::<char, u64>::new();
    instructions.template.chars().for_each(|x| {
        let ct = chars.entry(x).or_insert(0_u64);
        *ct += 1;
    });

    let mut pairs = HashMap::<String, u64>::new();

    for i in 0..instructions.template.len() - 1 {
        let pair = &instructions.template[i..i + 2];
        let ct = pairs.entry(pair.to_string()).or_insert(0_u64);
        *ct += 1
    }

    for _ in 0..40 {
        pairs = step2(&pairs, &mut chars, rules);
    }

    Ok(chars.values().max().unwrap() - chars.values().min().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        let parsed = parse(SAMPLE)?;

        let result = part1(&parsed)?;

        assert_eq!(result, 1588);

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let parsed = parse(SAMPLE)?;

        let result = part2(&parsed)?;

        assert_eq!(result, 2188189693529);

        Ok(())
    }

    const SAMPLE: &str = "\
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
";
}
