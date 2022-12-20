use std::cmp::{Eq, Ord, PartialEq};
use std::fmt;

use crate::Direction;

// contains helpers for graphs and signed points
// coordinates are laid out like this:Copy
//
//
//      |
//      |
//      |
//      |
// -----+----- y=0
//      |
//    * |
//      |
//      x=0
//
//  The * is at (x=-1, y = -2)

#[derive(Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct IPoint2D {
    pub x: i32,
    pub y: i32,
}

impl IPoint2D {
    pub const ORIGIN: IPoint2D = IPoint2D { x: 0, y: 0 };

    pub fn cardinal_distance(&self, other: &IPoint2D) -> (i32, i32) {
        (self.x - other.x, self.y - other.y)
    }

    pub fn up(&self) -> IPoint2D {
        IPoint2D {
            x: self.x,
            y: self.y + 1,
        }
    }

    pub fn down(&self) -> IPoint2D {
        IPoint2D {
            x: self.x,
            y: self.y - 1,
        }
    }

    pub fn left(&self) -> IPoint2D {
        IPoint2D {
            x: self.x - 1,
            y: self.y,
        }
    }

    pub fn right(&self) -> IPoint2D {
        IPoint2D {
            x: self.x + 1,
            y: self.y,
        }
    }

    pub fn mv(&self, direction: Direction) -> IPoint2D {
        match direction {
            Direction::Up => self.up(),
            Direction::Left => self.left(),
            Direction::Right => self.right(),
            Direction::Down => self.down(),
        }
    }

    pub fn move_by(&self, dx: i32, dy: i32) -> IPoint2D {
        IPoint2D {
            x: self.x + dx,
            y: self.y + dy,
        }
    }

    pub fn manhattan_distance(&self, other: IPoint2D) -> i32 {
        (other.x - self.x).abs() + (other.y - self.y).abs()
    }

    pub fn points_within_manhattan_distance(
        &self,
        distance: i32,
    ) -> impl Iterator<Item = IPoint2D> {
        let mut points = vec![];
        for dx in -distance..=distance {
            let y_allowed = distance - dx.abs();
            for dy in -y_allowed..=y_allowed {
                points.push(IPoint2D {
                    x: self.x + dx,
                    y: self.y + dy,
                });
            }
        }
        points.into_iter()
    }
}

impl fmt::Display for IPoint2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y,)
    }
}

impl fmt::Debug for IPoint2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y,)
    }
}

#[inline]
pub fn ipt(x: i32, y: i32) -> IPoint2D {
    IPoint2D { x, y }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_points_within_manhattan_distance() {
        let point = ipt(2, 3);
        let points: Vec<IPoint2D> = point.points_within_manhattan_distance(3).collect();
        assert_eq!(
            points,
            vec![
                ipt(-1, 3),
                ipt(0, 2),
                ipt(0, 3),
                ipt(0, 4),
                ipt(1, 1),
                ipt(1, 2),
                ipt(1, 3),
                ipt(1, 4),
                ipt(1, 5),
                ipt(2, 0),
                ipt(2, 1),
                ipt(2, 2),
                ipt(2, 3),
                ipt(2, 4),
                ipt(2, 5),
                ipt(2, 6),
                ipt(3, 1),
                ipt(3, 2),
                ipt(3, 3),
                ipt(3, 4),
                ipt(3, 5),
                ipt(4, 2),
                ipt(4, 3),
                ipt(4, 4),
                ipt(5, 3),
            ]
        );

        for pt in points {
            assert!(pt.manhattan_distance(point) <= 3);
        }
    }
}
