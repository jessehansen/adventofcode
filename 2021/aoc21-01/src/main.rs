use aoc_common::run;

fn main() {
    run(parse, part1, part2);
}

fn parse(contents: &str) -> Vec<u32> {
    contents
        .lines()
        .into_iter()
        .map(|x| x.parse().expect("invalid input"))
        .collect()
}

fn part1(contents: &Vec<u32>) -> usize {
    contents.windows(2).filter(|x| x[1] > x[0]).count()
}

fn part2(contents: &Vec<u32>) -> usize {
    let windows_of_3: Vec<u32> = contents.windows(3).map(|x| x.iter().sum()).collect();

    windows_of_3.windows(2).filter(|x| x[1] > x[0]).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() {
        let parsed = parse(SAMPLE);

        let result = part1(&parsed);

        assert_eq!(result, 7);
    }

    #[test]
    fn sample_part2() {
        let parsed = parse(SAMPLE);

        let result = part2(&parsed);

        assert_eq!(result, 5);
    }

    const SAMPLE: &str = "\
199
200
208
210
200
207
240
269
260
263
";
}
