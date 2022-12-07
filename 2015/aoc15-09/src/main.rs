use anyhow::*;
use aoc_common::*;
use itertools::Itertools;
use std::collections::HashMap;

fn main() -> Result<()> {
    run(parse, part1, part2)
}

struct Map {
    destinations: HashMap<String, usize>,
    distances: Vec<Vec<u32>>,
}

fn parse(contents: &str) -> Result<Map> {
    let mut id = 0;
    let mut destinations = HashMap::new();
    let mut distances = vec![vec![10_000; 10]; 10];
    contents.lines().for_each(|x| {
        if id > 10 {
            return;
        }

        let mut parts = x.split_whitespace();
        let from = parts.next().unwrap();
        if !destinations.contains_key(from) {
            destinations.insert(from.to_string(), id);
            id += 1;
        }
        let from_id = destinations[from];
        parts.next(); // skip "to"
        let to = parts.next().unwrap();
        if !destinations.contains_key(to) {
            destinations.insert(to.to_string(), id);
            id += 1;
        }
        let to_id = destinations[to];

        parts.next(); // skip "="

        distances[from_id][from_id] = 0;
        distances[to_id][to_id] = 0;

        let distance = parts.next().unwrap().parse().unwrap();
        distances[from_id][to_id] = distance;
        distances[to_id][from_id] = distance;
    });

    if id > 10 {
        bail!("too many destinations");
    }

    if distances.len() < 10 {
        distances.truncate(destinations.len());
        distances
            .iter_mut()
            .for_each(|d| d.truncate(destinations.len()));
    }

    Ok(Map {
        destinations,
        distances,
    })
}

fn calculate_path(path: Vec<usize>, distances: &[Vec<u32>]) -> u32 {
    path.windows(2)
        .map(|step| {
            let from = step[0];
            let to = step[1];
            distances[from][to]
        })
        .sum()
}

fn all_paths(map: &Map) -> Vec<u32> {
    (0..map.destinations.len())
        .permutations(map.destinations.len())
        .map(|x| calculate_path(x, &map.distances))
        .collect()
}

fn part1(map: &Map) -> Result<u32> {
    all_paths(map)
        .iter()
        .min()
        .copied()
        .ok_or_else(|| anyhow!("no min"))
}

fn part2(map: &Map) -> Result<u32> {
    all_paths(map)
        .iter()
        .max()
        .copied()
        .ok_or_else(|| anyhow!("no max"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        let parsed = parse(SAMPLE)?;

        let result = part1(&parsed)?;

        assert_eq!(result, 605);

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let parsed = parse(SAMPLE)?;

        let result = part2(&parsed)?;

        assert_eq!(result, 982);

        Ok(())
    }

    const SAMPLE: &str = "\
London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141
";
}
