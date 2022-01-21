use aoc_common::*;
use json::{parse, JsonValue};
use regex::Regex;

fn main() {
    run_raw(part1, part2);
}

fn part1(contents: &str) -> i32 {
    Regex::new(r"-?\d+")
        .unwrap()
        .find_iter(contents)
        .map(|x| x.as_str().parse::<i32>().unwrap())
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
        JsonValue::Array(_) => current.members().map(|value| sum_subtree(value)).sum(),
        JsonValue::Number(_) => current.as_i32().unwrap(),

        _ => 0,
    }
}

fn part2(contents: &str) -> i32 {
    sum_subtree(&parse(contents).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() {
        assert_eq!(part1("[1,2,3]"), 6);
        assert_eq!(part1("{\"a\":2,\"b\":4}"), 6);
        assert_eq!(part1("[[[3]]]"), 3);
        assert_eq!(part1("{\"a\":{\"b\":4},\"c\":-1}"), 3);
        assert_eq!(part1("{\"a\":[-1,1]}"), 0);
        assert_eq!(part1("[-1,{\"a\":1}]"), 0);
        assert_eq!(part1("[]"), 0);
        assert_eq!(part1("{}"), 0);
    }

    #[test]
    fn sample_part2() {
        assert_eq!(part2("[1,2,3]"), 6);
        assert_eq!(part2("[1,{\"c\":\"red\",\"b\":2},3]"), 4);
        assert_eq!(part2("{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}"), 0);
        assert_eq!(part2("[1,\"red\",5]"), 6);
    }
}
