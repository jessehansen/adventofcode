#![allow(clippy::needless_range_loop)]
use std::str::FromStr;

use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    Problem::go()
}

struct Problem {
    present_count: usize,
    start: usize,
}

const BLOCK_SIZE: usize = 100_000;

impl FromStr for Problem {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        let present_count = contents.parse_wrapped()?;
        let robins_inequality = [
            (100000, 4352000),
            (200000, 8912250),
            (300000, 13542990),
            (400000, 18218000),
            (500000, 22925240),
            (600000, 27657740),
            (700000, 32410980),
            (800000, 37181790),
            (900000, 41967820),
            (1000000, 46767260),
            (1100000, 51578680),
            (1200000, 56400920),
            (1300000, 61233020),
            (1400000, 66074170),
            (1500000, 70923680),
            (1600000, 75780960),
            (1700000, 80645490),
            (1800000, 85516820),
            (1900000, 90394550),
            (2000000, 95278320),
        ];
        // determine lowest possible house, skipping over a bunch of the range
        let (target, mut start) = (present_count, 0);

        for (key, value) in robins_inequality {
            if target >= value {
                start = key;
            } else {
                break;
            }
        }
        Ok(Self {
            present_count,
            start,
        })
    }
}

impl Problem {
    fn determine_house(&self, ppe: u32, max_50: bool) -> usize {
        let mut start = self.start;
        let target = self.present_count as u32;
        let mut end = start + BLOCK_SIZE;

        // Because each start value is also a multiple of BLOCK_SIZE,
        let mut houses = vec![
            // Pre-compute elf 1's contribution, if any
            if max_50 { 0 } else { ppe };
            BLOCK_SIZE
        ];

        // loop through blocks of 100_000 to easily eliminate some elves from contributing multiple
        // times
        loop {
            // Elves with numbers between start and end visit exactly once.
            for i in 0..BLOCK_SIZE {
                houses[i] = ppe * (start + i) as u32;
            }

            // Elves with numbers from block size to end / 2 may visit, but only once.
            for i in BLOCK_SIZE..(end / 2) {
                let presents = ppe * i as u32;
                let j = start.next_multiple_of(i) - start;

                if j < BLOCK_SIZE {
                    houses[j] += presents;
                }
            }

            let min_elf_contributing = if max_50 { start / 50 } else { 2 };
            // All remaining elves may visit multiple times
            for i in min_elf_contributing..BLOCK_SIZE {
                let presents = ppe * i as u32;
                let mut j = start.next_multiple_of(i) - start;
                let mut remaining = if max_50 {
                    51 - start.div_ceil(i)
                } else {
                    usize::MAX
                };

                while j < BLOCK_SIZE && remaining > 0 {
                    houses[j] += presents;
                    j += i;
                    remaining -= 1;
                }
            }

            if let Some(found) = houses.iter().position(|&p| p >= target) {
                return start + found;
            }

            start += BLOCK_SIZE;
            end += BLOCK_SIZE;
        }
    }
}

impl Solution for Problem {
    type Part1 = usize;
    type Part2 = usize;

    fn part1(&mut self) -> Result<Self::Part1> {
        dbg!(&self.start);
        Ok(self.determine_house(10, false))
    }

    fn part2(&self) -> Result<Self::Part2> {
        Ok(self.determine_house(11, true))
    }
}
