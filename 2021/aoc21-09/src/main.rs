use aoc_common::run;
use aoc_common::{Grid2D, Point2D};

fn main() {
    run(parse, part1, part2);
}

fn parse(contents: &str) -> Grid2D<u32> {
    contents
        .lines()
        .into_iter()
        .map(|x| {
            x.chars().into_iter().map(|x| {
                let result: u32 = x.to_string().parse().unwrap();
                result
            })
        })
        .collect()
}

fn low_points(grid: &Grid2D<u32>) -> impl Iterator<Item = (Point2D, &u32)> {
    let bounds = grid.bounds;
    grid.iter_horizontal()
        .filter(move |(pt, height)| !pt.neighbors(bounds).any(|pt| grid[pt] <= **height))
}

fn part1(grid: &Grid2D<u32>) -> String {
    let risk = low_points(grid).fold(0, |acc, (_, height)| acc + *height + 1);

    format!("{}", risk)
}

fn calculate_basin_size(grid: &Grid2D<u32>, low_point: Point2D) -> usize {
    let mut basin = vec![low_point];
    let mut last_size = 0;
    let bounds = grid.bounds;

    while basin.len() > last_size {
        last_size = basin.len();
        let basin_copy = basin.clone();
        for pt in basin_copy {
            let height = grid[pt];
            let mut flood: Vec<Point2D> = pt
                .neighbors(bounds)
                .filter(|pt| !basin.contains(pt) && grid[*pt] < 9 && grid[*pt] >= height)
                .collect();
            basin.append(&mut flood);
        }
    }

    basin.len()
}

fn part2(grid: &Grid2D<u32>) -> String {
    let low_points: Vec<Point2D> = low_points(grid).map(|(pt, _)| pt).collect();

    let mut basin_sizes = vec![];

    for pt in low_points {
        basin_sizes.push(calculate_basin_size(grid, pt));
    }

    basin_sizes.sort();

    let result = basin_sizes.iter().rev().take(3).fold(1, |acc, x| acc * x);

    format!("{}", result)
}
