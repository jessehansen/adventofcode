use aoc_common::run;
use std::collections::HashMap;
use std::str::FromStr;

fn main() {
    run(parse, part1, part2);
}

struct FormulaInstructions {
    template: String,
    rules: HashMap<String, String>,
}

impl FromStr for FormulaInstructions {
    type Err = ();

    fn from_str(input: &str) -> Result<FormulaInstructions, Self::Err> {
        let mut parts = input.split("\n\n");

        Ok(FormulaInstructions {
            template: parts.next().unwrap().trim().to_string(),
            rules: parts
                .next()
                .unwrap()
                .lines()
                .map(|x| {
                    let mut parts = x.split(" -> ");
                    (
                        parts.next().unwrap().to_string(),
                        parts.next().unwrap().to_string(),
                    )
                })
                .collect(),
        })
    }
}

fn parse(contents: &str) -> FormulaInstructions {
    contents.parse().unwrap()
}

fn part1(instructions: &FormulaInstructions) -> usize {
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

    chars.values().max().unwrap() - chars.values().min().unwrap()
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

fn part2(instructions: &FormulaInstructions) -> u64 {
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

    chars.values().max().unwrap() - chars.values().min().unwrap()
}
