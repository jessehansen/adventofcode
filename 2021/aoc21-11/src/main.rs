use aoc_common::run;
use aoc_common::Grid2D;

fn main() {
    run(Grid2D::<u32>::from_char_str, part1, part2);
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
                grid.transform_neighbors(pt, |(_, value)| value + 1);
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

fn part1(grid: &Grid2D<u32>) -> usize {
    let mut grid = grid.clone();
    let mut flashes = 0;
    for _ in 0..100 {
        flashes += step(&mut grid);
    }

    flashes
}

fn part2(contents: &Grid2D<u32>) -> usize {
    let mut grid = contents.clone();
    let mut steps = 1;
    loop {
        step(&mut grid);
        if grid.iter_horizontal().all(|(_, x)| x == &0) {
            break;
        }
        steps += 1;
    }

    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() {
        let parsed = Grid2D::<u32>::from_char_str(SAMPLE);

        let result = part1(&parsed);

        assert_eq!(result, 1656);
    }

    #[test]
    fn sample_part2() {
        let parsed = Grid2D::<u32>::from_char_str(SAMPLE);

        let result = part2(&parsed);

        assert_eq!(result, 195);
    }

    const SAMPLE: &str = "\
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
";
}
