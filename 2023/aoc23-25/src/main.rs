use std::{
    cmp::{max, min, Reverse},
    collections::{HashMap, HashSet},
    hash::Hash,
    str::FromStr,
};

use anyhow::*;
use aoc_common::*;
use rand::{thread_rng, Rng};

fn main() -> Result<()> {
    Problem::go()
}

type NodeId = usize;

struct Problem {
    // node_names: Vec<String>,
    nodes: HashMap<NodeId, HashSet<NodeId>>,
}

struct NodeIdBuilder(Vec<String>);

impl NodeIdBuilder {
    fn get_or_add(&mut self, name: &str) -> NodeId {
        if let Some(ix) =
            self.0.iter().enumerate().find_map(
                |(ix, added_name)| {
                    if name == added_name {
                        Some(ix)
                    } else {
                        None
                    }
                },
            )
        {
            ix
        } else {
            let ix = self.0.len();
            self.0.push(name.to_string());
            ix
        }
    }
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        let mut node_names = NodeIdBuilder(vec![]);
        let mut nodes = HashMap::new();
        for line in contents.lines() {
            let (node, connections) = line.split_once(": ").ok_or_invalid()?;
            let node = node_names.get_or_add(node);
            for other_node in connections
                .split(' ')
                .map(|name| node_names.get_or_add(name))
            {
                nodes
                    .entry(node)
                    .or_insert_with(HashSet::new)
                    .insert(other_node);
                nodes
                    .entry(other_node)
                    .or_insert_with(HashSet::new)
                    .insert(node);
            }
        }
        Ok(Self {
            // node_names: node_names.0,
            nodes,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct PathState {
    node: NodeId,
    path: Vec<NodeId>,
}

impl OptimizationState for PathState {
    type CacheKey = PathState;

    type Score = Reverse<usize>;

    fn cache_key(&self) -> Self::CacheKey {
        Self {
            node: self.node,
            path: self.path.clone(),
        }
    }

    fn score(&self) -> Self::Score {
        Reverse(self.path.len())
    }
}

impl Problem {
    fn find_shortest_path(&self, from: NodeId, to: NodeId) -> Option<Vec<NodeId>> {
        dijkstra(
            PathState {
                node: from,
                path: vec![],
            },
            |state| {
                let mut path = state.path.to_vec();
                path.push(state.node);

                self.nodes[&state.node].iter().filter_map(move |next| {
                    if path.contains(next) {
                        None
                    } else {
                        Some(PathState {
                            node: *next,
                            path: path.clone(),
                        })
                    }
                })
            },
            |state| state.node == to,
        )
        .map(|state| state.path)
    }

    fn find_group(&self, start: NodeId) -> HashSet<NodeId> {
        let mut stack = vec![start];
        let mut group = HashSet::new();
        while let Some(next) = stack.pop() {
            for &connected in &self.nodes[&next] {
                if group.insert(connected) {
                    stack.push(connected);
                }
            }
        }
        group
    }
}

impl Solution for Problem {
    type Part1 = usize;
    type Part2 = usize;

    fn part1(&mut self) -> Result<Self::Part1> {
        let mut rng = thread_rng();
        let mut edge_counts = HashMap::new();
        // sample shortest paths between 300 random nodes, the 3 most encountered edges should be
        // the ones to cut
        for _ in 0..300 {
            let from = *self
                .nodes
                .keys()
                .nth(rng.gen_range(0..self.nodes.len()))
                .ok_or_invalid()?;
            let to = *self
                .nodes
                .keys()
                .nth(rng.gen_range(0..self.nodes.len()))
                .ok_or_invalid()?;

            if let Some(path) = self.find_shortest_path(from, to) {
                for steps in path.windows(2) {
                    let key = (min(steps[0], steps[1]), max(steps[0], steps[1]));
                    let entry = edge_counts.entry(key).or_insert(0);
                    *entry += 1;
                }
            }
        }

        let mut edge_counts: Vec<_> = edge_counts.into_iter().collect();
        edge_counts.sort_by(|a, b| b.1.cmp(&a.1));

        let mut edges_to_remove = vec![];

        for (edge, _) in &edge_counts[0..3] {
            edges_to_remove.push((edge.0, edge.1));
        }
        dbg!(&edges_to_remove);

        for edge in edges_to_remove {
            self.nodes
                .get_mut(&edge.0)
                .unwrap()
                .retain(|a| a != &edge.1);
            self.nodes
                .get_mut(&edge.1)
                .unwrap()
                .retain(|a| a != &edge.0);
        }

        let group_1 = self.find_group(0);
        let group_2 = self.find_group(
            *self
                .nodes
                .keys()
                .find(|node| !group_1.contains(node))
                .unwrap(),
        );

        Ok(group_1.len() * group_2.len())
    }

    fn part2(&self) -> Result<Self::Part2> {
        Ok(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        let mut problem = Problem::from_str(SAMPLE)?;

        let result = problem.part1()?;

        assert_eq!(54, result);

        Ok(())
    }

    const SAMPLE: &str = "\
jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";
}
