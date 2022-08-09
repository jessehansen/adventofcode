use anyhow::*;
use aoc_common::run;
use aoc_common::{Grid2D, Point2D};
use std::collections::HashSet;

fn main() -> Result<()> {
    run(Grid2D::<u32>::from_char_str, part1, part2)
}

fn low_points(grid: &Grid2D<u32>) -> impl Iterator<Item = (Point2D, &u32)> {
    grid.iter_horizontal().filter(move |(pt, height)| {
        !grid
            .cardinal_neighbors(*pt)
            .any(|(_, other_height)| other_height <= *height)
    })
}

fn part1(grid: &Grid2D<u32>) -> Result<u32> {
    Ok(low_points(grid).fold(0, |acc, (_, height)| acc + *height + 1))
}

fn calculate_basin_size(grid: &Grid2D<u32>, low_point: Point2D) -> usize {
    let mut basin = HashSet::from([low_point]);
    let mut last_size = 0;

    while basin.len() > last_size {
        last_size = basin.len();
        let basin_copy = basin.clone();
        for pt in basin_copy {
            let height = grid[pt];
            grid.cardinal_neighbors(pt).for_each(|(pt, other_height)| {
                if other_height < &9 && other_height >= &height {
                    basin.insert(pt);
                }
            });
        }
    }

    basin.len()
}

fn part2(grid: &Grid2D<u32>) -> Result<usize> {
    let low_points: Vec<Point2D> = low_points(grid).map(|(pt, _)| pt).collect();

    let mut basin_sizes = vec![];

    for pt in low_points {
        basin_sizes.push(calculate_basin_size(grid, pt));
    }

    basin_sizes.sort_unstable();

    Ok(basin_sizes.iter().rev().take(3).product())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        let parsed = Grid2D::<u32>::from_char_str(SAMPLE)?;

        let result = part1(&parsed)?;

        assert_eq!(result, 15);

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let parsed = Grid2D::<u32>::from_char_str(SAMPLE)?;

        let result = part2(&parsed)?;

        assert_eq!(result, 1134);

        Ok(())
    }

    const SAMPLE: &str = "\
2199943210
3987894921
9856789892
8767896789
9899965678
";
}
