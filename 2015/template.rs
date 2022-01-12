use aoc_common::*;

fn main() {
    run_vec(parse, part1, part2);
}

fn parse(contents: &str) -> Vec<String> {
    contents.lines().map(|x| x.to_string()).collect()
}

fn part1(contents: &[String]) -> usize {
    contents.len()
}

fn part2(contents: &[String]) -> usize {
    contents.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() {
        let parsed = parse(SAMPLE);

        let result = part1(&parsed);

        assert_eq!(result, 0);
    }

    #[test]
    #[ignore]
    fn sample_part2() {
        let parsed = parse(SAMPLE);

        let result = part2(&parsed);

        assert_eq!(result, 0);
    }

    const SAMPLE: &str = "\
";
}
