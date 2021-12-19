use aoc_common::*;
use regex::Regex;
use std::cmp::Ordering;
use std::mem::swap;
use std::str::FromStr;

fn main() {
    run(parse, part1, part2);
}

#[derive(Debug, Copy, Clone)]
struct TargetArea {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

impl FromStr for TargetArea {
    type Err = ();

    fn from_str(line: &str) -> Result<TargetArea, Self::Err> {
        let re = Regex::new(
            r"x=(?P<x_min>-?\d+)\.\.(?P<x_max>-?\d+), y=(?P<y_min>-?\d+)\.\.(?P<y_max>-?\d+)",
        )
        .unwrap();

        let caps = re.captures(line.trim()).unwrap();

        let mut x_min: i32 = caps.name("x_min").unwrap().as_str().parse().unwrap();
        let mut x_max: i32 = caps.name("x_max").unwrap().as_str().parse().unwrap();
        let mut y_min: i32 = caps.name("y_min").unwrap().as_str().parse().unwrap();
        let mut y_max: i32 = caps.name("y_max").unwrap().as_str().parse().unwrap();

        if x_max < x_min {
            swap(&mut x_min, &mut x_max);
        }

        if y_max < y_min {
            swap(&mut y_min, &mut y_max);
        }

        Ok(TargetArea {
            x_min,
            x_max,
            y_min,
            y_max,
        })
    }
}

fn parse(contents: &str) -> TargetArea {
    contents.parse().unwrap()
}

fn calc_max_y_when_hits(x_velocity: i32, y_velocity: i32, target: &TargetArea) -> Option<i32> {
    let mut x = 0;
    let mut y = 0;
    let mut dx = x_velocity;
    let mut dy = y_velocity;
    let mut max_y = 0;

    while x <= target.x_max && y >= target.y_min {
        x += dx;
        y += dy;
        dx = match dx.cmp(&0) {
            Ordering::Greater => dx - 1,
            Ordering::Equal => 0,
            Ordering::Less => dx + 1,
        };
        dy -= 1;
        if y > max_y {
            max_y = y;
        }

        if x >= target.x_min && x <= target.x_max && y >= target.y_min && y <= target.y_max {
            return Some(max_y);
        }
    }
    None
}

fn part1(target: &TargetArea) -> i32 {
    let mut result = 0;
    // any time initial x_velocity is greater than the max, we will miss the target
    for x_velocity in 1..=(target.x_max + 1) {
        for y_velocity in 1..1000 {
            if let Some(max_y) = calc_max_y_when_hits(x_velocity, y_velocity, target) {
                if max_y > result {
                    result = max_y;
                }
            }
        }
    }

    result
}

fn part2(target: &TargetArea) -> usize {
    let mut result = 0;
    // any time initial x_velocity is greater than the max, we will miss the target
    for x_velocity in 1..=(target.x_max + 1) {
        for y_velocity in -1000..1000 {
            if calc_max_y_when_hits(x_velocity, y_velocity, target).is_some() {
                result += 1;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() {
        let parsed = parse(SAMPLE);

        let result = part1(&parsed);

        assert_eq!(result, 45);
    }

    #[test]
    fn sample_part2() {
        let parsed = parse(SAMPLE);

        let result = part2(&parsed);

        assert_eq!(result, 112);
    }

    #[test]
    fn problem_initial_v() {
        let parsed = parse(SAMPLE);

        let result = calc_max_y_when_hits(6, 0, &parsed);

        assert_eq!(result, Some(0));
    }

    const SAMPLE: &str = "target area: x=20..30, y=-10..-5";
}
