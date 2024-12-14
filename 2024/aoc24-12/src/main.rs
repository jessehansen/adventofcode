use std::{collections::HashSet, str::FromStr};

use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    Problem::go()
}

struct Problem {
    map: Grid2D<char>,
    regions: Vec<Region>,
}

#[derive(Debug, PartialEq, Eq)]
struct Region {
    plant: char,
    points: Vec<Point2D>,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        Ok(Self {
            map: Grid2D::from_char_str(contents)?,
            regions: vec![],
        })
    }
}

fn flood_fill(map: &Grid2D<char>, point: Point2D, plant: char) -> Vec<Point2D> {
    let mut todo = vec![point];
    let mut points = HashSet::new();

    while let Some(pt) = todo.pop() {
        points.insert(pt);
        todo.append(
            &mut map
                .cardinal_neighbors(pt)
                .filter(|(neighbor, neighbor_plant)| {
                    neighbor_plant == &&plant && !points.contains(neighbor)
                })
                .map(|(neighbor, _)| neighbor)
                .collect(),
        );
    }

    points.into_iter().collect()
}

fn flood_fence(
    coords: &[(Point2D, Direction)],
    start: Point2D,
    dir: Direction,
) -> Vec<(Point2D, Direction)> {
    let mut todo = vec![(start, dir)];
    let mut fence = HashSet::new();

    while let Some((pt, dir)) = todo.pop() {
        use Direction::*;

        fence.insert((pt, dir));
        todo.append(
            &mut coords
                .iter()
                .filter(|(neighbor, neighbor_direction)| {
                    neighbor_direction == &dir
                        && !fence.contains(&(*neighbor, dir))
                        && pt.manhattan_distance(*neighbor) == 1
                        && match dir {
                            Up | Down => neighbor.y == pt.y,
                            Left | Right => neighbor.x == pt.x,
                        }
                })
                .copied()
                .collect(),
        );
    }

    fence.into_iter().collect()
}

impl Region {
    fn in_map(map: &Grid2D<char>) -> Vec<Region> {
        let mut in_region = HashSet::new();
        let mut regions = vec![];

        for (point, &plant) in map.iter_horizontal() {
            if in_region.contains(&point) {
                continue;
            }

            let r = Region {
                plant,
                points: flood_fill(map, point, plant),
            };

            for r_pt in &r.points {
                in_region.insert(*r_pt);
            }
            regions.push(r);
        }

        regions
    }

    fn area(&self) -> usize {
        self.points.len()
    }

    fn fence_coords(&self, map: &Grid2D<char>) -> Vec<(Point2D, Direction)> {
        self.points
            .iter()
            .flat_map(|pt| {
                CARDINAL_DIRECTIONS
                    .iter()
                    .filter(|dir| match map.cardinal_neighbor(*pt, **dir) {
                        None => true,
                        Some((_, p)) if p != &self.plant => true,
                        _ => false,
                    })
                    .map(|dir| (*pt, *dir))
            })
            .collect()
    }

    fn perimeter(&self, map: &Grid2D<char>) -> usize {
        self.fence_coords(map).len()
    }

    fn fence_price(&self, map: &Grid2D<char>) -> usize {
        self.area() * self.perimeter(map)
    }

    fn sides(&self, map: &Grid2D<char>) -> usize {
        let mut in_fence_side = HashSet::new();
        let mut fence_sides = vec![];
        let coords = self.fence_coords(map);

        for (point, dir) in coords.iter() {
            if in_fence_side.contains(&(*point, *dir)) {
                continue;
            }

            let side = flood_fence(&coords, *point, *dir);

            for side_pair in &side {
                in_fence_side.insert(*side_pair);
            }
            fence_sides.push(side);
        }

        fence_sides.len()
    }

    fn discount_fence_price(&self, map: &Grid2D<char>) -> usize {
        self.area() * self.sides(map)
    }
}

impl Solution for Problem {
    type Part1 = usize;
    type Part2 = usize;

    fn part1(&mut self) -> Result<Self::Part1> {
        self.regions.append(&mut Region::in_map(&self.map));

        Ok(self.regions.iter().map(|r| r.fence_price(&self.map)).sum())
    }

    fn part2(&self) -> Result<Self::Part2> {
        Ok(self
            .regions
            .iter()
            .map(|r| r.discount_fence_price(&self.map))
            .sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_region() -> Result<()> {
        let map = Grid2D::from_char_str(SAMPLE_SMALL)?;

        let regions = Region::in_map(&map);

        assert_eq!(regions.len(), 5);
        assert_eq!(regions[0].plant, 'A');
        assert_eq!(regions[0].area(), 4);
        assert_eq!(regions[0].perimeter(&map), 10);
        assert_eq!(regions[0].sides(&map), 4);
        assert_eq!(regions[1].sides(&map), 4);
        assert_eq!(regions[2].sides(&map), 8);
        assert_eq!(regions[3].sides(&map), 4);
        assert_eq!(regions[4].sides(&map), 4);

        Ok(())
    }

    #[test]
    fn five_regions() -> Result<()> {
        let map = Grid2D::from_char_str(SAMPLE_FIVE_REGIONS)?;

        let regions = Region::in_map(&map);

        assert_eq!(regions.len(), 5);

        Ok(())
    }

    #[test]
    fn sample_part1() -> Result<()> {
        let mut problem = Problem::from_str(SAMPLE)?;

        let result = problem.part1()?;

        assert_eq!(1930, result);

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let mut problem = Problem::from_str(SAMPLE)?;
        problem.part1()?;

        let result = problem.part2()?;

        assert_eq!(1206, result);

        Ok(())
    }

    const SAMPLE_SMALL: &str = "\
AAAA
BBCD
BBCC
EEEC
";

    const SAMPLE_FIVE_REGIONS: &str = "\
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
";

    const SAMPLE: &str = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";
}
