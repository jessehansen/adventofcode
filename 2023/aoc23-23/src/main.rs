use std::{
    collections::{HashSet, VecDeque},
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
    start: Point2D,
    end: Point2D,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        let map = Grid2D::from_char_str(contents)?;
        let start = map
            .row(0)
            .find_map(|(pt, &space)| if space == Empty { Some(pt) } else { None })
            .ok_or_invalid()?;
        let end = map
            .row(map.bounds.height - 1)
            .find_map(|(pt, &space)| if space == Empty { Some(pt) } else { None })
            .ok_or_invalid()?;
        Ok(Self { map, start, end })
    }
}

impl Problem {
    fn next_steps(&self, pt: &Point2D, visited: &[bool], slippery: bool) -> Vec<Point2D> {
        self.map[*pt]
            .next_directions(slippery)
            .into_iter()
            .filter_map(|dir| {
                let maybe_next = pt.mv(dir, self.map.bounds);

                if maybe_next.is_some_and(|next| {
                    self.map[next] != Forest && !visited[next.index(self.map.bounds.width)]
                }) {
                    maybe_next
                } else {
                    None
                }
            })
            .collect()
    }

    fn find_longest_path(&self, start: Point2D, end: Point2D) -> usize {
        let mut paths = vec![(start, vec![false; self.map.bounds.len()])];
        let mut result = 0;

        while let Some((location, visited)) = paths.pop() {
            for next_step in self.next_steps(&location, &visited, true) {
                let mut visited = visited.clone();
                visited[next_step.index(self.map.bounds.width)] = true;
                if next_step == end {
                    result = result.max(visited.iter().filter(|x| **x).count());
                } else {
                    paths.push((next_step, visited));
                }
            }
        }

        result
    }

    // the input map is actually a series of long paths with forks. Build a map of each decision
    // point to the next paths, and the distance of each
    fn find_forks(&self, start: Point2D, end: Point2D) -> Graph<Point2D, usize> {
        let mut points: VecDeque<(Point2D, Point2D)> = VecDeque::new();
        let mut connected: HashSet<(Point2D, (Point2D, usize))> = HashSet::new();

        for step in self.next_steps(&start, &vec![false; self.map.bounds.len()], false) {
            points.push_back((start, step));
        }

        'outer: while let Some((sub_path_start, mut next_fork)) = points.pop_front() {
            let mut visited = vec![false; self.map.bounds.len()];
            visited[sub_path_start.index(self.map.bounds.width)] = true;

            let mut steps = 0;
            while self
                .map
                .cardinal_neighbors(next_fork)
                .filter(|(_pt, &space)| space != Forest)
                .count()
                < 3
            {
                steps += 1;
                visited[next_fork.index(self.map.bounds.width)] = true;
                if let Some(new) = self.next_steps(&next_fork, &visited, false).first() {
                    next_fork = *new;
                    visited[next_fork.index(self.map.bounds.width)] = true;
                    if next_fork == end {
                        connected.insert((sub_path_start, (end, steps + 1)));
                        break 'outer;
                    }
                } else {
                    continue 'outer;
                }
            }

            visited[next_fork.index(self.map.bounds.width)] = true;
            connected.insert((sub_path_start, (next_fork, steps + 1)));

            for next_path_start in self.next_steps(&next_fork, &visited, false) {
                points.push_back((next_fork, next_path_start));
            }
        }

        let mut shortened_graph = Graph::default();

        for (src, (dest, steps)) in connected {
            let src = shortened_graph.insert_unique_node(src);
            let dest = shortened_graph.insert_unique_node(dest);
            shortened_graph.connect(src, dest, steps);
        }
        shortened_graph
    }
}

impl Solution for Problem {
    type Part1 = usize;
    type Part2 = usize;

    fn part1(&mut self) -> Result<Self::Part1> {
        Ok(self.find_longest_path(self.start, self.end))
    }

    fn part2(&self) -> Result<Self::Part2> {
        let graph = self.find_forks(self.start, self.end);
        graph
            .longest_path(
                graph.find_node_id(&self.start).unwrap(),
                graph.find_node_id(&self.end).unwrap(),
            )
            .ok_or_invalid()
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
