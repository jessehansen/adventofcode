use std::{cmp::Ordering, str::FromStr};

use anyhow::*;
use aoc_common::*;

fn main() -> Result<()> {
    Problem::go()
}

struct Problem {
    map: Grid2D<usize>,
}

impl FromStr for Problem {
    type Err = Error;

    fn from_str(contents: &str) -> Result<Self> {
        Ok(Self {
            map: Grid2D::from_char_str(contents)?,
        })
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct LavaPathMove {
    direction: Direction,
    steps: usize,
}
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct LavaPathStep {
    heat_loss: usize,
    direction: Direction,
    steps: usize,
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct LavaPathState {
    heat_loss: usize,
    current_location: Point2D,
    last_move: Option<LavaPathMove>,
}

impl LavaPathState {
    fn new() -> LavaPathState {
        LavaPathState {
            heat_loss: 0,
            current_location: Point2D::ORIGIN,
            last_move: None,
        }
    }
}

impl Ord for LavaPathState {
    fn cmp(&self, other: &Self) -> Ordering {
        // comparing heat loss in reverse to minimize instead of maximize
        other
            .heat_loss
            .cmp(&self.heat_loss)
            .then_with(|| self.current_location.cmp(&other.current_location))
    }
}

impl PartialOrd for LavaPathState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Eq, PartialEq, Hash)]
struct LavaPathCacheKey {
    current_location: Point2D,
    last_move: Option<LavaPathMove>,
}

impl From<LavaPathState> for LavaPathCacheKey {
    fn from(value: LavaPathState) -> Self {
        LavaPathCacheKey {
            current_location: value.current_location,
            last_move: value.last_move,
        }
    }
}

impl Problem {
    fn next_moves(
        &self,
        state: &LavaPathState,
        min_steps: usize,
        max_steps: usize,
    ) -> Vec<LavaPathState> {
        use Direction::*;

        let pt = state.current_location;
        let heat_loss_in = state.heat_loss;
        let mut dirs = vec![Right, Down, Left, Up];
        if let Some(last_move) = state.last_move {
            // can't continue same direction or turn around in one step
            dirs.retain(|&dir| dir != last_move.direction && dir != last_move.direction.opposite());
        }

        dirs.into_iter()
            .flat_map(move |direction| {
                (min_steps..=max_steps).filter_map(move |steps| {
                    pt.move_by(direction, steps, self.map.bounds)
                        .map(move |location| {
                            let heat_loss = heat_loss_in
                            + pt.to(&location).map(|step| self.map[step]).sum::<usize>()
                            // pt.to includes start * end, so remove the start
                            - self.map[pt];
                            LavaPathState {
                                heat_loss,
                                current_location: location,
                                last_move: Some(LavaPathMove { direction, steps }),
                            }
                        })
                })
            })
            .collect::<Vec<_>>()
    }
}

impl Solution for Problem {
    type Part1 = usize;
    type Part2 = usize;

    fn part1(&mut self) -> Result<Self::Part1> {
        let solution = dijkstra::<LavaPathState, LavaPathCacheKey, _, _, _>(
            LavaPathState::new(),
            |state| self.next_moves(state, 1, 3),
            |state| state.current_location == self.map.bounds.bottom_right(),
        )
        .unwrap();

        Ok(solution.heat_loss)
    }

    fn part2(&self) -> Result<Self::Part2> {
        let solution = dijkstra::<LavaPathState, LavaPathCacheKey, _, _, _>(
            LavaPathState::new(),
            |state| self.next_moves(state, 4, 10),
            |state| state.current_location == self.map.bounds.bottom_right(),
        )
        .unwrap();

        Ok(solution.heat_loss)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        let mut problem = Problem::from_str(SAMPLE)?;

        let result = problem.part1()?;

        assert_eq!(102, result);

        Ok(())
    }

    #[test]
    fn sample_part2() -> Result<()> {
        let problem = Problem::from_str(SAMPLE)?;

        let result = problem.part2()?;

        assert_eq!(94, result);

        Ok(())
    }

    const SAMPLE: &str = "\
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
}
