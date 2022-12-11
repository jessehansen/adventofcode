use std::collections::VecDeque;

use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    go(Problem::parse)
}

struct Problem {
    input: String,
}

impl Problem {
    fn parse(input: &str) -> Result<Problem> {
        Ok(Problem {
            input: input.to_string(),
        })
    }
}

fn index_of(buffer: &VecDeque<char>, c: &char) -> Option<usize> {
    for (ix, item) in buffer.iter().enumerate() {
        if item == c {
            return Some(ix);
        }
    }

    None
}

fn find_pos(contents: &str, marker_len: usize) -> Result<usize> {
    let mut buffer: VecDeque<char> = VecDeque::new();

    for (pos, c) in contents.chars().enumerate() {
        if let Some(ix) = index_of(&buffer, &c) {
            // this rotates characters moving the last occurrence of the repeated character to
            // the end of the ring buffer, then removes them
            //
            // ex: buffer=abcdef with current character d (ix=3)
            buffer.rotate_left(ix + 1);
            // buffer=efabcd
            buffer.resize(buffer.len() - ix - 1, ' ');
            // buffer=ef
        }
        buffer.push_back(c);
        if buffer.len() == marker_len {
            return Ok(pos + 1);
        }
    }

    bail!("no position found");
}

// alternate solution using .windows()
// same runtime as find_pos
fn find_pos_windows(contents: &str, marker_len: usize) -> Result<usize> {
    for (pos, window) in contents
        .chars()
        .collect::<Vec<char>>()
        .windows(marker_len)
        .enumerate()
    {
        if window
            .iter()
            .enumerate()
            .all(|(pos, c)| !window[pos + 1..].contains(c))
        {
            return Ok(pos + marker_len);
        }
    }

    bail!("no position found");
}

impl Solution<usize, usize> for Problem {
    fn part1(&mut self) -> Result<usize> {
        find_pos(self.input.as_str(), 4)
    }

    fn part2(&self) -> Result<usize> {
        find_pos_windows(self.input.as_str(), 14)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(7, find_pos("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4)?);
        assert_eq!(5, find_pos("bvwbjplbgvbhsrlpgdmjqwftvncz", 4)?);
        assert_eq!(6, find_pos("nppdvjthqldpwncqszvftbrmjlhg", 4)?);
        assert_eq!(10, find_pos("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4)?);
        assert_eq!(11, find_pos("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4)?);

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(19, find_pos_windows("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14)?);
        assert_eq!(23, find_pos_windows("bvwbjplbgvbhsrlpgdmjqwftvncz", 14)?);
        assert_eq!(23, find_pos_windows("nppdvjthqldpwncqszvftbrmjlhg", 14)?);
        assert_eq!(
            29,
            find_pos_windows("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14)?
        );
        assert_eq!(
            26,
            find_pos_windows("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14)?
        );

        Ok(())
    }
}
