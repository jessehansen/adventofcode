use anyhow::*;
use aoc_common::*;
use std::fmt;
use std::str::FromStr;

fn main() -> Result<()> {
    run(Grid2D::from_char_str, part1, part2)
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
enum SeaFloor {
    #[default]
    Empty,
    SeaCucumberEast,
    SeaCucumberSouth,
}
use SeaFloor::*;

#[derive(thiserror::Error, Debug)]
pub enum InvalidSeaFloorError {
    #[error("invalid sea floor character {0}")]
    InvalidSeaFloorCharacter(String),
}

use InvalidSeaFloorError::*;

impl FromStr for SeaFloor {
    type Err = InvalidSeaFloorError;

    fn from_str(input: &str) -> std::result::Result<SeaFloor, Self::Err> {
        match input {
            ">" => std::result::Result::Ok(SeaCucumberEast),
            "v" => std::result::Result::Ok(SeaCucumberSouth),
            "." => std::result::Result::Ok(Empty),

            c => std::result::Result::Err(InvalidSeaFloorCharacter(c.to_string())),
        }
    }
}

impl fmt::Display for SeaFloor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Empty => ".",
                SeaCucumberEast => ">",
                SeaCucumberSouth => "v",
            }
        )
    }
}

fn wrap_right(start: Point2D, bounds: Bounds2D) -> Point2D {
    match start.right(bounds.width) {
        Some(loc) => loc,
        None => pt(0, start.y),
    }
}

fn wrap_down(start: Point2D, bounds: Bounds2D) -> Point2D {
    match start.down(bounds.height) {
        Some(loc) => loc,
        None => pt(start.x, 0),
    }
}

fn get_moves<F>(floor: &Grid2D<SeaFloor>, herd: SeaFloor, movement: F) -> Vec<(Point2D, Point2D)>
where
    F: Fn(Point2D, Bounds2D) -> Point2D,
{
    floor
        .iter_horizontal()
        .filter_map(|(pt, space)| {
            if *space != herd {
                None
            } else {
                let target = movement(pt, floor.bounds);
                match floor[target] {
                    Empty => Some((pt, target)),
                    _ => None,
                }
            }
        })
        .collect()
}

fn do_moves(floor: &mut Grid2D<SeaFloor>, moves: &[(Point2D, Point2D)]) -> bool {
    let mut moved = false;
    for (start, target) in moves {
        do_move(floor, *start, *target);
        moved = true;
    }
    moved
}

fn do_move(floor: &mut Grid2D<SeaFloor>, start: Point2D, target: Point2D) {
    floor[target] = floor[start];
    floor[start] = Empty;
}

fn part1(floor: &Grid2D<SeaFloor>) -> Result<usize> {
    let mut floor = floor.clone();
    let mut step = 0;
    loop {
        step += 1;
        let mut moved = false;

        let east_moves = get_moves(&floor, SeaCucumberEast, wrap_right);
        moved |= do_moves(&mut floor, &east_moves);

        let south_moves = get_moves(&floor, SeaCucumberSouth, wrap_down);
        moved |= do_moves(&mut floor, &south_moves);

        if !moved {
            break;
        }
    }
    Ok(step)
}

fn part2(_: &Grid2D<SeaFloor>) -> Result<usize> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() -> Result<()> {
        let parsed = Grid2D::from_char_str(SAMPLE)?;

        let result = part1(&parsed)?;

        assert_eq!(result, 58);

        Ok(())
    }

    const SAMPLE: &str = "\
v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>
";
}
