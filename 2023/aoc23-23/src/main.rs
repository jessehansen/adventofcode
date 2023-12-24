use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt,
    str::FromStr,
};

use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    Problem::go()
}

#[derive(Clone, Copy, Default, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
enum Space {
    #[default]
    Empty,
    Forest,
    SlopeLeft,
    SlopeRight,
    SlopeUp,
    SlopeDown,
}

use Space::*;

impl FromStr for Space {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        match contents {
            "." => Ok(Empty),
            "#" => Ok(Forest),
            "<" => Ok(SlopeLeft),
            ">" => Ok(SlopeRight),
            "^" => Ok(SlopeUp),
            "v" => Ok(SlopeDown),
            _ => bail!("invalid space"),
        }
    }
}

impl fmt::Display for Space {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Empty => ".",
                Forest => "#",
                SlopeLeft => "<",
                SlopeRight => ">",
                SlopeUp => "^",
                SlopeDown => "v",
            }
        )
    }
}

impl Space {
    fn next_directions(&self, slippery: bool) -> Vec<Direction> {
        if !slippery {
            return vec![Up, Left, Down, Right];
        }
        use Direction::*;
        match self {
            Empty => vec![Up, Left, Down, Right],
            SlopeLeft => vec![Left],
            SlopeRight => vec![Right],
            SlopeUp => vec![Up],
            SlopeDown => vec![Down],
            Forest => vec![],
        }
    }
}

struct Problem {
    map: Grid2D<Space>,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        Ok(Self {
            map: Grid2D::from_char_str(contents)?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct LongWalkState {
    location: Point2D,
    visited: HashSet<Point2D>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct LongWalkCacheKey(Point2D, Vec<Point2D>);

impl OptimizationState for LongWalkState {
    type CacheKey = LongWalkCacheKey;
    type Score = usize;

    fn cache_key(&self) -> Self::CacheKey {
        LongWalkCacheKey(self.location, self.visited.iter().copied().collect())
    }

    fn score(&self) -> Self::Score {
        self.visited.len()
    }
}

impl Problem {
    fn next_steps(&self, pt: &Point2D, visited: &[Point2D], slippery: bool) -> Vec<Point2D> {
        self.map[*pt]
            .next_directions(slippery)
            .into_iter()
            .filter_map(|dir| {
                let maybe_next = pt.mv(dir, self.map.bounds);

                if maybe_next
                    .is_some_and(|next| self.map[next] != Forest && !visited.contains(&next))
                {
                    maybe_next
                } else {
                    None
                }
            })
            .collect()
    }

    fn find_paths(&self, start: Point2D, end: Point2D) -> Vec<Vec<Point2D>> {
        let mut paths = vec![vec![start]];
        let mut completed_paths = vec![];

        while let Some(path) = paths.pop() {
            let location = path.last().unwrap();
            for path_forward in
                self.next_steps(location, &path, true)
                    .into_iter()
                    .map(|next_step| {
                        let mut path_forward = path.clone();
                        path_forward.push(next_step);
                        path_forward
                    })
            {
                if path_forward.last() == Some(&end) {
                    completed_paths.push(path_forward);
                } else {
                    paths.push(path_forward);
                }
            }
        }

        completed_paths
    }

    // the input map is actually a series of long paths with forks. Build a map of each decision
    // point to the next paths, and the distance of each
    fn find_forks(
        &self,
        start: Point2D,
        end: Point2D,
    ) -> HashMap<Point2D, HashSet<(Point2D, usize)>> {
        let mut points: VecDeque<(Point2D, Point2D)> = VecDeque::new();
        let mut connected: HashSet<(Point2D, (Point2D, usize))> = HashSet::new();

        for step in self.next_steps(&start, &[], false) {
            points.push_back((start, step));
        }

        'outer: while let Some((sub_path_start, mut next_fork)) = points.pop_front() {
            let mut visited = vec![];
            visited.push(sub_path_start);

            let mut steps = 0;
            while self
                .map
                .cardinal_neighbors(next_fork)
                .filter(|(_pt, &space)| space != Forest)
                .count()
                < 3
            {
                steps += 1;
                visited.push(next_fork);
                if let Some(new) = self.next_steps(&next_fork, &visited, false).first() {
                    next_fork = *new;
                    visited.push(next_fork);
                    if next_fork == end {
                        connected.insert((sub_path_start, (end, steps + 1)));
                        break 'outer;
                    }
                } else {
                    continue 'outer;
                }
            }

            visited.push(next_fork);
            connected.insert((sub_path_start, (next_fork, steps + 1)));

            for next_path_start in self.next_steps(&next_fork, &visited, false) {
                points.push_back((next_fork, next_path_start));
            }
        }

        let mut shortened_graph: HashMap<Point2D, HashSet<(Point2D, usize)>> = HashMap::new();

        for (src, (dest, steps)) in connected {
            shortened_graph
                .entry(src)
                .or_default()
                .insert((dest, steps));
            shortened_graph
                .entry(dest)
                .or_default()
                .insert((src, steps));
        }
        shortened_graph
    }

    fn longest_path_from(
        &self,
        graph: &HashMap<Point2D, HashSet<(Point2D, usize)>>,
        location: Point2D,
        path: Vec<Point2D>,
        steps: usize,
    ) -> usize {
        let target = pt(self.map.bounds.width - 2, self.map.bounds.height - 1);
        if location == target {
            return steps;
        }
        if path.contains(&location) {
            return 0;
        }
        let mut next_path = path.clone();
        next_path.push(location);

        graph[&location]
            .iter()
            .map(|(point, additional_steps)| {
                if !next_path.contains(point) {
                    self.longest_path_from(
                        graph,
                        *point,
                        next_path.clone(),
                        steps + additional_steps,
                    )
                } else {
                    0
                }
            })
            .max()
            .unwrap()
    }
}

impl Solution for Problem {
    type Part1 = usize;
    type Part2 = usize;

    fn part1(&mut self) -> Result<Self::Part1> {
        let start = self
            .map
            .row(0)
            .find_map(|(pt, &space)| if space == Empty { Some(pt) } else { None })
            .ok_or_invalid()?;
        let end = self
            .map
            .row(self.map.bounds.height - 1)
            .find_map(|(pt, &space)| if space == Empty { Some(pt) } else { None })
            .ok_or_invalid()?;
        Ok(self
            .find_paths(start, end)
            .into_iter()
            .map(|path| path.len() - 1)
            .max()
            .ok_or_invalid()?)
    }

    fn part2(&self) -> Result<Self::Part2> {
        let start = self
            .map
            .row(0)
            .find_map(|(pt, &space)| if space == Empty { Some(pt) } else { None })
            .ok_or_invalid()?;
        let end = self
            .map
            .row(self.map.bounds.height - 1)
            .find_map(|(pt, &space)| if space == Empty { Some(pt) } else { None })
            .ok_or_invalid()?;

        let graph = self.find_forks(start, end);
        Ok(self.longest_path_from(&graph, start, vec![], 0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        let mut problem = Problem::from_str(SAMPLE)?;

        let result = problem.part1()?;

        assert_eq!(94, result);

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let problem = Problem::from_str(SAMPLE)?;

        let result = problem.part2()?;

        assert_eq!(154, result);

        Ok(())
    }

    const SAMPLE: &str = "\
#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";
}
