use std::cmp::Ordering;
use std::fmt::Display;
use std::str::FromStr;

use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    Problem::go()
}

#[derive(Eq, PartialEq, Clone)]
enum PacketData {
    List(String),
    Integer(u32),
}
use PacketData::*;

impl FromStr for PacketData {
    type Err = Error;

    fn from_str(contents: &str) -> Result<PacketData> {
        Ok(
            match contents.chars().next().ok_or(anyhow!("missing data"))? {
                '[' => List(contents.to_string()),
                _ => Integer(contents.parse()?),
            },
        )
    }
}

impl Display for PacketData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Integer(i) => write!(f, "{i}"),
            List(s) => write!(f, "{s}"),
        }
    }
}

impl std::cmp::PartialOrd for PacketData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for PacketData {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Integer(s), Integer(o)) => s.cmp(o),
            (List(_), Integer(o)) => self.cmp(&List(format!("[{o}]"))),
            (Integer(s), List(_)) => List(format!("[{s}]")).cmp(other),
            (List(s), List(o)) => compare_list(s.as_str(), o.as_str()),
        }
    }
}

fn next_list_item(list: &str, pos: &mut usize) -> Option<PacketData> {
    let start = *pos;
    let mut open_brackets = 0;
    if list.chars().nth(*pos) == Some(',') {
        *pos += 1;
    }
    while *pos < list.len() {
        match list.chars().nth(*pos) {
            Some('[') => open_brackets += 1,
            Some(',') => {
                if open_brackets == 0 {
                    break;
                }
            }
            Some(']') => {
                open_brackets -= 1;
                if open_brackets < 0 {
                    // end of outer list
                    break;
                }
                if open_brackets == 0 {
                    *pos += 1;
                    break;
                }
            }
            None => return None,
            _ => {}
        }
        *pos += 1
    }
    let end = *pos;
    // advance to next item
    *pos += 1;
    list.chars()
        .skip(start)
        .take(end - start)
        .collect::<String>()
        .parse()
        .ok()
}

fn compare_list(left: &str, right: &str) -> Ordering {
    // compare the first value of each list, then the second value, and so on.
    // If the left list runs out of items first, the inputs are in the right order.
    // If the right list runs out of items first, the inputs are not in the right order.
    // If the lists are the same length and no comparison makes a decision about the order,
    // continue checking the next part of the input.

    if !left.starts_with('[') || !left.ends_with(']') {
        panic!("invalid list");
    }
    if !right.starts_with('[') || !right.ends_with(']') {
        panic!("invalid list");
    }

    // skip '['
    let mut lpos = 1;
    let mut rpos = 1;
    loop {
        match (
            next_list_item(left, &mut lpos),
            next_list_item(right, &mut rpos),
        ) {
            (Some(left_item), Some(right_item)) => {
                let ord = left_item.cmp(&right_item);
                if ord == Ordering::Equal {
                    continue;
                }
                return left_item.cmp(&right_item);
            }
            (None, Some(_)) => {
                return Ordering::Less;
            }
            (Some(_), None) => {
                return Ordering::Greater;
            }
            (None, None) => {
                return Ordering::Equal;
            }
        }
    }
}

struct Packet((PacketData, PacketData));

impl FromStr for Packet {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Packet> {
        Ok(Packet(parse_pair(contents, '\n')?))
    }
}

struct Problem {
    packets: Vec<Packet>,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Problem> {
        Ok(Problem {
            packets: parse_line_groups(contents)?,
        })
    }
}

impl Solution for Problem {
    type Part1 = usize;
    type Part2 = usize;

    fn part1(&mut self) -> Result<Self::Part1> {
        Ok(self
            .packets
            .iter()
            .enumerate()
            .filter(|(_, Packet((left, right)))| left < right)
            .map(|(ix, _)| ix + 1)
            .sum())
    }

    fn part2(&self) -> Result<Self::Part2> {
        let mut all_packets: Vec<&PacketData> = self
            .packets
            .iter()
            .flat_map(|Packet((left, right))| [left, right])
            .collect();
        let two = List("[[2]]".to_string());
        let six = List("[[6]]".to_string());
        all_packets.push(&two);
        all_packets.push(&six);
        all_packets.sort();

        let ix_two = all_packets
            .iter()
            .enumerate()
            .find(|&(_ix, packet)| packet == &&two)
            .ok_or(anyhow!("missing packet in sorted list"))?
            .0
            + 1;
        let ix_six = all_packets
            .iter()
            .enumerate()
            .find(|&(_ix, packet)| packet == &&six)
            .ok_or(anyhow!("missing packet in sorted list"))?
            .0
            + 1;

        Ok(ix_two * ix_six)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn less_than(left: &str, right: &str) -> bool {
        let left: PacketData = left.parse().expect("parse failure");
        let right: PacketData = right.parse().expect("parse failure");

        left < right
    }

    #[test]
    fn test_comparisons() -> Result<()> {
        assert!(less_than("[1,1,3,1,1]", "[1,1,5,1,1]"));

        assert!(less_than("[[1],[2,3,4]]", "[[1],4]"));

        assert!(!less_than("[9]", "[[8,7,6]]"));

        assert!(less_than("[[4,4],4,4]", "[[4,4],4,4,4]"));

        assert!(!less_than("[7,7,7,7]", "[7,7,7]"));

        assert!(less_than("[]", "[3]"));

        assert!(!less_than("[[[]]]", "[[]]"));

        assert!(!less_than(
            "[1,[2,[3,[4,[5,6,7]]]],8,9]",
            "[1,[2,[3,[4,[5,6,0]]]],8,9]"
        ));

        Ok(())
    }

    #[test]
    fn sample_part1() -> Result<()> {
        let mut problem = Problem::from_str(SAMPLE)?;

        let result = problem.part1()?;

        assert_eq!(13, result);

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let problem = Problem::from_str(SAMPLE)?;

        let result = problem.part2()?;

        assert_eq!(140, result);

        Ok(())
    }

    const SAMPLE: &str = "\
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";
}
