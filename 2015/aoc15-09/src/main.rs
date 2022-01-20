use aoc_common::*;
use std::collections::HashMap;

fn main() {
    run(parse, part1, part2);
}

struct Map {
    destinations: HashMap<String, usize>,
    distances: Vec<Vec<u32>>,
}

fn parse(contents: &str) -> Map {
    let mut id = 0;
    let mut destinations = HashMap::new();
    let mut distances = vec![vec![10_000; 10]; 10];
    contents.lines().for_each(|x| {
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

        if id > 10 {
            panic!("Too many destinations");
        }

        parts.next(); // skip "="

        distances[from_id][from_id] = 0;
        distances[to_id][to_id] = 0;

        let distance = parts.next().unwrap().parse().unwrap();
        distances[from_id][to_id] = distance;
        distances[to_id][from_id] = distance;
    });

    if distances.len() < 10 {
        distances.truncate(destinations.len());
        distances
            .iter_mut()
            .for_each(|d| d.truncate(destinations.len()));
    }

    Map {
        destinations,
        distances,
    }
}

fn calculate_paths(start_id: usize, unvisited: Vec<&usize>, map: &Map) -> Vec<u32> {
    let mut result = vec![];
    for next_step_id in &unvisited {
        let future_steps: Vec<&usize> = unvisited
            .iter()
            .filter(|id| id != &next_step_id)
            .cloned()
            .collect();
        let step_distance = map.distances[start_id][**next_step_id];
        if !future_steps.is_empty() {
            let future_paths = calculate_paths(**next_step_id, future_steps, map);
            future_paths
                .into_iter()
                .for_each(|x| result.push(step_distance + x))
        } else {
            result.push(step_distance);
        }
    }
    result
}

fn all_paths(map: &Map) -> Vec<u32> {
    let mut result = vec![];
    for id in map.destinations.values() {
        result.append(&mut calculate_paths(
            *id,
            map.destinations.values().filter(|x| **x != *id).collect(),
            map,
        ));
    }
    result
}

fn part1(map: &Map) -> u32 {
    *all_paths(map).iter().min().unwrap()
}

fn part2(map: &Map) -> u32 {
    *all_paths(map).iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() {
        let parsed = parse(SAMPLE);

        let result = part1(&parsed);

        assert_eq!(result, 605);
    }

    #[test]
    fn sample_part2() {
        let parsed = parse(SAMPLE);

        let result = part2(&parsed);

        assert_eq!(result, 982);
    }

    const SAMPLE: &str = "\
London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141
";
}
