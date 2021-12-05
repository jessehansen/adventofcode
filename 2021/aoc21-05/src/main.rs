use std::fs;
use std::str::FromStr;
use std::fmt;
use std::collections::HashMap;

fn main() {
    let sample = fs::read_to_string("./sample.txt")
        .expect("Something went wrong reading the file");
    let input = fs::read_to_string("./input.txt")
        .expect("Something went wrong reading the file");

    part1(&sample, "sample");
    part1(&input, "input");

    part2(&sample, "sample");
    part2(&input, "input");
}

#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Point {
    type Err = ();

    fn from_str(input: &str) -> Result<Point, Self::Err> {
        let parts: Vec<i32> = input.split(",").map(|x| x.parse().unwrap()).collect();

        if parts.len() != 2 {
            return Err(());
        }

        Ok(Point{x: parts[0], y: parts[1]})
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Copy, Clone)]
struct Line {
    a: Point,
    b: Point,
}

impl FromStr for Line {
    type Err = ();

    fn from_str(input: &str) -> Result<Line, Self::Err> {
        let parts: Vec<&str> = input.split_whitespace().collect();

        if parts.len() != 3 {
            return Err(());
        }

        Ok(Line{
            a: Point::from_str(parts[0])?,
            b: Point::from_str(parts[2])?,
        })
    }
}

struct LineIter {
    current: Point,
    end: Point,
    delta_x: i32,
    delta_y: i32,
    done: bool,
}

impl Line {
    fn iter(&self) -> LineIter{
        let delta_x = if self.a.x != self.b.x { if self.a.x > self.b.x { -1 } else { 1 } } else { 0 };
        let delta_y = if self.a.y != self.b.y { if self.a.y > self.b.y { -1 } else { 1 } } else { 0 };
        LineIter{
            current: self.a,
            end: self.b,
            delta_x,
            delta_y,
            done: false,
        }
    }
}

impl Iterator for LineIter {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None
        }
        let result = self.current;
        if result.x == self.end.x && result.y == self.end.y {
            self.done = true;
        } else {
            self.current = Point{x:self.current.x + self.delta_x, y: self.current.y + self.delta_y};
        }
        Some(result)
    }
}

fn parse(contents:&str) -> Vec<Line> {
    return contents.lines().into_iter().map(|x| x.parse().expect("invalid input")).collect();
}

fn part1(contents:&str, description: &str){
    let lines = parse(contents);
    let mut grid = HashMap::new();
    for line in lines {
        if line.a.x != line.b.x && line.a.y != line.b.y {
            continue;
        }
        for point in line.iter() {
            let magnitude = grid.entry(format!("{}", point)).or_insert(0);
            *magnitude += 1;
        }
    }

    let hot_spots = grid.into_values().filter(|x| *x > 1).count();

    println!("Answer Part 1 ({}) = {}", description, hot_spots);
}

fn part2(contents:&str, description: &str){
    let lines = parse(contents);
    let mut grid = HashMap::new();
    for line in lines {
        for point in line.iter() {
            let magnitude = grid.entry(format!("{}", point)).or_insert(0);
            *magnitude += 1;
        }
    }

    let hot_spots = grid.into_values().filter(|x| *x > 1).count();

    println!("Answer Part 2 ({}) = {}", description, hot_spots);
}
