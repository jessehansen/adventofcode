use std::{cmp::min, collections::HashSet, str::FromStr};

use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    Problem::go()
}

struct Problem {
    bricks: Vec<Cuboid>,
    // brick at key is supported by brick at values
    supported_by: Vec<Vec<usize>>,
    // brick at key supports bricks at values
    supports: Vec<Vec<usize>>,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        let mut bricks = contents
            .lines()
            .map(|line| Ok(line.parse_pair('~')?.into()))
            .collect::<Result<Vec<Cuboid>>>()?;
        bricks.sort_unstable_by_key(|brick| min(brick.origin.z, brick.terminex.z));
        let empty: Vec<_> = (0..(bricks.len())).map(|_| Vec::default()).collect();
        Ok(Self {
            bricks,
            supported_by: empty.clone(),
            supports: empty,
        })
    }
}

impl Problem {
    fn settle(&mut self) {
        // we've already sorted by min z, so looping from bottom to top should allow us to settle
        let mut settled: Vec<Cuboid> = vec![];
        for (brick_index, brick) in self.bricks.iter().enumerate() {
            let x_range = brick.x_range();
            let y_range = brick.y_range();
            let min_z = brick.min_z();
            let mut shift_by = 0;

            loop {
                let z_below = min_z - shift_by - 1;
                let settled_on: Vec<_> = settled
                    .iter()
                    .enumerate()
                    .filter_map(|(brick_below_index, brick_below)| {
                        let other_x = brick_below.x_range();
                        let other_y = brick_below.y_range();
                        let other_z = brick_below.z_range();
                        if other_x.overlaps(&x_range)
                            && other_y.overlaps(&y_range)
                            && other_z.contains(&z_below)
                        {
                            Some(brick_below_index)
                        } else {
                            None
                        }
                    })
                    .collect();

                if settled_on.is_empty() && z_below > 0 {
                    shift_by += 1;
                } else {
                    for supporting_brick in &settled_on {
                        self.supports[*supporting_brick].push(brick_index);
                    }
                    self.supported_by[brick_index] = settled_on;
                    break;
                }
            }
            settled.push(brick.shift_down_by(shift_by));
        }

        self.bricks = settled;
    }

    fn count_chain_reaction(&self, brick_to_disintegrate: usize) -> usize {
        let mut falling_bricks = HashSet::new();
        self.add_falling_bricks(brick_to_disintegrate, &mut falling_bricks);
        falling_bricks.len()
    }

    fn add_falling_bricks(&self, brick_ix: usize, falling_bricks: &mut HashSet<usize>) {
        let supports = &self.supports[brick_ix];
        let mut recurse = vec![];

        for &supported_ix in supports {
            if !self.supported_by[supported_ix].iter().any(|support_ix| {
                brick_ix != *support_ix
                    && !falling_bricks.contains(support_ix)
                    && !supports.contains(support_ix)
            }) {
                falling_bricks.insert(supported_ix);
                recurse.push(supported_ix)
            }
        }
        for supported_ix in recurse {
            self.add_falling_bricks(supported_ix, falling_bricks);
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
                self.supports[*brick_ix]
                    .iter()
                    .all(|supported_ix| self.supported_by[*supported_ix].len() > 1)
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
