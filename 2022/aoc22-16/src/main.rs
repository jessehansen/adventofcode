use std::cmp::max;
use std::collections::HashMap;
use std::str::FromStr;

use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    Problem::go()
}

#[derive(Debug)]
struct Valve {
    name: String,
    flow_rate: u32,
    tunnels: Vec<String>,
}

impl FromStr for Valve {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Valve> {
        // Valve {} has flow rate={}; tunnels lead to valves {}
        let name = contents.substring(6, 2).to_string();
        let (_, rest) = contents
            .split_once('=')
            .ok_or(anyhow!("malformed valve, no flow rate"))?;
        let (flow_rate, rest) = rest
            .split_once(';')
            .ok_or(anyhow!("malformed valve, no semicolon after flow rate"))?;

        let (_, rest) = rest
            .split_once("valve")
            .ok_or(anyhow!("malformed valve, no 'valve' after semicolon"))?;

        let (_, tunnels) = rest.split_once(' ').ok_or(anyhow!(
            "malformed valve, no space after 'tunnels lead to valve(s)'"
        ))?;

        Ok(Valve {
            name,
            flow_rate: flow_rate.parse()?,
            tunnels: tunnels.split(", ").map(|x| x.to_string()).collect(),
        })
    }
}

struct Problem {
    cave: HashMap<String, Valve>,
    valve_indices: HashMap<String, usize>,
    adjacency: Vec<Vec<usize>>,
    flow: Vec<u32>,
    opt: Vec<Vec<Vec<u32>>>,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Problem> {
        let valves: Vec<Valve> = parse_lines(contents)?;

        Ok(Problem {
            cave: valves.into_iter().map(|v| (v.name.clone(), v)).collect(),
            valve_indices: HashMap::new(),
            adjacency: vec![],
            flow: vec![],
            opt: vec![],
        })
    }
}

impl Solution for Problem {
    type Part1 = u32;
    type Part2 = u32;

    fn part1(&mut self) -> Result<Self::Part1> {
        let mut valves_sorted: Vec<&Valve> = self.cave.values().collect();
        valves_sorted.sort_by(|a, b| b.flow_rate.cmp(&a.flow_rate));

        // put highest flow valves earlier
        self.valve_indices = valves_sorted
            .iter()
            .enumerate()
            .map(|(ix, valve)| (valve.name.clone(), ix))
            .collect();

        let non_zero_valves = valves_sorted.iter().filter(|v| v.flow_rate > 0).count();
        let len = valves_sorted.len();

        // build adjacency and flow based on valves

        self.adjacency = vec![vec![0usize; 0]; len];
        self.flow = vec![0u32; len];

        for v in &valves_sorted {
            let ix = self.valve_indices[&v.name];
            self.flow[ix] = v.flow_rate;
            for path in &v.tunnels {
                self.adjacency[ix].push(self.valve_indices[path]);
            }
        }

        // all non-zero valves can be opened
        let bitmask_max = 1 << non_zero_valves;

        // build a 3D array of values representing time elapsed, room, and which valves are closed
        // (1 in bitmask position x = valve x is closed)
        //
        // populate the array by brute force

        let mut opt = vec![vec![vec![0u32; bitmask_max]; len]; 30];

        for t in 1..30 {
            for ix in 0..len {
                let bitmask_ix = 1 << ix; // ix converted to bitmask,
                                          // i.e. 3 = 8 (1000), 6=64 (1000000)
                for x in 0..bitmask_max {
                    let mut o = opt[t][ix][x];
                    if bitmask_ix & x != 0 && t >= 2 {
                        o = max(o, opt[t - 1][ix][x - bitmask_ix] + self.flow[ix] * t as u32);
                    }
                    for &j in &self.adjacency[ix] {
                        o = max(o, opt[t - 1][j][x]);
                    }
                    opt[t][ix][x] = o;
                }
            }
        }

        self.opt = opt;

        let aa = self.valve_indices[&"AA".to_string()];
        Ok(self.opt[29][aa][bitmask_max - 1])
    }

    fn part2(&self) -> Result<Self::Part2> {
        let mut best = 0;
        let aa = self.valve_indices[&"AA".to_string()];
        let bitmask_max = self.opt[0][0].len();
        for x in 0..bitmask_max / 2 {
            let y = bitmask_max - 1 - x;
            best = max(best, self.opt[25][aa][x] + self.opt[25][aa][y]);
        }

        Ok(best)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        let mut problem = Problem::from_str(SAMPLE)?;

        let result = problem.part1()?;

        assert_eq!(1651, result);

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let mut problem = Problem::from_str(SAMPLE)?;

        problem.part1()?;
        let result = problem.part2()?;

        assert_eq!(1707, result);

        Ok(())
    }

    const SAMPLE: &str = "\
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
";
}
