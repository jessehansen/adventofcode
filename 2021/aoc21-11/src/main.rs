use aoc_common::run;
use aoc_common::{Bounds2D, Grid2D, Point2D};

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

fn step(grid: &mut Grid2D<u32>) -> usize {
    let bounds = grid.bounds;
    let mut flashes = 0;

    grid.transform(|(_, x)| x + 1);

    let mut flashing = true;
    while flashing {
        flashing = false;

        // loop over bounds instead of grid to prevent borrow problems
        bounds.iter_horizontal().for_each(|pt| {
            if grid[pt] > 9 && grid[pt] < 100 {
                flashing = true;
                flash(grid, bounds, pt);
                // don't flash this location again this step
                grid[pt] += 100;
            }
        });
    }

    grid.transform(|(_, x)| {
        if x > &9 {
            flashes += 1;
            0
        } else {
            *x
        }
    });

    flashes
}

fn flash(grid: &mut Grid2D<u32>, bounds: Bounds2D, pt: Point2D) {
    pt.neighbors(bounds).for_each(|pt| {
        grid[pt] += 1;
    });
}

fn part1(grid: &Grid2D<u32>) -> String {
    let mut grid = grid.clone();
    let mut flashes = 0;
    for _ in 0..100 {
        flashes += step(&mut grid);
    }

    format!("{}", flashes)
}

fn part2(contents: &Grid2D<u32>) -> String {
    let mut grid = contents.clone();
    let mut steps = 1;
    loop {
        step(&mut grid);
        if grid.iter_horizontal().all(|(_, x)| x == &0) {
            break;
        }
        steps += 1;
    }

    format!("{}", steps)
}
