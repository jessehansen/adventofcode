use std::{
    cmp::min,
    collections::{HashMap, HashSet},
    str::FromStr,
};

use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    Problem::go()
}

struct Problem {
    bricks: Vec<Cuboid>,
    // brick at key is supported by brick at values
    supported_by: HashMap<usize, Vec<usize>>,
    // brick at key supports bricks at values
    supports: HashMap<usize, Vec<usize>>,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        let mut bricks = contents
            .lines()
            .map(|line| Ok(line.parse_pair('~')?.into()))
            .collect::<Result<Vec<Cuboid>>>()?;
        bricks.sort_unstable_by_key(|brick| min(brick.origin.z, brick.terminex.z));
        Ok(Self {
            bricks,
            supported_by: HashMap::new(),
            supports: HashMap::new(),
        })
    }
}

impl Problem {
    fn settle(&mut self) {
        // we've already sorted by min z, so looping from bottom to top should allow us to settle
        let mut settled: Vec<Cuboid> = vec![];
        for (brick_index, brick) in self.bricks.iter().enumerate() {
            let mut brick = *brick;
            loop {
                let settled_on: Vec<_> = settled
                    .iter()
                    .enumerate()
                    .filter_map(|(brick_below_index, brick_below)| {
                        if brick
                            .bottom_layer()
                            .any(|pt| brick_below.contains(&pt.shift_z_down()))
                        {
                            Some(brick_below_index)
                        } else {
                            None
                        }
                    })
                    .collect();

                if settled_on.is_empty() && brick.min_z() > 1 {
                    // lowest level should be 1
                    brick = brick.shift_down();
                } else {
                    for supporting_brick in &settled_on {
                        self.supports
                            .entry(*supporting_brick)
                            .or_default()
                            .push(brick_index);
                    }
                    self.supported_by.insert(brick_index, settled_on);
                    break;
                }
            }
            settled.push(brick);
        }

        self.bricks = settled;
    }

    fn count_chain_reaction(&self, brick_to_disintegrate: usize) -> usize {
        let mut falling_bricks = HashSet::new();
        self.add_falling_bricks(brick_to_disintegrate, &mut falling_bricks);
        falling_bricks.len()
    }

    fn add_falling_bricks(&self, brick_ix: usize, falling_bricks: &mut HashSet<usize>) {
        if let Some(supports) = self.supports.get(&brick_ix) {
            let mut recurse = vec![];

            for supported_ix in supports {
                if !self.supported_by[supported_ix].iter().any(|support_ix| {
                    brick_ix != *support_ix
                        && !falling_bricks.contains(support_ix)
                        && !supports.contains(support_ix)
                }) {
                    falling_bricks.insert(*supported_ix);
                    recurse.push(*supported_ix)
                }
            }
            for supported_ix in recurse {
                self.add_falling_bricks(supported_ix, falling_bricks);
            }
        }
    }
}

impl Solution for Problem {
    type Part1 = usize;
    type Part2 = usize;

    fn part1(&mut self) -> Result<Self::Part1> {
        self.settle();

        Ok((0..self.bricks.len())
            .filter(|brick_ix| {
                match self.supports.get(brick_ix) {
                    None => true, // if a brick doesn't support other bricks, it can be removed
                    Some(supports) =>
                    // a supporting brick can only be removed if every block it supports is
                    // supported by at least one other brick
                    {
                        supports
                            .iter()
                            .all(|supported_ix| self.supported_by[supported_ix].len() > 1)
                    }
                }
            })
            .count())
    }

    fn part2(&self) -> Result<Self::Part2> {
        Ok((0..self.bricks.len())
            .map(|brick_ix| self.count_chain_reaction(brick_ix))
            .sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        let mut problem = Problem::from_str(SAMPLE)?;

        let result = problem.part1()?;

        assert_eq!(5, result);

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let mut problem = Problem::from_str(SAMPLE)?;
        problem.settle();

        assert_eq!(6, problem.count_chain_reaction(0));
        assert_eq!(0, problem.count_chain_reaction(1));
        assert_eq!(0, problem.count_chain_reaction(2));
        assert_eq!(0, problem.count_chain_reaction(3));
        assert_eq!(0, problem.count_chain_reaction(4));
        assert_eq!(1, problem.count_chain_reaction(5));
        assert_eq!(0, problem.count_chain_reaction(6));

        assert_eq!(7, problem.part2()?);

        Ok(())
    }

    const SAMPLE: &str = "\
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
}
