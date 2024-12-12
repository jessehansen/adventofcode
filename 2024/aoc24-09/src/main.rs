use std::{iter::repeat, str::FromStr};

use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    Problem::go()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Block {
    File(usize),
    Free,
}
use Block::*;

struct FileSystem(Vec<Block>);

struct Problem {
    fs: FileSystem,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        let blocks = contents
            .chars()
            .collect::<Vec<char>>()
            .chunks(2)
            .enumerate()
            .flat_map(|(id, chunk)| {
                repeat(File(id))
                    .take(chunk[0].to_string().parse().unwrap())
                    .chain(if chunk.len() > 1 {
                        repeat(Free).take(chunk[1].to_string().parse().unwrap())
                    } else {
                        repeat(Free).take(0)
                    })
            })
            .collect();
        Ok(Self {
            fs: FileSystem(blocks),
        })
    }
}

impl FileSystem {
    fn first_free(&self, len: usize) -> Option<usize> {
        (0..(self.0.len() + 1 - len)).find(|&ix_start| {
            self.0[ix_start..(ix_start + len)]
                .iter()
                .all(|block| *block == Free)
        })
    }

    fn defrag(&self) -> FileSystem {
        let mut defragged = FileSystem(self.0.clone());
        let mut next_free = defragged.first_free(1);

        for p in (0..defragged.0.len()).rev() {
            match defragged.0[p] {
                File(_) if next_free < Some(p) => {
                    defragged.0.swap(next_free.unwrap(), p);
                    next_free = defragged.first_free(1);
                }
                _ => {}
            }
        }

        defragged
    }

    fn checksum(&self) -> usize {
        self.0
            .iter()
            .enumerate()
            .map(|(pos, block)| match block {
                File(id) => id * pos,
                _ => 0,
            })
            .sum()
    }

    fn defrag_continuous(&self) -> FileSystem {
        // this assumes the fs is already contiguous, and will corrupt
        // if it is not
        let mut defragged = FileSystem(self.0.clone());

        let max_file_id = match defragged.0.last() {
            Some(File(id)) => *id,
            _ => 0,
        };

        for file_id in (0..=max_file_id).rev() {
            let block_start = defragged
                .0
                .iter()
                .enumerate()
                .filter_map(|(ix, block)| match block {
                    File(id) if *id == file_id => Some(ix),
                    _ => None,
                })
                .next()
                .unwrap();
            let len = defragged
                .0
                .iter()
                .filter(|block| block == &&File(file_id))
                .count();

            if let Some(target_start) = defragged.first_free(len) {
                if target_start < block_start {
                    for i in 0..len {
                        defragged.0.swap(target_start + i, block_start + i);
                    }
                }
            }
        }

        defragged
    }
}

impl Solution for Problem {
    type Part1 = usize;
    type Part2 = usize;

    fn part1(&mut self) -> Result<Self::Part1> {
        Ok(self.fs.defrag().checksum())
    }

    fn part2(&self) -> Result<Self::Part2> {
        Ok(self.fs.defrag_continuous().checksum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() -> Result<()> {
        let problem = Problem::from_str("12345")?;

        assert_eq!(problem.fs.0, vec![
            File(0),
            Free,
            Free,
            File(1),
            File(1),
            File(1),
            Free,
            Free,
            Free,
            Free,
            File(2),
            File(2),
            File(2),
            File(2),
            File(2),
        ]);

        Ok(())
    }

    #[test]
    fn defrag_test() -> Result<()> {
        let problem = Problem::from_str("12345")?;

        assert_eq!(problem.fs.first_free(1), Some(1));

        let defragged = problem.fs.defrag();

        assert_eq!(defragged.0, vec![
            File(0),
            File(2),
            File(2),
            File(1),
            File(1),
            File(1),
            File(2),
            File(2),
            File(2),
            Free,
            Free,
            Free,
            Free,
            Free,
            Free
        ]);

        Ok(())
    }

    #[test]
    fn sample_part1() -> Result<()> {
        let mut problem = Problem::from_str(SAMPLE)?;

        let result = problem.part1()?;

        assert_eq!(1928, result);

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let problem = Problem::from_str(SAMPLE)?;

        let result = problem.part2()?;

        assert_eq!(2858, result);

        Ok(())
    }

    const SAMPLE: &str = "2333133121414131402";
}
