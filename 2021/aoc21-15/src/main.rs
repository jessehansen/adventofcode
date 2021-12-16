use aoc_common::run;
use aoc_common::{Bounds2D, Grid2D, Point2D};

fn main() {
    run(Grid2D::<u32>::from_char_str, part1, part2);
}

fn part1(cave: &Grid2D<u32>) -> u32 {
    cave.shortest_path()
}

fn part2(cave_tile: &Grid2D<u32>) -> u32 {
    let mut cave = Grid2D::new_constant(
        Bounds2D {
            width: cave_tile.bounds.width * 5,
            height: cave_tile.bounds.height * 5,
        },
        0,
    );

    //copy tile to first "row" of cave
    for i in 0..5 {
        for j in 0..5 {
            cave_tile
                .iter_horizontal()
                .for_each(|(Point2D { x, y }, risk)| {
                    let target = Point2D {
                        x: x + (i * cave_tile.bounds.width),
                        y: y + (j * cave_tile.bounds.height),
                    };
                    let target_risk = *risk + i as u32 + j as u32;
                    cave[target] = (target_risk - 1) % 9 + 1
                });
        }
    }

    cave.shortest_path()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() {
        let parsed = Grid2D::<u32>::from_char_str(SAMPLE);

        let result = part1(&parsed);

        assert_eq!(result, 40);
    }

    #[test]
    fn sample_part2() {
        let parsed = Grid2D::<u32>::from_char_str(SAMPLE);

        let result = part2(&parsed);

        assert_eq!(result, 315);
    }

    const SAMPLE: &str = "\
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
";
}
