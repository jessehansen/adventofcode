use aoc_common::*;
use std::fmt;
use std::str::FromStr;

fn main() {
    run(Grid2D::from_char_str, part1, part2);
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum SeaFloor {
    Empty,
    SeaCucumberEast,
    SeaCucumberSouth,
}
use SeaFloor::*;

impl FromStr for SeaFloor {
    type Err = ();

    fn from_str(input: &str) -> Result<SeaFloor, Self::Err> {
        match input {
            ">" => Ok(SeaCucumberEast),
            "v" => Ok(SeaCucumberSouth),
            "." => Ok(Empty),

            _ => Err(()),
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

impl Default for SeaFloor {
    fn default() -> Self {
        Empty
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

fn part1(floor: &Grid2D<SeaFloor>) -> usize {
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
    step
}

fn part2(_: &Grid2D<SeaFloor>) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() {
        let parsed = Grid2D::from_char_str(SAMPLE);

        let result = part1(&parsed);

        assert_eq!(result, 58);
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
