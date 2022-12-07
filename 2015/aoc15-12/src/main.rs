use anyhow::*;
use aoc_common::*;
use json::{parse, JsonValue};
use regex::Regex;

fn main() -> Result<()> {
    run_raw(part1, part2)
}

fn part1(contents: &str) -> Result<i32> {
    Regex::new(r"-?\d+")?
        .find_iter(contents)
        .map(|x| wrap_parse_error(x.as_str().parse::<i32>()))
        .sum()
}

fn sum_subtree(current: &JsonValue) -> i32 {
    match current {
        JsonValue::Object(_) => {
            if current.entries().any(|(_, value)| value == "red") {
                0
            } else {
                current.entries().map(|(_, value)| sum_subtree(value)).sum()
            }
        }
        JsonValue::Array(_) => current.members().map(sum_subtree).sum(),
        JsonValue::Number(_) => current.as_i32().unwrap(),

        _ => 0,
    }
}

fn part2(contents: &str) -> Result<i32> {
    let json = parse(contents)?;
    Ok(sum_subtree(&json))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        assert_eq!(part1("[1,2,3]")?, 6);
        assert_eq!(part1("{\"a\":2,\"b\":4}")?, 6);
        assert_eq!(part1("[[[3]]]")?, 3);
        assert_eq!(part1("{\"a\":{\"b\":4},\"c\":-1}")?, 3);
        assert_eq!(part1("{\"a\":[-1,1]}")?, 0);
        assert_eq!(part1("[-1,{\"a\":1}]")?, 0);
        assert_eq!(part1("[]")?, 0);
        assert_eq!(part1("{}")?, 0);

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        assert_eq!(part2("[1,2,3]")?, 6);
        assert_eq!(part2("[1,{\"c\":\"red\",\"b\":2},3]")?, 4);
        assert_eq!(part2("{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}")?, 0);
        assert_eq!(part2("[1,\"red\",5]")?, 6);

        Ok(())
    }
}
