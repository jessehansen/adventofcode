use std::{collections::VecDeque, str::FromStr};

use anyhow::*;
use aoc_common::*;

use fnv::{FnvHashMap, FnvHashSet};

fn main() -> Result<()> {
    Problem::go()
}

#[derive(Clone, Copy, Default, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
enum Space {
    #[default]
    Garden,
    Rock,
    Start,
}

use Space::*;

impl FromStr for Space {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        match contents {
            "." => Ok(Garden),
            "#" => Ok(Rock),
            "S" => Ok(Start),
            _ => bail!("invalid space"),
        }
    }
}

struct Problem {
    map: Grid2D<Space>,
    space_cache: FnvHashMap<IPoint2D, Vec<IPoint2D>>,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        let map: Grid2D<Space> = Grid2D::from_char_str(contents)?;
        let cache_capacity = map.bounds.width * map.bounds.height;
        Ok(Self {
            map,
            space_cache: FnvHashMap::with_capacity_and_hasher(cache_capacity, Default::default()),
        })
    }
}

const CENTER: Point2D = Point2D { x: 65, y: 65 };
const CORNERS: [Point2D; 4] = [
    Point2D { x: 0, y: 0 },
    Point2D { x: 130, y: 0 },
    Point2D { x: 0, y: 130 },
    Point2D { x: 130, y: 130 },
];

impl Problem {
    fn prime_space_cache(&mut self) {
        // for every location in the grid, figure out where next steps would lead
        let rocks: FnvHashSet<IPoint2D> = self
            .map
            .iter_horizontal()
            .filter_map(|(pt, space)| {
                if space == &Rock {
                    pt.try_into().ok()
                } else {
                    None
                }
            })
            .collect();

        for (pt, &space) in self.map.iter_horizontal() {
            if space == Rock {
                continue;
            }
            if let std::result::Result::Ok(pt) = IPoint2D::try_from(pt) {
                self.space_cache.insert(
                    pt,
                    pt.cardinal_neighbors()
                        .into_iter()
                        .filter(|pt| !rocks.contains(pt))
                        .collect(),
                );
            }
        }
    }

    fn count_reachable_in_steps(&self, steps: &[usize]) -> Vec<usize> {
        let mut current_locations: FnvHashSet<IPoint2D> = self
            .map
            .iter_horizontal()
            .filter_map(|(pt, &space)| match space {
                Start => IPoint2D::try_from(pt).ok(),
                _ => None,
            })
            .collect();

        let mut results = vec![];

        for step in 1..=(usize::MAX) {
            let next_locations = current_locations
                .iter()
                .flat_map(|pt| {
                    let (template_pt, copy_distance) =
                        pt.map_infinite_to_template_bounds(&self.map.bounds);
                    self.space_cache[&template_pt]
                        .iter()
                        .map(|pt| pt + copy_distance)
                        .collect::<Vec<_>>()
                })
                .collect::<FnvHashSet<_>>();

            current_locations = next_locations;

            if step == steps[results.len()] {
                results.push(current_locations.len());
                if results.len() == steps.len() {
                    return results;
                }
            }
        }
        vec![]
    }

    fn count_even_and_odd_reachable(
        &self,
        from: &[Point2D],
        limit: usize,
    ) -> (usize, usize, usize, usize) {
        let mut map = self.map.clone();
        let mut todo = VecDeque::new();

        let mut even_inner = 0;
        let mut even_outer = 0;
        let mut odd_inner = 0;
        let mut odd_outer = 0;

        for &start in from {
            map[start] = Rock;
            todo.push_back((start, 0));
        }

        while let Some((position, cost)) = todo.pop_front() {
            if cost % 2 == 1 {
                if position.manhattan_distance(CENTER) <= 65 {
                    odd_inner += 1;
                } else {
                    odd_outer += 1;
                }
            } else if cost <= 64 {
                even_inner += 1;
            } else {
                even_outer += 1;
            }

            if cost < limit {
                for next in position.cardinal_neighbors(map.bounds) {
                    if map[next] != Rock {
                        todo.push_back((next, cost + 1));
                        // don't count this location again
                        map[next] = Rock;
                    }
                }
            }
        }

        (even_inner, even_outer, odd_inner, odd_outer)
    }
}

impl Solution for Problem {
    type Part1 = usize;
    type Part2 = usize;

    fn part1(&mut self) -> Result<Self::Part1> {
        self.prime_space_cache();
        Ok(self.count_reachable_in_steps(&[64])[0])
    }

    fn part2(&self) -> Result<Self::Part2> {
        // geometric solution
        // determine the # of blocks reachable via even and odd number of steps
        // then, given x = number of steps
        //             a = # of blocks reachable on even steps in a single grid
        //             b = # of blocks reachable on odd steps in a single grid
        //             c = # of blocks reachable from the corners on even steps, within 65
        //             manhattan distanc
        //             d = # of blocks reachable from the middle on odd steps, within 65 manhattan
        //             distance
        // reachable = a*x^2 + b*(x+1)^2 + c*n - d*(n+1)

        let (even_inner, even_outer, odd_inner, odd_outer) =
            self.count_even_and_odd_reachable(&[CENTER], 130);
        let evens = even_inner + even_outer;
        let odds = odd_inner + odd_outer;
        let remove_corners = odd_outer;

        let (even_from_corners, ..) = self.count_even_and_odd_reachable(&CORNERS, 64);

        let x = 202300;
        let part_two = (x * x * evens) + ((x + 1) * (x + 1) * odds) + (x * even_from_corners)
            - ((x + 1) * remove_corners);
        Ok(part_two)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_six_steps() -> Result<()> {
        let problem = Problem::from_str(SAMPLE)?;

        assert_eq!(16, problem.count_reachable_in_steps(&[6])[0]);

        Ok(())
    }

    #[test]
    fn sample_more_steps() -> Result<()> {
        let problem = Problem::from_str(SAMPLE)?;

        // steps > 10 requires infinite grid step counting to work
        let counts = problem.count_reachable_in_steps(&[6, 10, 50, 100 /*, 500, 1000, 5000*/]);

        assert_eq!(16, counts[0]);
        assert_eq!(50, counts[1]);
        assert_eq!(1594, counts[2]);
        assert_eq!(6536, counts[3]);
        // assert_eq!(167004, counts[4]);
        // assert_eq!(668697, counts[5]);
        // assert_eq!(16733044, counts[6]);

        Ok(())
    }

    const SAMPLE: &str = "\
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
}
