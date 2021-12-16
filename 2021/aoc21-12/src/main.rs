use aoc_common::run;
use std::collections::HashMap;

fn main() {
    run(parse, part1, part2);
}

struct Map {
    nodes: HashMap<String, Node>,
}

fn calc_max_small_cave_visits(path_so_far: &[&str]) -> usize {
    let len = path_so_far.len();
    for index in 0..len {
        let node = path_so_far[index];
        if node.chars().next().unwrap().is_lowercase()
            && path_so_far[index + 1..len].iter().any(|x| *x == node)
        {
            return 1;
        }
    }
    2
}

impl Map {
    fn add_edge(&mut self, edge: [String; 2]) {
        for (node_name, other) in [(&edge[0], &edge[1]), (&edge[1], &edge[0])] {
            let node = self.nodes.entry(node_name.to_string()).or_insert(Node {
                edges: vec![],
                is_small: node_name.chars().next().unwrap().is_lowercase(),
            });

            if other != "start" {
                // don't care who is connected to start
                node.edges.push(other.to_string());
            }
        }
    }

    fn traverse<'a>(&'a self, at: String, path_so_far: Vec<&'a str>) -> Vec<Vec<&'a str>> {
        let mut paths = vec![];
        for next in &self.nodes[&at].edges {
            let next_node = &self.nodes[next];
            if next_node.is_small && path_so_far.iter().any(|x| x == &next) {
                continue;
            }
            let mut path = path_so_far.clone();
            path.push(&next);
            if next == "end" {
                paths.push(path);
            } else {
                for other_path in self.traverse(next.to_string(), path) {
                    paths.push(other_path);
                }
            }
        }
        paths
    }

    fn traverse2<'a>(&'a self, at: String, path_so_far: Vec<&'a str>) -> Vec<Vec<&'a str>> {
        let max_small_cave_visits = calc_max_small_cave_visits(&path_so_far);
        let mut paths = vec![];
        for next in &self.nodes[&at].edges {
            let next_node = &self.nodes[next];
            if next_node.is_small
                && path_so_far.iter().filter(|x| x == &next).count() >= max_small_cave_visits
            {
                continue;
            }
            let mut path = path_so_far.clone();
            path.push(&next);
            if next == "end" {
                paths.push(path);
            } else {
                for other_path in self.traverse2(next.to_string(), path) {
                    paths.push(other_path);
                }
            }
        }
        paths
    }
}

struct Node {
    edges: Vec<String>,
    is_small: bool,
}

fn parse(contents: &str) -> Map {
    let mut map = Map {
        nodes: HashMap::new(),
    };
    for line in contents.lines() {
        let mut edge = line.split('-');
        map.add_edge([
            edge.next().unwrap().to_string(),
            edge.next().unwrap().to_string(),
        ]);
    }
    map
}

fn part1(map: &Map) -> usize {
    let paths = map.traverse("start".to_string(), vec!["start"]);

    paths.len()
}

fn part2(map: &Map) -> usize {
    let paths = map.traverse2("start".to_string(), vec!["start"]);
    paths.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() {
        let parsed = parse(SAMPLE);

        let result = part1(&parsed);

        assert_eq!(result, 10);
    }

    #[test]
    fn sample_part2() {
        let parsed = parse(SAMPLE);

        let result = part2(&parsed);

        assert_eq!(result, 36);
    }

    #[test]
    fn sample_med_part1() {
        let parsed = parse(SAMPLE_MED);

        let result = part1(&parsed);

        assert_eq!(result, 19);
    }

    #[test]
    fn sample_med_part2() {
        let parsed = parse(SAMPLE_MED);

        let result = part2(&parsed);

        assert_eq!(result, 103);
    }

    #[test]
    fn sample_lrg_part1() {
        let parsed = parse(SAMPLE_LRG);

        let result = part1(&parsed);

        assert_eq!(result, 226);
    }

    #[test]
    fn sample_lrg_part2() {
        let parsed = parse(SAMPLE_LRG);

        let result = part2(&parsed);

        assert_eq!(result, 3509);
    }

    const SAMPLE: &str = "\
start-A
start-b
A-c
A-b
b-d
A-end
b-end
";

    const SAMPLE_MED: &str = "\
dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc
";

    const SAMPLE_LRG: &str = "\
fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW
";
}
