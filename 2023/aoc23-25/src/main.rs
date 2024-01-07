use std::str::FromStr;

use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    Problem::go()
}

struct Problem {
    graph: UnweightedGraphX<String>,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        let mut graph = UnweightedGraphX::default();
        for line in contents.lines() {
            let (node, connections) = line.split_once(": ").ok_or_invalid()?;
            graph.add_edges_bidi(
                node.to_string(),
                connections.split(' ').map(|name| name.to_string()),
            );
        }
        Ok(Self {
            // node_names: node_names.0,
            graph,
        })
    }
}

impl Solution for Problem {
    type Part1 = usize;
    type Part2 = &'static str;

    fn part1(&mut self) -> Result<Self::Part1> {
        // modified Edmonds-Karp maximal flow algorithm
        //
        // 1. Given a node, find the furthest node from it, call it start
        // 2. Find a second node, furthest from start, this is end
        // 3. There must be 3 distinct shortest paths between start and end. Find and remove those
        //    edges
        // 4. Now start must be connected to the size of the group

        let start = self.graph.furthest_node(0).ok_or_invalid()?.to_owned();
        let end = self.graph.furthest_node(start).ok_or_invalid()?.to_owned();

        for _ in 0..3 {
            let path = self
                .graph
                .shortest_path(start, end)
                .unwrap()
                .into_iter()
                .map(|p| p.to_owned())
                .collect::<Vec<_>>();
            for edge in path.windows(2) {
                self.graph.disconnect_bidi(edge[0], edge[1]);
            }
        }

        // we must have removed the min-cut, count nodes connected to start
        let group_1 = self.graph.find_connected_group(start);

        Ok(group_1.len() * (self.graph.node_count() - group_1.len()))
    }

    fn part2(&self) -> Result<Self::Part2> {
        Ok("N/A")
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
