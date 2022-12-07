use anyhow::*;
use aoc_common::*;
use itertools::Itertools;
use std::collections::HashMap;
use std::str::FromStr;

fn main() -> Result<()> {
    run(parse_all, part1, part2)
}

struct SeatingHappiness {
    attendees: HashMap<String, usize>,
    happiness: Vec<Vec<i32>>,
}

impl FromStr for SeatingHappiness {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        let mut attendees = HashMap::new();
        let mut happiness = vec![vec![0; 100]; 100];

        for line in contents.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let person_a = parts[0];
            let happiness_delta = match parts[2] {
                "gain" => parts[3].parse().unwrap(),
                "lose" => -parts[3].parse::<i32>().unwrap(),
                _ => panic!(),
            };
            let person_b = parts[10].trim_end_matches('.');

            let person_a_index = match attendees.get(person_a) {
                Some(ix) => *ix,
                None => {
                    let ix = attendees.len();
                    attendees.insert(person_a.to_string(), ix);
                    ix
                }
            };
            let person_b_index = match attendees.get(person_b) {
                Some(ix) => *ix,
                None => {
                    let ix = attendees.len();
                    attendees.insert(person_b.to_string(), ix);
                    ix
                }
            };

            happiness[person_a_index][person_b_index] = happiness_delta;
        }

        happiness.truncate(attendees.len());
        happiness
            .iter_mut()
            .for_each(|row| row.truncate(attendees.len()));

        Ok(SeatingHappiness {
            attendees,
            happiness,
        })
    }
}

impl SeatingHappiness {
    fn add_me(&self) -> SeatingHappiness {
        let mut attendees = self.attendees.clone();
        attendees.insert("me".to_string(), attendees.len());
        let mut happiness = self.happiness.clone();
        happiness.iter_mut().for_each(|row| row.push(0));
        happiness.push(vec![0; attendees.len()]);

        SeatingHappiness {
            attendees,
            happiness,
        }
    }
}

fn all_happiness(contents: &SeatingHappiness) -> Vec<i32> {
    (0..contents.happiness.len())
        .permutations(contents.happiness.len())
        .map(|x| calculate_happiness(x, &contents.happiness))
        .collect()
}

fn calculate_happiness(seating: Vec<usize>, happiness: &[Vec<i32>]) -> i32 {
    let head = seating[0];
    let foot = seating[seating.len() - 1]; // technically the person to the left of head, but the naming was easier this way
    seating
        .windows(2)
        .map(|neighbors| {
            let a = neighbors[0];
            let b = neighbors[1];
            happiness[a][b] + happiness[b][a]
        })
        .sum::<i32>()
        + happiness[head][foot]
        + happiness[foot][head]
}

fn part1(contents: &SeatingHappiness) -> Result<i32> {
    all_happiness(contents)
        .into_iter()
        .max()
        .ok_or_else(|| anyhow!("no happiness"))
}

fn part2(contents: &SeatingHappiness) -> Result<i32> {
    all_happiness(&contents.add_me())
        .into_iter()
        .max()
        .ok_or_else(|| anyhow!("no happiness"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        let parsed = parse_all(SAMPLE)?;

        let result = part1(&parsed)?;

        assert_eq!(result, 330);

        Ok(())
    }

    const SAMPLE: &str = "\
Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.
";
}
