use aoc_common::*;
use std::fmt;

fn main() {
    run(parse, part1, part2);
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Image {
    lines: Vec<String>,
    outliers: char,
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.lines.join("\n"))
    }
}

impl Image {
    fn process(&self, alg: &str) -> Image {
        let mut lines: Vec<String> = vec![];
        for y in -1..(self.lines.len() as i32 + 1) {
            let mut line = String::new();
            for x in -1..(self.lines[0].len() as i32 + 1) {
                line.push(alg.chars().nth(self.pixel_num(x, y)).unwrap());
            }
            lines.push(line);
        }

        let outliers = alg.chars().nth(self.pixel_num(-3000, -3000)).unwrap();

        Image { lines, outliers }
    }

    fn pixel_num(&self, x: i32, y: i32) -> usize {
        let mut result = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                result <<= 1;
                result |= self.pixel_at(x as i32 + dx, y as i32 + dy)
            }
        }
        result
    }

    fn pixel_at(&self, x: i32, y: i32) -> usize {
        if x < 0 || y < 0 || x >= self.lines[0].len() as i32 || y >= self.lines.len() as i32 {
            return if self.outliers == '#' { 1 } else { 0 };
        }
        match self.lines[y as usize].chars().nth(x as usize) {
            Some('#') => 1,
            Some('.') => 0,
            _ => panic!(),
        }
    }

    fn lit(&self) -> usize {
        if self.outliers == '#' {
            panic!("Infinite lit pixels");
        }

        self.lines
            .iter()
            .map(|x| x.chars().filter(|c| *c == '#').count())
            .sum()
    }
}

fn parse(contents: &str) -> (String, Image) {
    let mut parts = contents.split("\n\n");

    (
        parts.next().unwrap().to_string(),
        Image {
            lines: parts
                .next()
                .unwrap()
                .lines()
                .map(|x| x.to_string())
                .collect(),
            outliers: '.',
        },
    )
}

fn part1((alg, img): &(String, Image)) -> usize {
    let mut img = img.clone();
    for _ in 0..2 {
        img = img.process(alg);
    }
    img.lit()
}

fn part2((alg, img): &(String, Image)) -> usize {
    let mut img = img.clone();
    for _ in 0..50 {
        img = img.process(alg);
    }
    img.lit()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pixel_num() {
        let (_, img) = parse(SAMPLE);

        assert_eq!(img.pixel_num(2, 2), 34);
    }

    #[test]
    fn sample_part1() {
        let parsed = parse(SAMPLE);

        let result = part1(&parsed);

        assert_eq!(result, 35);
    }

    #[test]
    fn sample_part2() {
        let parsed = parse(SAMPLE);

        let result = part2(&parsed);

        assert_eq!(result, 3351);
    }

    const SAMPLE: &str = "\
..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###
";
}
